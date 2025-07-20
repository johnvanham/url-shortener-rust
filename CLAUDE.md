# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- **Run the application**: `RUST_LOG=info cargo run`
- **Build for release**: `cargo build --release`
- **Build CSS**: `tailwindcss -i css/style.css -o public/style.css --watch`
- **Build Docker image**: `docker build -t url-shortener .`
- **Run with Docker Compose**: `docker compose up --build`

## Environment Variables

- `DATABASE_URL`: Optional. SQLite database file path (defaults to "urls.db" in current directory)
- `RUST_LOG`: Optional. Log level (recommended: "info" for development)

## Architecture

This is a simple URL shortener built with Rust using the Rocket web framework and SQLite for storage.

### Core Components

- **main.rs**: Web server with Rocket routes
  - `/` - Serves the HTML form
  - `/shorten` - POST endpoint that accepts key/url pairs and stores them in SQLite
  - `/<key>` - GET endpoint that redirects to the stored URL
  - `/style.css` - Serves the compiled Tailwind CSS

- **db_handler.rs**: SQLite operations module
  - `init_database()` - Creates the URLs table if it doesn't exist
  - `get_url_from_db()` - Retrieves URL by key
  - `set_url_in_db()` - Stores key/URL pair (uses INSERT OR REPLACE)
  - `get_db_path()` - Gets database file path from environment

### Key Design Patterns

- URLs are stored in SQLite with the prefix "surl_" + user-provided key
- Database schema includes key (TEXT PRIMARY KEY), url (TEXT NOT NULL), and created_at timestamp
- Error handling returns user-friendly messages for database failures
- Static files (HTML, CSS) are served directly from the `public/` directory
- Form data is processed using Rocket's `Form` derive macro
- Comprehensive logging using env_logger for debugging and monitoring

### Development Workflow

1. Run `RUST_LOG=info cargo run` for the Rust server with logging
2. Run Tailwind CSS watcher for style changes
3. The application serves on Rocket's default port (8000)
4. SQLite database file is created automatically on first run

### Testing with curl

```bash
# Add a URL
curl -X POST -d "key=test&url=https://www.google.com" http://127.0.0.1:8000/shorten

# Test redirect (should return 303 with location header)
curl -I http://127.0.0.1:8000/test

# Test non-existent key (should return 404)
curl -I http://127.0.0.1:8000/nonexistent
```

CSS changes require recompiling with Tailwind CLI since this project uses the standalone version rather than npm.