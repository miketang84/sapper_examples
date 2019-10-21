use chrono::prelude::*;
use sapper::{Result, Module, Request, Response, Router};
use sapper_std::{Context, render, PathParams, FormParams, QueryParams};

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Clone)]
pub struct BlogModule;

use AppDB;
use models::Blog as BlogModel;
use models::NewBlog;
use super::establish_connection;


// macro_rules! get_db {
//     ($req:expr, $dbkey:ty) => ({
//         let conn_wr = $req.ext().get::<$dbkey>();
//         let db = match conn_wr {
//             Some(conn) => {
//                 conn
//             },
//             None => return res_500!("no db defined.")
//         };

//         db
//     })
// }


macro_rules! get_db {
    ($req:expr, $dbkey:ty) => ({
	establish_connection()
    })
}



impl BlogModule {

    fn index(req: &mut Request) -> Result<Response> {

	res_html!("index.html", Context::new())
    }

    fn post_view(req: &mut Request) -> Result<Response> {
	use schema::blogs::dsl::*;

	let db = get_db!(req, AppDB);

	let params = get_path_params!(req);
	let postid = t_param!(params, "postid").parse::<i64>().unwrap();

	let post = blogs.find(postid)
		.first::<BlogModel>(&db)
		.expect("Error loading blog");

	let mut c = Context::new();
	c.add("post", &post);

	res_html!("post.html", c)
    }

    fn posts_view(req: &mut Request) -> Result<Response> {
	use schema::blogs::dsl::*;

	let db = get_db!(req, AppDB);
	let results = blogs.load::<BlogModel>(&db).expect("Error loading blogs");

	let posts = results;

	println!("{:?}", posts);

	let mut c = Context::new();
	c.add("posts", &posts);

	res_html!("posts.html", c)
    }

    fn create_post_view(req: &mut Request) -> Result<Response> {

	res_html!("create_post.html", Context::new())
    }

    fn create_post(req: &mut Request) -> Result<Response> {
	use schema::blogs;

	let db = get_db!(req, AppDB);
	let params = get_form_params!(req);
	let title = t_param!(params, "title");
	let content = t_param!(params, "content");

	//let now: DateTime<UTC> = UTC::now();

	let new_blog = NewBlog {
	    title: title,
	    content: content,
	    // created_time: now
	};

	diesel::insert(&new_blog)
	    .into(blogs::table)
	    .get_result::<BlogModel>(&db)
	    .expect("Error saving new blog");

	res_redirect!("/posts")
    }

    fn edit_post_view(req: &mut Request) -> Result<Response> {
	use schema::blogs::dsl::*;

	let params = get_path_params!(req);
	let postid = t_param!(params, "postid").parse::<i64>().unwrap();

	let db = get_db!(req, AppDB);
	let post = blogs.find(postid)
		.first::<BlogModel>(&db)
		.expect("Error loading blog");

	let mut c = Context::new();
	c.add("post", &post);
	res_html!("edit_post.html", c)
    }

    fn edit_post(req: &mut Request) -> Result<Response> {
	use schema::blogs::dsl::*;


	let db = get_db!(req, AppDB);
	let params = get_form_params!(req);
	let postid = t_param!(params, "postid").parse::<i64>().unwrap();
	let utitle = t_param!(params, "title");
	let ucontent = t_param!(params, "content");

	//let now: DateTime<UTC> = UTC::now();

	//let mut post: BlogModel = blogs.find(postid)
	//                .first::<BlogModel>(&db)
	//                .expect("Error finding blog");
	//post.title = utitle.clone();
	//post.content = ucontent.clone();
	//post.save_changes::<BlogModel>(&db).unwrap();


	let post = diesel::update(blogs.find(postid))
			 .set((title.eq(utitle), content.eq(ucontent)))
			 .get_result::<BlogModel>(&db)
			 .expect(&format!("Unable to find blog {}", postid));

	res_redirect!(&("/post/".to_owned() + &postid.to_string()))
    }

    fn delete_post(req: &mut Request) -> Result<Response> {
	use schema::blogs::dsl::*;

	let db = get_db!(req, AppDB);
	let params = get_path_params!(req);
	let postid = t_param!(params, "postid").parse::<i64>().unwrap();

	diesel::delete(blogs.find(postid))
	    .execute(&db)
	    .expect("Error deleting blog");

	res_redirect!("/posts")
    }

}

// set before, after middleware, and add routers
impl Module for BlogModule {

    fn before(&self, req: &mut Request) -> Result<()> {
	Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
	Ok(())
    }

    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {

	router.get("/", Self::index);
	router.get("/post/:postid", Self::post_view);
	router.get("/posts", Self::posts_view);
	router.get("/post/create", Self::create_post_view);
	router.post("/post/create", Self::create_post);
	router.get("/post/:postid/edit", Self::edit_post_view);
	router.post("/post/edit", Self::edit_post);
	router.get("/post/:postid/delete", Self::delete_post);

	Ok(())

    }


}
