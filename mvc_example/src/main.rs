#![feature(question_mark, custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate typemap;
extern crate chrono;
extern crate dotenv;
extern crate sporm;
extern crate serde;
extern crate serde_json;

extern crate sapper;
extern crate sapper_request_basic_logger;
extern crate sapper_tmpl;
#[macro_use]
extern crate sapper_body_params;
extern crate sapper_query_params;
#[macro_use]
extern crate sapper_macros;

use std::sync::Arc;
use typemap::Key;
use dotenv::dotenv;
use std::env;

use sapper::{SApp, SAppWrapper, Request, Response, Result, SModule};
use sporm::pool::ManagedPool;


mod gen;
pub use gen::public::Blog as BlogModel;

mod blog;
use blog::Blog as BlogModule;


#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        sapper_request_basic_logger::init(req)?;
        sapper_query_params::process(req)?;
        sapper_body_params::process(req)?;

        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        sapper_request_basic_logger::log(req, res)?;

        Ok(())
    }
}

pub struct AppDB;
impl Key for AppDB { type Value = Arc<ManagedPool>; }


pub fn main() {
    env_logger::init().unwrap();
    dotenv().ok();
    
    let db_url = env::var("DB_URL").expect("No postgres url variable DB_URL in .env config.");
    let pool = Arc::new(ManagedPool::init(&db_url, 1).unwrap());
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .init_global(Box::new(move |req: &mut Request| -> Result<()> {
            req.ext_mut().insert::<AppDB>(pool.clone());
            
            Ok(())
        }))
        .with_wrapper(Box::new(MyApp))
        .add_module(Box::new(BlogModule));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}
