
use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;

#[derive(Clone)]
pub struct Biz;

use A_INT;
use A_HashMap;
use A_Mutex;

impl Blog {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn post_view(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn posts_view(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn create_post_view(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn create_post(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn edit_post_view(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn edit_post(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn delete_post(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    
    fn test(req: &mut Request) -> Result<Response> {
        let a_global = req.ext().get::<A_INT>();
        println!("in test, a_global is {:?}", a_global);
        let a_hash = req.ext().get::<A_HashMap>();
        println!("in test, a_hash is {:?}", a_hash);
        let a_mutex = req.ext().get::<A_Mutex>();
        println!("in test, a_mutex is {:?}", a_mutex);
        {
            let mut a_mutex = a_mutex.unwrap();
            let mut data = a_mutex.lock().unwrap();
            data.insert("foo", "bar");
            
        }
        println!("in test, modified a_mutex is {:?}", a_mutex);
        
        let mut response = Response::new();
        response.write_body("hello, tang gang gang!".to_string());
        
        Ok(response)
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        
        println!("in test_post, raw_body: {:?}", req.raw_body());
        
        let mut response = Response::new();
        response.write_body("hello, I'am !".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SModule for Blog {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SRouter) -> Result<()> {
        
        router.get("/", Blog::index);
        router.get("/post/:postid", Blog::post_view);
        router.get("/posts", Blog::posts_view);
        router.get("/post/create", Blog::create_post_view);
        router.post("/post/create", Blog::create_post);
        router.get("/post/edit", Blog::edit_post_view);
        router.post("/post/edit", Blog::edit_post);
        router.post("/post/delete", Blog::delete_post);
        
        Ok(())
        
    }
    
    
}

