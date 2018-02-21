use std::time::Duration;

use dotenv::dotenv;
use embedded_migrations;

use config;
use reqwest;
use rocket;
use graphql::query::{Schema, Query};
use graphql::mutation::Mutation;
use pg::create_db_pool;
use mindstream::rss;
use routes;

use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;
use config::Config;
use diesel::Connection;
pub fn create_diesel_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let database_url = config.database_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn establish_connection(config: &Config) -> PgConnection {
    let database_url = config.database_url.clone();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn run() {
    dotenv().ok();
    let conf = config::Config::from_env();
    let connection = create_db_pool(&conf);
    let _diesel_pool = create_diesel_pool(&conf);
    let diesel_connection = establish_connection(&conf);
    embedded_migrations::run(&diesel_connection).expect("Migration Error");

    let client = reqwest::Client::new();
    rss::run_rss_job(Duration::from_secs(&conf.rss_job_interval * 60), client, connection.clone());
    rocket::ignite()
        .manage(Query::new(connection.clone()))
        .manage(create_db_pool(&conf))
        .manage(conf)
        .manage(Schema::new(
            Query::new(connection),
            Mutation,
        ))
        .mount("/", routes![
            routes::index,
            routes::files,
            routes::graphiql,
            routes::post_graphql_handler,
        ])
        .launch();
}
