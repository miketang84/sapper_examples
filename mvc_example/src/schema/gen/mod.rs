pub mod public;

pub mod schema;
pub mod table;
pub mod column;

use sporm::table::Table;
use sporm::table::IsTable;
use gen::public::Blog;


pub fn get_all_tables() -> Vec<Table> {
    vec![
        Blog::table(),
    ]
}