use actix_web::{get, web, App, HttpServer, Responder};
use mongodb::{Client, Collection};
use serde::{Serialize, Deserialize};
use futures_util::stream::StreamExt;


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    // Conectarse a MongoDB
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = client.database("users");
    let collection: Collection<User> = db.collection("users");

    // Obtener la lista de usuarios desde la base de datos
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut users = Vec::new();
    while let Some(result) = cursor.next().await {

        if let Ok(user) = result {
            users.push(user);
        }
    }

    // Convertir la lista de usuarios a JSON y devolverla
    web::Json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
