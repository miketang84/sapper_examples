use chrono::{DateTime, UTC};

use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;
use sapper_tmpl::Context;
use sapper::header::Location;
use sapper::status;

#[derive(Clone)]
pub struct Blog;

use AppDB;
use sporm::query::{Query, Equality};
use BlogModel;

impl Blog {

    fn index(req: &mut Request) -> Result<Response> {
        
        res_html!("index.html", Context::new())
    }
    
    fn post_view(req: &mut Request) -> Result<Response> {
        
        let params = get_path_params!(req);
        let postid = params.find("postid").unwrap_or("");
        
        let db = get_db!(req, AppDB);
        let post: BlogModel = Query::select()
            .from_table("blog")
            .filter("id", Equality::EQ, &postid.parse::<i64>().unwrap())
            .collect_one(db.as_ref()).unwrap();
        
        let mut c = Context::new();
        c.add("post", &post);
        
        res_html!("post.html", c)
    }
    
    fn posts_view(req: &mut Request) -> Result<Response> {
        
        let db = get_db!(req, AppDB);
        
        let posts: Vec<BlogModel> = Query::select()
            .from_table("blog")
            .collect(db.as_ref()).unwrap();
        println!("{:?}", posts);
        
        let mut c = Context::new();
        c.add("posts", &posts);
        
        res_html!("posts.html", c)
    }
    
    fn create_post_view(req: &mut Request) -> Result<Response> {
        
        res_html!("create_post.html", Context::new())
    }
    
    fn create_post(req: &mut Request) -> Result<Response> {
        
        let db = get_db!(req, AppDB);
        let params = get_body_params!(req);
        let title = check_param!(params, "title");
        let content = check_param!(params, "content");

        let now: DateTime<UTC> = UTC::now();
        
        // insert to db
        Query::insert()
            .into_table("blog")
            .set("title", title)
            .set("content", content)
            .set("created_time", &now)
            .execute(db.as_ref()).unwrap();

        res_redirect!("/posts")
    }
    
    fn edit_post_view(req: &mut Request) -> Result<Response> {
        let params = get_path_params!(req);
        let postid = params.find("postid").unwrap_or("");
        
        let db = get_db!(req, AppDB);
        let post: BlogModel = Query::select()
            .from_table("blog")
            .filter("id", Equality::EQ, &postid.parse::<i64>().unwrap())
            .collect_one(db.as_ref()).unwrap();
        
        let mut c = Context::new();
        c.add("post", &post);
        res_html!("edit_post.html", c)
    }
    
    fn edit_post(req: &mut Request) -> Result<Response> {
        
        let db = get_db!(req, AppDB);
        let params = get_body_params!(req);
        let postid = check_param!(params, "postid");
        let title = check_param!(params, "title");
        let content = check_param!(params, "content");

        let now: DateTime<UTC> = UTC::now();
        
        Query::update()
            .from_table("blog")
            .set("title", title)
            .set("content", content)
            .filter("id", Equality::EQ, &postid.parse::<i64>().unwrap())
            .execute(db.as_ref()).unwrap();

        res_redirect!(&("/post/".to_owned() + postid))
    }
    
    fn delete_post(req: &mut Request) -> Result<Response> {
        let db = get_db!(req, AppDB);
        let params = get_path_params!(req);
        let postid = params.find("postid").unwrap_or("");
        
        // insert to db
        Query::delete()
            .from_table("blog")
            .filter("id", Equality::EQ, &postid.parse::<i64>().unwrap())
            .execute(db.as_ref()).unwrap();
            
        res_redirect!("/posts")
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
        router.get("/post/:postid/edit", Blog::edit_post_view);
        router.post("/post/edit", Blog::edit_post);
        router.get("/post/:postid/delete", Blog::delete_post);
        
        Ok(())
        
    }
    
    
}

