#[macro_use] extern crate log;
extern crate sapper;
extern crate sapper_std;
extern crate sapper_demo;

use sapper::{ App, Armor, Request, Response, Result as SapperResult };
use sapper_demo::{ Foo, Bar, Global };
use std::sync::Arc;

struct WebApp;

impl Armor for WebApp {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
	sapper_std::init(req, Some("session"))?;
	Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
	sapper_std::finish(req, res)?;
	Ok(())
    }
}

fn main() {
    let global = Arc::new(String::from("global variable"));
    let mut app = App::new();
    app.address("127.0.0.1")
	.port(8080)
	.init_global(
	    Box::new(move |req: &mut Request| {
		req.ext_mut().insert::<Global>(global.clone());
		Ok(())
	    })
	)
	.with_armor(Box::new(WebApp))
	.add_module(Box::new(Foo))
	.add_module(Box::new(Bar))
	.static_file_service(true)
	.not_found_page(String::from("not found"));

    println!("Start listen on {}", "127.0.0.1:8080");
    app.run_http();
}
