use sapper::{ SapperModule, SapperRouter, Response, Request, Result as SapperResult, Key, Error as SapperError };
use std::sync::Arc;
use sapper::header::ContentType;

pub struct Bar;

impl Bar {
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut res = Response::new();
        res.headers_mut().set(ContentType::html());
        res.write_body(String::from("bar"));
        Ok(res)
    }
}

impl SapperModule for Bar {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let global = req.ext().get::<Global>().unwrap().to_string();
        let res = json!({
            "error": global
        });
        Err(SapperError::CustomJson(res.to_string()))
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/bar", Bar::index);
        Ok(())
    }
}

pub struct Global;

impl Key for Global {
    type Value = Arc<String>;
}
