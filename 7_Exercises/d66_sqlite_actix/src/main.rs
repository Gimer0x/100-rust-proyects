// Day 66: Project: SQLite Integration with Actix-Web
// Integrate an SQLite database into your Actix-Web server using SQLx, enabling persistent 
// storage for your app. You’ll learn to define models, execute queries, and connect your 
// API with a real relational database.

// Key Concepts:
// + sqlx::query_as maps DB rows to structs
// + SQLite is perfect for local development
// + SqlitePool provides safe, async connection pooling
// You now have a real persistent backend using SQLite—ideal for apps like blogs, admin dashboards, 
// or embedded systems.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
struct Post {
    id: i64,
    title: String,
    content: String,
}
 
#[derive(Deserialize)]
struct NewPost {
    title: String,
    content: String,
}

async fn get_posts(db: web::Data<SqlitePool>) ->impl Responder {
    let posts = sqlx::query_as::<_, Post>("Select * from posts")
        .fetch_all(db.get_ref())
        .await;

    match posts {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e))
    }
}

async fn add_post(db: web::Data<SqlitePool>, json: web::Json<NewPost>) -> impl Responder {
    let result = sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?)")
                    .bind(&json.title)
                    .bind(&json.content)
                    .execute(db.get_ref())
                    .await;

    match result{
        Ok(_) => HttpResponse::Created().body("Post created"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert failed: {}", e))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Blog API w/ SQLite at http://127.0.0.1:8080");

    let db = SqlitePool::connect("sqlite:blog.db").await.expect("DB connect failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/posts", web::get().to(get_posts))
            .route("/posts", web::post().to(add_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
