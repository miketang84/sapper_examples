
use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;

use std::str;


use serde_json;
use serde_json::builder::ObjectBuilder;

use sapper::header::ContentType;

macro_rules! res_json {
    ($json:expr) => ({
        let mut response = Response::new();
        response.headers_mut().set(ContentType::json());
        response.write_body(serde_json::to_string(&$json).unwrap());

        Ok(response)
    })
}


macro_rules! res_json_ok {
    ($info:expr) => ({
        let json2ret = ObjectBuilder::new()
            .insert("success", true)
            .insert("info", $info)
            .unwrap();

        res_json!(json2ret)
    })
}

macro_rules! res_json_error {
    ($info:expr) => ({
        let json2ret = ObjectBuilder::new()
            .insert("success", false)
            .insert("info", $info)
            .unwrap();

        res_json!(json2ret)
    })
}


#[derive(Clone)]
pub struct Biz;

impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
             
        let json2ret = ObjectBuilder::new()
            .insert("foo", "bar")
            .insert("doit", 123)
            .unwrap();

        res_json!(json2ret)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        res_json_ok!("success.")    
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        res_json_error!("fail.")    
    }
    
}

// set before, after middleware, and add routers
impl SModule for Biz {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Biz before.");
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Biz after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

