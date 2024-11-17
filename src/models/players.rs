use crate::schema::players;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = players)]
pub struct Players {
    pub id: i32,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: String,
}
