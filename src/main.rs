// Import necessary crates and modules
use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use serde::{ Serialize, Deserialize };
use chrono::Utc; // Chrono crate for handling dates and times

// Define a generic API response structure with Serialize trait for JSON responses
#[derive(Serialize)]
struct ApiResponse<T> where T: Serialize {
    status: &'static str,
    data: T,
}

// Structure to hold API metadata information
#[derive(Serialize)]
struct ApiMeta {
    author: &'static str,
    version: &'static str,
    timestamp: String,
}

// Handler function for the "/ping" endpoint
// This function responds with a simple JSON object indicating success
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        status: "success",
        data: "pong",
    })
}

// Handler function for the "/version" endpoint
// This function returns API version information along with the current server time and author information
async fn version() -> impl Responder {
    let meta = ApiMeta {
        author: "Flaming Chameleon",
        version: "1.0.0",
        timestamp: Utc::now().to_rfc3339(), // Generate the current time in RFC3339 format
    };
    HttpResponse::Ok().json(ApiResponse {
        status: "success",
        data: meta,
    })
}

// Data structure for receiving JSON data through POST requests
#[derive(Deserialize)]
struct EchoData {
    message: String,
}

// Handler function for POST requests to "/echo"
// Echoes back the message received in the request
async fn echo(data: web::Json<EchoData>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        status: "success",
        data: data.into_inner().message,
    })
}

// Main function to create and run the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping)) // Route for GET requests to "/ping"
            .route("/version", web::get().to(version)) // Route for GET requests to "/version"
            .route("/echo", web::post().to(echo)) // Route for POST requests to "/echo"
    })
        .bind("127.0.0.1:8080")
        ? // Bind server to localhost on port 8080
        .run().await // Start the server asynchronously
}
