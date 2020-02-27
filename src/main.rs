#[macro_use]
extern crate chrono;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tower_web;

use chrono::{DateTime, Utc};
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct JsonResource;

#[derive(Debug, Response)]
struct MyResponse {
    foo: usize,
    bar: &'static str,
}

#[derive(Debug, Response)]
#[web(status = "201")]
struct CreateResponse {
    message: &'static str,

    #[web(header)]
    date: String,
}

impl_web! {
    impl JsonResource {
        #[get("/")]
        fn hello_world(&self) -> Result<serde_json::Value, ()> {
            Ok(json!({
                "message": "hello world",
            }))
        }

        #[get("/custom_type")]
        #[content_type("application/json")]
        fn custom_type(&self) -> Result<MyResponse, ()> {
            Ok(MyResponse {
                foo: 123,
                bar: "hello world"
            })
        }

        #[post("/create")]
        #[content_type("application/json")]
        fn create(&self) -> Result<CreateResponse, ()> {
            Ok(CreateResponse {
                message: "created",
                date: get_rfc7231_date_time(),
            })
        }
    }
}

fn get_rfc7231_date_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    return now.to_rfc2822();
}

fn main() {
    let addr = "0.0.0.0:5000".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);
    ServiceBuilder::new()
        .resource(JsonResource)
        .run(&addr)
        .unwrap();
}
