use mongodb::bson::Document;
use mongodb::Client;
use poem::listener::TcpListener;
use poem::{EndpointExt, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    tracing_subscriber::fmt::init();

    let mongo_client = Client::with_uri_str("localhost:8080").await.expect("REASON").
        database("MyMongo");
    let docs: mongodb::Collection<Document> = mongo_client.collection("docs");
    struct Api;

    #[OpenApi]
    impl Api {
        #[oai(path = "/hello", method = "get")]
        async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
            match name.0 {
                Some(name) => PlainText(format!("hello, {name}!")),
                None => PlainText("hello!".to_string()),
            }
        }
    }

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let swagger_ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(
            Route::new()
                .nest("/api/push-registration", api_service)
                .nest("/swagger", swagger_ui)
                //.nest("/api-spec", open_api_spec)
                .data(docs),
        )
        .await
}