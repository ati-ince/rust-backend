# Rust Backend API Server

A simple Rust backend API server with Docker support.

## Prerequisites

- Docker
- Python 3.x (for test client)
- pip (Python package manager)

## Setup

1. Install Python dependencies for the test client:
```bash
pip install requests
```

## Running the Server

### Using Docker (Recommended)

1. Build the Docker image:
```bash
docker build -t rust-backend .
```

2. Run the container:
```bash
docker run -p 8000:8000 rust-backend
```

### Running Locally

1. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build and run the server:
```bash
cargo run
```

## Testing the Server

Run the test client script:
```bash
python test_client.py
```

The test client will attempt to connect to the server at http://localhost:8000 and display the response.

## API Endpoints

- `GET /`: Returns a simple JSON message
  - Response: `{"content": "Hello from Rust Backend!"}`

## Development

- The server runs on port 8000
- The main server code is in `src/main.rs`
- The test client is in `test_client.py`

## Docker Commands

- Build image: `docker build -t rust-backend .`
- Run container: `docker run -p 8000:8000 rust-backend`
- Stop container: `docker stop $(docker ps -q --filter ancestor=rust-backend)` 