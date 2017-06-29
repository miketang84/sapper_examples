use diesel::prelude::*;

use super::schema::blogs;
#[derive(Queryable, Serialize, Debug, Deserialize)]
pub struct Blog {
    pub id: i64,
    pub title: String,
    pub content: String,
    // pub created_time: DateTime<UTC>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="blogs"]
pub struct NewBlog<'a> {
    pub title: &'a str,
    pub content: &'a str,
    // pub created_time: DateTime<UTC>,
}

