extern crate dotenv;
extern crate sporm;
extern crate spcodegen;

use dotenv::dotenv;
use std::env;

use spcodegen::generator;
use spcodegen::generator::Config;
use sporm::pool::ManagedPool;
//use rustorm::database::DatabaseDev;

/// this will generate needed model code for tests in ./examples/gen directory
fn main(){
    dotenv().ok();
    
    let db_url = env::var("DB_URL").expect("No postgres url variable DB_URL in .env config.");
    
    let pool = ManagedPool::init(&db_url[..], 1).unwrap();
    let db = pool.connect().unwrap();
    let config = Config {
            base_module: Some("gen".to_string()),
            include_table_references: true,
            use_condensed_name: true,
            generate_table_meta: true,
            base_dir: "schema".to_string(),
            include_views: true,
        };
    generator::generate_all(db.as_dev(), &config);
    
    println!("Ok.");
}
