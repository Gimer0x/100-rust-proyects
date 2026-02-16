// Day 68: Project: Todo App with Actix-Web + PostgreSQL (Full CRUD)
// Create a complete full-featured Todo application backed by PostgreSQL using 
// Actix-Web and SQLx. This project ties together everything from database modeling, 
// routing, error handling, to CRUD operations—all structured for scalable web development.
// 
// What’s Different from Day 67?
// + We’ll structure this app as:
// + Organized endpoints
// + Middleware-ready
// + Documentation-friendly responses
// 
// Full support for:
// - GET /todos (List)
// - POST /todos (Create)
// - GET /todos/{id} (View one)
// - PUT /todos/{id} (Update)
// - DELETE /todos/{id} (Delete)
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}
 
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

async fn get_all(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
                    .fetch_all(pool.get_ref())
                    .await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch todos!")
    }
}

async fn get_one(id: web::Path<i32>, pool: web::Data<SqlitePool>) ->  impl Responder {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
                    .bind(*id)
                    .fetch_optional(pool.get_ref())
                    .await;
    
    match result {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("Query failed"),
    }
}

async fn create_todo(data: web::Json<CreateTodo>, pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query("INSERT INTO todos (title) VALUES ($1)")
        .bind(&data.title)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Todo created!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create todo!")
    }
}

async fn update_todo(id: web::Path<i32>, data: web::Json<UpdateTodo>, pool: web::Data<SqlitePool>) -> impl Responder{
    let current = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(*id)
            .fetch_optional(pool.get_ref())
            .await;
    
    if let Ok(Some(todo)) = current {
        let new_title = data.title.clone().unwrap_or(todo.title);
        let new_completed = data.completed.unwrap_or(todo.completed);

        let result = sqlx::query("UPDATE todos SET title = $1, completed= $2 WHERE id = $3")
                        .bind(new_title)
                        .bind(new_completed)
                        .bind(*id)
                        .execute(pool.get_ref())
                        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("Todo updated!"),
            Err(_) => HttpResponse::InternalServerError().body("Update failed!")
        }
    } else {
        HttpResponse::NotFound().body("Todo not found!")
    }
}

async fn delete_todo(id: web::Path<i32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("DELETE FROM todos WHERE id = $1")
            .bind(*id)
            .fetch_optional(pool.get_ref())
            .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Delete completed!"),
        Err(_) => HttpResponse::InternalServerError().body("Delete failed!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DB not set!");
    let db = SqlitePool::connect(&db_url).await.expect("Failed to connect!");
    
    println!("Todo App running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .app_data(web::Data::new(db.clone()))
            .route("/todos", web::get().to(get_all))
            .route("/todos/{id}", web::get().to(get_one))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
