use email_newsletter::run;
use std::net::TcpListener;

use actix_web::web;
#[derive(serde::Deserialize)]
struct FormData {
    username: String,
}
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.username)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create the listener first
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Pass it to run()
    run(listener)?.await
}
