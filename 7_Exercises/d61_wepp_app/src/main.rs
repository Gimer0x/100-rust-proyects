// Day 61: Project: Hello Actix-Web App
// Create your first HTTP web server using the Actix-Web framework. This simple app responds 
// with "Hello, Rust Web!" and introduces routing, handlers, and basic server setup.
// Key Concepts:
// + #[get("/")]: Registers a handler for GET requests at /
// + HttpResponse::Ok().body(...): Returns a plain response
// + HttpServer::new(): Starts the web server
// You've just created your first Actix-Web server. From here, we’ll build more advanced 
// RESTful APIs, connect databases, and add frontend components.
use actix_web::{get, App, HttpServer, Responder, HttpResponse}; // Actix web types and macros.

//  route macro that registers the hello function as the 
// handler for HTTP GET requests to "/".
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Rusteceans!")
}

// macro that sets up the async runtime Actix uses (based on Tokio). 
// It lets you write async fn main().
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting server at http://127.0.0.1:8080");

    // Create a new server factory for each worker thread.
    HttpServer::new(|| {
        // Build the application with routes and middleware.
        App::new() 
            .service(hello) // Register the route handler.
    })
    .bind(("127.0.0.1", 8080))? // Bind to host and port.
    .run() // Start the server.
    .await // Wait for the server future to complete.
}

