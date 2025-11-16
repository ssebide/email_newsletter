#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

//lets start simple, we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
