use chrono::{DateTime, UTC};

use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;
use sapper_tmpl::Context;
use sapper::header::Location;
use sapper::status;

use sapper_body_params::ReqBodyParams;

#[derive(Clone)]
pub struct Blog;

use AppDB;

use sporm::query::{Query, Equality};


impl Blog {

    fn index(req: &mut Request) -> Result<Response> {
        
        res_html!("index.html", Context::new())
    }
    
    fn post_view(req: &mut Request) -> Result<Response> {
        
        let mut c = Context::new();
        
        
        res_html!("post.html", c)
    }
    
    fn posts_view(req: &mut Request) -> Result<Response> {
        
        let mut c = Context::new();
        
        
        res_html!("posts.html", c)
    }
    
    fn create_post_view(req: &mut Request) -> Result<Response> {
        
        res_html!("create_post.html", Context::new())
    }
    
    fn create_post(req: &mut Request) -> Result<Response> {
        
        let pool_wr = req.ext().get::<AppDB>();
        let db = match pool_wr {
            Some(pool) => {
                match pool.connect() {
                    Ok(conn) => conn,
                    Err(_) => {
                        return res_json_error!("get db error")
                    }
                }
            },
            None => return res_json_error!("get db error 2")
        };
        
        let body_params = req.ext().get::<ReqBodyParams>();
        let body_params = match body_params {
            Some(body_params) => {
                body_params
            },
            None => return res_json_error!("no body params error")
        };
        
        let title = &body_params.get("title").unwrap()[0];
        let content = &body_params.get("content").unwrap()[0];
        let now: DateTime<UTC> = UTC::now();
        
        // insert to db
        Query::insert()
            .into_table("blog")
            .set("title", title)
            .set("content", content)
            .set("created_time", &now)
            .execute(db.as_ref()).unwrap();

          
        // redirect
        let mut response = Response::new();
        response.set_status(status::Found);
        response.headers_mut().set(Location("/posts".to_owned()));
        // response.write_body("Redirect".to_string());
        
        Ok(response)
    }
    
    fn edit_post_view(req: &mut Request) -> Result<Response> {
        
        res_html!("edit_post.html", Context::new())
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
        // let a_global = req.ext().get::<A_INT>();
        // println!("in test, a_global is {:?}", a_global);
        // let a_hash = req.ext().get::<A_HashMap>();
        // println!("in test, a_hash is {:?}", a_hash);
        // let a_mutex = req.ext().get::<A_Mutex>();
        // println!("in test, a_mutex is {:?}", a_mutex);
        // {
        //     let mut a_mutex = a_mutex.unwrap();
        //     let mut data = a_mutex.lock().unwrap();
        //     data.insert("foo", "bar");
            
        // }
        // println!("in test, modified a_mutex is {:?}", a_mutex);
        
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

