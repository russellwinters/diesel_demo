use crate::schema::posts;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
