use uuid::Uuid;
use postgres::rows::Row;
use postgres::types::ToSql;

use pg::Insertable;

#[derive(Debug)]
pub struct ConversationUser {
    pub uuid: Uuid,
    pub conversation_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl ConversationUser {
    pub fn new(user_uuid: Uuid, conversation_uuid: Uuid) -> Self {
        ConversationUser {
            uuid: Uuid::new_v4(),
            conversation_uuid,
            user_uuid,
        }
    }
}

impl<'a> From<Row<'a>> for ConversationUser {
    fn from(row: Row) -> Self {
        ConversationUser {
            uuid: row.get("uuid"),
            conversation_uuid: row.get("conversation_uuid"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl Insertable for ConversationUser {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO conversations_users (uuid, user_uuid, conversation_uuid) VALUES ($1::uuid, $2::uuid, $3::uuid)
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.user_uuid, &self.conversation_uuid])
    }
}
