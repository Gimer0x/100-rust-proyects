// Day 65: Project: JWT Authentication in Actix-Web
// Implement secure JWT-based authentication in Actix-Web. You'll create login endpoints
// that return a JSON Web Token, and a protected route that requires a valid token in the
// request header. This is essential for stateless API security.
//
// Key Concepts:
// + jsonwebtoken for secure, signed tokens
// + Authorization headers for bearer tokens
// + Token claims include user identity and expiration
// You've now implemented stateless session authentication, crucial for secure APIs, user
// dashboards, and mobile-first services.

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};

// Secret key used to sign and verify JWTs. In production, use an env var and a long random key.
const SECRET_KEY: &[u8] = b"supersecretkeyyoushouldchange";

// -----------------------------------------------------------------------------
// JWT Claims: the payload inside the token (who + when it expires).
// -----------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // "sub" (subject): identifies the user (e.g. username).
    sub: String,
    // "exp" (expiration): Unix timestamp when the token becomes invalid.
    exp: usize,
}

// -----------------------------------------------------------------------------
// Request body for POST /login: username and password from the client.
// -----------------------------------------------------------------------------
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// -----------------------------------------------------------------------------
// POST /login — Authenticate user and return a JWT if credentials are valid.
// Logic: check username/password → if valid, build claims → sign token → return JSON.
// -----------------------------------------------------------------------------
async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    // Hardcoded check for demo; in production use a DB and hashed passwords.
    if req.username == "admin" && req.password == "password" {
        // Build claims: who (sub) and expiration (now + 1 hour).
        let claims = Claims {
            sub: req.username.clone(),
            exp: chrono::Utc::now().timestamp() as usize + 3600,
        };
        // Sign the token with HS256 using SECRET_KEY; the server can later verify it.
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET_KEY),
        )
        .expect("Token creation failed!");
        // Return 200 OK with JSON: { "token": "<jwt>" }.
        HttpResponse::Ok().json(serde_json::json!({"token": token}))
    } else {
        // Wrong username or password → 401 Unauthorized.
        HttpResponse::Unauthorized().body("Invalid credentials!")
    }
}

// -----------------------------------------------------------------------------
// GET /protected — Requires a valid JWT in the Authorization header.
// Logic: read "Authorization: Bearer <token>" → decode and verify token → respond with user.
// -----------------------------------------------------------------------------
async fn protected(req: HttpRequest) -> impl Responder {
    // Get the Authorization header (e.g. "Bearer eyJhbGc...").
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(header_value) = auth_header {
        // Expect format "Bearer <token>"; strip the "Bearer " prefix to get the token.
        if let Some(token) = header_value.strip_prefix("Bearer ") {
            // Validate using HS256 and SECRET_KEY (same as when we signed).
            let validation = Validation::new(Algorithm::HS256);
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET_KEY),
                &validation,
            );
            return match token_data {
                // Token valid: use the claims (e.g. sub) to personalize the response.
                Ok(data) => HttpResponse::Ok().body(format!("Welcome, {}!", data.claims.sub)),
                // Token invalid or expired.
                Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
            };
        }
    }

    // No header or not "Bearer <token>" format.
    HttpResponse::Unauthorized().body("Authorization header missing or malformed")
}

// -----------------------------------------------------------------------------
// Entry point: bind server and register routes.
// -----------------------------------------------------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("JWT Auth API running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
