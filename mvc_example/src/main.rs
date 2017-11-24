extern crate env_logger;
extern crate dotenv;

extern crate chrono;
extern crate serde;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate sapper_std;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate sapper;

use std::env;
use std::sync::Arc;
use sapper::Key;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use sapper::{SapperApp, SapperAppShell, Request, Response, Result, SapperModule};

pub mod schema;
pub mod models;

mod blog;
use blog::BlogModule;


#[derive(Clone)]
struct MyApp;

impl SapperAppShell for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        sapper_std::init(req, None)?;

        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        sapper_std::finish(req, res)?;

        Ok(())
    }
}

pub struct AppDB;
impl Key for AppDB { type Value = Arc<PgConnection>; }



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn main() {
    env_logger::init().unwrap();
    dotenv().ok();
    
    // let conn = Arc::new(establish_connection());
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        // .init_global(Box::new(move |req: &mut Request| -> Result<()> {
        //     req.ext_mut().insert::<AppDB>(conn.clone());
            
        //     Ok(())
        // }))
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(BlogModule));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}
