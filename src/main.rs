use email_newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create the listener first
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Pass it to run()
    run(listener)?.await
}
