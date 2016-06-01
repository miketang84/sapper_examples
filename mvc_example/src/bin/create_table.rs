extern crate sporm;
extern crate dotenv;

use sporm::pool::ManagedPool;
use sporm::database::Database;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    let db_url = env::var("DB_URL").expect("No postgres url variable DB_URL in .env config.");
    
    let pool = ManagedPool::init(&db_url, 1).unwrap();
    let db = pool.connect().unwrap();

    let sql = r"
CREATE TABLE blog (
    id bigserial NOT NULL,
    title character varying,
    content character varying,
    created_time timestamp with time zone NOT NULL,
    CONSTRAINT blog_pkey PRIMARY KEY (id)
);
";

    let result = db.as_ref().execute_sql_with_return(sql, &vec![]);

    println!("result: {:#?}", result);
}
