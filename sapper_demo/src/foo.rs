use sapper::{ SapperModule, SapperRouter, Response, Request, Result as SapperResult };
use sapper_std::{ QueryParams, PathParams, FormParams, JsonParams, Context, render };
use serde_json;

pub struct Foo;

impl Foo {
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut web = Context::new();
        web.add("data", &"Foo 模块");
        res_html!("index.html", web)
    }

    // 解析 `/query?query=1`
    fn query(req: &mut Request) -> SapperResult<Response> {
        let params = get_query_params!(req);
        let query = t_param_parse!(params, "query", i64);
        let mut web = Context::new();
        web.add("data", &query);
        res_html!("index.html", web)
    }

    // 解析 `/user/:id`
    fn get_user(req: &mut Request) -> SapperResult<Response> {
        let params = get_path_params!(req);
        let id = t_param!(params, "id").clone();

        println!("{}", id);

        let json2ret = json!({
            "id": id
        });

        res_json!(json2ret)
    }

    // 解析 body json 数据
    fn post_json(req: &mut Request) -> SapperResult<Response> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Person {
            foo: String,
            bar: String,
            num: i32,
        }

        let person: Person = get_json_params!(req);

        println!("{:#?}", person);

        let json2ret = json!({
            "status": true
        });
        res_json!(json2ret)
    }

    // 解析 form 数据
    fn test_post(req: &mut Request) -> SapperResult<Response> {
        let params = get_form_params!(req);
        let foo = t_param!(params, "foo");
        let bar = t_param!(params, "bar");
        let num = t_param_parse!(params, "num", i32);

        println!("{}, {}, {}", foo, bar, num);

        let json2ret = json!({
            "status": true
        });
        res_json!(json2ret)
    }
}

impl SapperModule for Foo {
    fn before(&self, _req: &mut Request) -> SapperResult<()> {
        Ok(())
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/foo", Foo::index);

        router.get("/query", Foo::query);

        router.get("/user/:id", Foo::get_user);

        router.post("/test_post", Foo::test_post);

        router.post("/post_json", Foo::post_json);

        Ok(())
    }
}
