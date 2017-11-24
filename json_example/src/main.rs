
extern crate sapper;
extern crate env_logger;
extern crate serde;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate sapper_std;
#[macro_use] extern crate serde_derive;

use sapper::{SapperApp, SapperAppShell, Request, Response, Result, SapperModule};


mod biz;
use biz::Biz;

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



pub fn main() {
    env_logger::init().unwrap();
    
    // fn init_global(req: &mut Request) -> Result<()> {
    //     Ok(())
    // }
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        // .init_global(Box::new(init_global))
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(Biz));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}

