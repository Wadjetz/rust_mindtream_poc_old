use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;

use errors::*;
use token;
use graphql::query::Query;
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
    //pub created: String,
    //pub updated: String,
    //pub last_connection: String,
    //pub active: bool,
}

impl User {
    pub fn new(uuid: Uuid, login: String, email: String, password: String) -> Self {
        User {
            uuid, login, email, password,
        }
    }

    pub fn new_secure(login: String, email: String, password: String) -> Result<User> {
        let hashed_password = hash_password(&password)?;
        let user = User {
            uuid: Uuid::new_v4(),
            login,
            email,
            password: hashed_password,
        };
        Ok(user)
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    Ok(verify(password, hashed_password)?)
}

graphql_object!(User: Query as "User" |&self| {
    description: "User"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field email() -> &String as "email" {
        &self.email
    }

    field login() -> &String as "login" {
        &self.login
    }
});

impl<'a> From<Row<'a>> for User {
    fn from(row: Row) -> Self {
        User {
            uuid: row.get("uuid"),
            login: row.get("login"),
            email: row.get("email"),
            password: row.get("password"),
        }
    }
}

impl Insertable for User {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO users (uuid, login, email, password)
            VALUES ($1, $2, $3, $4);
        "#.to_owned()
    }

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.login, &self.email, &self.password])
    }
}

pub fn signup_resolver<'a>(executor: &Executor<'a, Query>, login: String, email: String, password: String) -> Result<String> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let user = User::new_secure(login, email, password)?;
    pg.insert(&user)?;
    let token = token::create_token(user.uuid, user.email)?;
    Ok(token)
}

fn find_user_by_email(pg: &PgDatabase, email: &str) -> Result<Option<User>> {
    let query = r#"SELECT * FROM users WHERE email = $1;"#;
    Ok(pg.find_one::<User>(query, &[&email])?)
}

pub fn login_resolver<'a>(executor: &Executor<'a, Query>, email: String, password: String) -> Result<String> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    if let Some(user) = find_user_by_email(&pg, &email)? {
        if let Ok(true) = verify_password(&password, &user.password) {
            Ok(token::create_token(user.uuid, email)?)
        } else {
            Err(ErrorKind::WrongCredentials.into())
        }
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}