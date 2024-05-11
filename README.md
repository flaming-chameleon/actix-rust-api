[![Watch the video](https://img.youtube.com/vi/AAlvIpQafQk/maxresdefault.jpg)](https://youtu.be/AAlvIpQafQk)
[Here is my Youtube video](https://youtu.be/AAlvIpQafQk): https://youtu.be/AAlvIpQafQk


# Rust Actix Web API

This Rust project is a simple, yet functional web API built using the Actix-Web framework. It provides three primary endpoints that demonstrate basic API operations, such as responding to GET and POST requests, serializing data with Serde, and working with timestamps using the Chrono crate.

## Features

- **Ping Endpoint**: Returns a simple response to check if the server is alive.
- **Version Endpoint**: Provides API metadata including the version, author, and the current server timestamp.
- **Echo Endpoint**: Echoes back any message sent to it via a POST request, showcasing how to handle incoming JSON data.

## Endpoints

- `GET /ping`: Returns a JSON object `{ "status": "success", "data": "pong" }`.
- `GET /version`: Returns a JSON object with API metadata.
- `POST /echo`: Accepts a JSON object with a "message" field and echoes it back in the response.

## Prerequisites

Before running this project, you will need:

- Rust and Cargo (Rust's package manager): Follow the installation guide at [rust-lang.org](https://www.rust-lang.org/tools/install).
- Actix-Web and its dependencies, which can be added to your project by including them in your `Cargo.toml`.

## Setup and Running

1. **Clone the Repository**:
   ```bash
   git clone [<repository-url>](https://github.com/flaming-chameleon/actix-rust-api.git)
   cd actix-rust-api
   ```

2. **Build the Project**:
   ```bash
   cargo build
   ```

3. **Run the Server**:
   ```bash
   cargo run
   ```

This will start the server on `localhost:8080`. You can test the API endpoints using a tool like curl or Postman.

## Testing the API

You can use curl to interact with the API from the command line. Here are examples of how you might test each endpoint:

- **Ping**:
  ```bash
  curl http://localhost:8080/ping
  ```

- **Version**:
  ```bash
  curl http://localhost:8080/version
  ```

- **Echo**:
  ```bash
  curl -X POST http://localhost:8080/echo -H "Content-Type: application/json" -d '{"message":"Hello, world!"}'
  ```

## Contributions

Contributions are welcome! Please feel free to submit pull requests, create issues, or provide feedback.

---
