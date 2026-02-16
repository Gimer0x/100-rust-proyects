// Day 63: Project: User Authentication API (Actix-Web)
// Build a basic user authentication API with login and signup endpoints using in-memory storage.
// You’ll practice password hashing, POST data handling, and secure API logic—a key step for secure 
// web applications.
// Key Concepts:
// + bcrypt for password hashing & verification
// + Mutex<HashMap> for safe shared user state
// + serde for JSON parsing and response
// You now have a basic authentication backend—perfect for login-protected apps, token systems, and 
// user registration workflows.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Clone)]
struct User {
    username: String,
    hashed_password: String,
}

struct AppState {
    users: Mutex<HashMap<String, User>>,
}

// Handler parameters (Actix extracts these from the request):

// req: parsed JSON body as SignupRequest (username, password, etc.).
//      Invalid JSON or wrong shape returns 400 before the handler runs.
// req: web::Json<SignupRequest>,

// data: shared application state (e.g. DB pool, in-memory store).
//       Same AppState instance registered with .app_data() in main.
// data: web::Data<AppState>
async fn signup(req: web::Json<SignupRequest>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();

    if users.contains_key(&req.username) {
        return HttpResponse::Conflict().body("User exists!");
    }

    let hashed = hash(&req.password, DEFAULT_COST).unwrap();
    let user = User {
        username: req.username.clone(),
        hashed_password: hashed,
    };

    users.insert(req.username.clone(), user);
    HttpResponse::Created().body("User created!")
}

// req: web::Json<SignupRequest>
// + web::Json<T> is an extractor: it reads the request body, parses it as JSON, and deserializes it into T.
// + So req is the JSON body of the request, already parsed as a SignupRequest (e.g. username, email, password).
// + If the body isn’t valid JSON or doesn’t match SignupRequest, Actix returns 400 before your handler runs.
// + You use req (e.g. req.username, req.password) or req.into_inner() to get the inner SignupRequest value.

// data: web::Data<AppState>
// + web::Data<T> is an extractor for application state you registered with .app_data(...).
// + AppState is your shared state (e.g. DB pool, config, in-memory store).
// + data is a shared reference to that state, so you can read/write it (e.g. access a DB or a Mutex<Vec<...>>) from inside the handler.
// + Actix clones the inner Arc when injecting; it doesn’t move your AppState.

async fn login(req: web::Json<SignupRequest>, data: web::Data<AppState>) -> impl Responder {
    // .lock() tries to acquire the mutex.
    // If no one else holds it: you get a guard that lets you use the inner value (e.g. the Vec).
    // If another request is holding it: the current thread blocks until the lock is released.
    let users = data.users.lock().unwrap();

    match users.get(&req.username) {
        Some(user) => {
            if verify(&req.password, &user.hashed_password).unwrap_or(false) {
                HttpResponse::Ok().body("Login successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid password!")
            }
        }
        None => HttpResponse::NotFound().body("User not found!")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("🔐 Auth API running at http://127.0.0.1:8080");
 
    let users = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
