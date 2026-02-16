// Day 69: Project: Blog API Backend (Posts, Comments, Categories)
// Build a full-featured Blog API backend using Actix-Web and PostgreSQL. You’ll implement 
// endpoints for posts, comments, and categories, creating relationships and learning about 
// joined data, foreign keys, and nested querying.

// Key Concepts:
// + Foreign key relationships
// + Handling multiple resources with different models
// + REST architecture with structured endpoints
// You’ve built a multi-entity Blog API with full database integration—ideal for building 
// real blog engines, CMS systems, or educational platforms.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};

// FromRow: Tells SQLx how to build a struct from a database row.
// Used with query_as::<_, YourStruct>() so each row is turned into a YourStruct (e.g. Comment, Post, Todo).
// SQLx needs a way to map column values (names and types) into your struct’s fields. 
// FromRow (usually #[derive(FromRow)]) does that: it matches column names to field names and 
// handles types.
// FromRow = “turn a DB row into this struct.”
#[derive(Serialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    body: String,
    category_id: i32,
}

// Serialize: Tells Serde how to turn a struct into JSON (or other formats).
// When: You return that struct in an HTTP response, e.g. HttpResponse::Ok().json(comments).
// Why: Actix’s .json() uses Serde; it only works for types that implement Serialize. 
// Then the response body is JSON like [{"id":1,"author":"Alice",...}].
// Without it: You can’t use .json(your_struct); you’d have to build the 
// JSON string yourself or use another way to serialize.
// Serialize = “turn this struct into JSON for the response.”
#[derive(Serialize, FromRow)]
struct Comment {
    id: i32,
    post_id: i32,
    author: String,
    content: String,
}
 
#[derive(Serialize, FromRow)]
struct Category {
    id: i32,
    name: String,
}
 
#[derive(Deserialize)]
struct NewPost {
    title: String,
    body: String,
    category_id: i32,
}
 
#[derive(Deserialize)]
struct NewComment {
    post_id: i32,
    author: String,
    content: String,
}
 
#[derive(Deserialize)]
struct NewCategory {
    name: String,
}

// web::Data<SqlitePool> is how Actix gives your handler access to the shared db connection pool.
async fn list_posts(pool: web::Data<SqlitePool>) -> impl Responder {
    // pool.get_ref() gives you a shared reference (&SqlitePool) to the pool inside web::Data<SqlitePool>
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY id")
                .fetch_all(pool.get_ref())
                .await;
    
    HttpResponse::Ok().json(posts.unwrap())
}

// Use query for writes (insert, update and delete) and when you don’t need a struct per row;
async fn create_post(pool: web::Data<SqlitePool>, data: web::Json<NewPost>) -> impl Responder {
    let result = sqlx::query("INSERT INTO posts (title, body, category_id) VALUES ($1, $2, $3)")
                    .bind(&data.title)
                    .bind(&data.body)
                    .bind(&data.category_id)
                    .execute(pool.get_ref())
                    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Post created!"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating post!")
    }
}


async fn list_comments(pool: web::Data<SqlitePool>) -> impl Responder {
    // use query_as when you want SELECT results as Rust structs.
    // ::<_, Comment>	Type parameters: “Map each row to Comment.” The _ is for the database 
    // drive with "_" SQLx infers it, (e.g. Sqlite).
    let comments = sqlx::query_as::<_, Comment>("SELECT * FROM comments ORDER BY id")
                    .fetch_all(pool.get_ref())
                    .await;

    HttpResponse::Ok().json(comments.unwrap())
}

async fn add_comment(pool: web::Data<SqlitePool>, data: web::Json<NewComment>) -> impl Responder{
    let result = sqlx::query("INSERT INTO comments(post_id, author, content) VALUES ($1, $2, $3)")
                    .bind(&data.post_id)
                    .bind(&data.author)
                    .bind(&data.content)
                    .execute(pool.get_ref())
                    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("New comment added!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add comment!")
    }
}

async fn list_categories(pool: web::Data<SqlitePool>) -> impl Responder {
    let cats = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool.get_ref())
        .await;
    HttpResponse::Ok().json(cats.unwrap())
}

async fn add_category(pool: web::Data<SqlitePool>, data: web::Json<NewCategory>) -> impl Responder {
    let result = sqlx::query("INSERT INTO categories (name) VALUES ($1)")
        .bind(&data.name)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("Category added"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create category"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = SqlitePool::connect(&db_url).await.expect("Could not connect to DB");
 
    println!("Blog API running at http://localhost:8080");
 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/posts", web::get().to(list_posts))
            .route("/posts", web::post().to(create_post))
            .route("/comments", web::get().to(list_comments))
            .route("/comments", web::post().to(add_comment))
            .route("/categories", web::get().to(list_categories))
            .route("/categories", web::post().to(add_category))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}