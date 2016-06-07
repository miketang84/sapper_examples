use diesel::prelude::*;
use chrono::{DateTime, UTC};

#[derive(Queryable, Serialize, Debug)]
#[changeset_for(blogs)]
pub struct Blog {
    pub id: i64,
    pub title: String,
    pub content: String,
    // pub created_time: DateTime<UTC>,
}

use super::schema::blogs;

#[insertable_into(blogs)]
pub struct NewBlog<'a> {
    pub title: &'a str,
    pub content: &'a str,
    // pub created_time: DateTime<UTC>,
}

