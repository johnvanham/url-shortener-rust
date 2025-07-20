# URL Shortener ![Build & Test](https://github.com/johnvanham/url-shortener-rust/actions/workflows/rust.yml/badge.svg)

## Overview

Simple URL shortener written in Rust as a learning experience. Feel free to use it for your own purposes.

It uses:

- [SQLite](https://www.sqlite.org/) as the backend database
- [Rocket](https://rocket.rs/) web framework
- [Tailwind CSS](https://tailwindcss.com/) for some styling
- [Docker](https://www.docker.com/) for containerized deployment
- Comprehensive logging with [env_logger](https://docs.rs/env_logger/latest/env_logger/)

### Required tools

1. [Install rust](https://www.rust-lang.org/tools/install). This project was tested using the stable channel which at the time of writing installed rustc 1.70.0
2. [Install the standalone Tailwind CSS CLI](https://tailwindcss.com/blog/standalone-cli). You could also use npm but I didn't want a package.json or node_modules when this was mainly for me to learn some rust development

### Cloning the repo

Once you have the tools installed, clone this repo using your preferred method. For example...

`gh repo clone johnvanham/url-shortener-rust`

### Running Locally

1. Use cargo in the project directory with logging enabled: `RUST_LOG=info cargo run`
2. Run tailwindcss to build the css file and (optionally) watch changes: `tailwindcss -i css/style.css -o public/style.css --watch`

The application will:
- Create a SQLite database file (`urls.db`) automatically on first run
- Start the web server on `http://127.0.0.1:8000`
- Log all operations to help with debugging

If editing any of the rust code you will need to stop and re-start cargo.

### Running with Docker

Using Docker Compose (recommended for development):
```bash
docker compose up --build
```

Or build and run manually:
```bash
docker build -t url-shortener .
docker run -p 8000:8000 -v $(pwd)/data:/app/data url-shortener
```

### Deploying

#### Using Pre-built Image from GitHub Container Registry

The easiest way to deploy is using the pre-built image:

```bash
# Pull the latest image
docker pull ghcr.io/johnvanham/url-shortener-rust:latest

# Run with persistent data storage
docker run -d \
  --name url-shortener \
  -p 8000:8000 \
  -v url_shortener_data:/app/data \
  -e RUST_LOG=info \
  ghcr.io/johnvanham/url-shortener-rust:latest
```

#### Building Your Own Image

```bash
# Build the image
docker build -t url-shortener .

# Run with data persistence
docker run -d \
  --name url-shortener \
  -p 8000:8000 \
  -v $(pwd)/data:/app/data \
  -e RUST_LOG=info \
  url-shortener
```

#### Environment Variables

- `DATABASE_URL`: Path to SQLite database file (default: `/app/data/urls.db`)
- `RUST_LOG`: Log level (recommended: `info` for production, `debug` for troubleshooting)

## Usage

### Web Interface

Navigate to `http://127.0.0.1:8000` in your browser to use the web interface:

1. Enter a custom key (e.g., "google")
2. Enter the URL to shorten (e.g., "https://www.google.com")
3. Click "Shorten"
4. Access your shortened URL at `http://127.0.0.1:8000/google`

### API Testing with curl

```bash
# Add a URL
curl -X POST -d "key=test&url=https://www.google.com" http://127.0.0.1:8000/shorten

# Test the redirect (should return 303 with location header)
curl -I http://127.0.0.1:8000/test

# Test a non-existent key (should return 404)
curl -I http://127.0.0.1:8000/nonexistent
```

### Example Response

```bash
$ curl -X POST -d "key=example&url=https://www.example.com" http://127.0.0.1:8000/shorten
URL shortened successfully: Stored URL for key: surl_example

$ curl -I http://127.0.0.1:8000/example
HTTP/1.1 303 See Other
location: https://www.example.com
server: Rocket
...
```
