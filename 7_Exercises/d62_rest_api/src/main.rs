// Day 62: Project: REST API Server with Actix-Web
// Create a simple RESTful API using Actix-Web that handles CRUD-like 
// HTTP requests for a Book resource. You’ll learn routing, JSON serialization, 
// and handling dynamic data via endpoints.
// Key Concepts:
// + web::Data: App state sharing with threads
// + web::Json: Automatically deserializes/serializes JSON
// + Mutex: Enables safe mutation of shared state
// You’ve now built a working REST API server with shared state, data serialization, 
// and endpoint handling—core skills for backend microservices, APIs, and app backends.

// Import Actix types for routing, server setup, and HTTP responses.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// Import Serde traits to serialize/deserialize Book to/from JSON.
use serde::{Deserialize, Serialize};
// Import Mutex to allow safe, shared mutation of the books list.
use std::sync::Mutex;

// Auto-derive JSON support and cloning for Book values.
#[derive(Serialize, Deserialize, Clone)]
// Data model for a book resource in the API.
struct Book {
    id: usize,
    title: String,
    author: String,
}

// Application state shared across handlers.
struct AppState {
    // Shared, mutable list of books protected by a mutex.
    books: Mutex<Vec<Book>>,
}

// GET /books handler: returns the current list of books as JSON.
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    // Lock the mutex to read the books safely.
    let books = data.books.lock().unwrap();
    // Serialize the list to JSON and return a 200 OK response.
    HttpResponse::Ok().json(&*books)
}

// POST /books handler: adds a new book from JSON payload.
async fn add_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    // Lock the mutex to mutate the books safely.
    let mut books = data.books.lock().unwrap();
    // Move the JSON payload into the vector.
    books.push(book.into_inner());
    // Return a 201 Created response.
    HttpResponse::Created().body("Book added!")
}

// Actix runtime entry point for async main.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print the server address for convenience.
    println!("REST API running at http://127.0.0.1:8080");

    // Initialize shared application state with an empty list of books.
    let books = web::Data::new(AppState {
        books: Mutex::new(vec![]),
    });

    // Build and run the HTTP server.
    HttpServer::new(move || {
        // Create the Actix app and register shared state and routes.
        App::new()
            .app_data(books.clone())
            .route("/books", web::get().to(get_books))
            .route("/books", web::post().to(add_book))
    })
    // Bind the server to localhost:8080.
    .bind(("127.0.0.1", 8080))?
    // Start the server.
    .run()
    // Await the server future so the process stays alive.
    .await
}


// Test with curl or Postman:
// GET all books
// 
// curl http://127.0.0.1:8080/books
// POST new book
// 
// curl -X POST -H "Content-Type: application/json" \
// -d '{"id":1,"title":"Rust in Action","author":"Tim McNamara"}' \
// http://127.0.0.1:8080/books