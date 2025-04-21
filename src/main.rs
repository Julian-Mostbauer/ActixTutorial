mod endpoints;
mod persistence;

use endpoints::*;
use persistence::*;

use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct UserInputData {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(load_users_from_file()));

    let user_db_clone = user_db.clone();
    ctrlc::set_handler(move || {
        let db = user_db_clone.lock().unwrap();
        save_users_to_file(&db);
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    HttpServer::new(move || {
        let user_db = user_db.clone();
        App::new()
            .app_data(web::Data::new(user_db))
            .service(index)
            .service(create_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
