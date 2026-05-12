use axum::{
    response::Html,
    Router,
    routing::{get, post},
    extract::Form,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/search", get(search));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(r#"
        <h1>Login</h1>

        <form action="/login" method="POST">
            <input name="username" placeholder="Username">
            <input name="password" type="password" placeholder="Password">

            <button type="submit">Login</button>
        </form>
    "#)
}

#[derive(serde::Deserialize)]
struct Input{
    username:String,
    password:String,
}
async fn login(Form(input):Form<Input>) -> String {
    format!(
        "username: {}, password: {}",
        input.username,
        input.password,
    )
}

async fn search() -> &'static str {
    "Search handler stub"
}
