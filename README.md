# URL Shortner ![Build & Test](https://github.com/johnvanham/url-shortener-rust/actions/workflows/rust.yml/badge.svg)

## Overview

Simple URL shortner written in Rust as a learning experience. Feel free to use it for your own purposes.

It uses:

- [Redis](https://redis.io/) as the backend database
- [Rocket](https://rocket.rs/) web framework
- [Tailwind CSS](https://tailwindcss.com/) for some styling
- [Docker](https://www.docker.com/) for running redis and deploying the URL shortner as a docker container cross-compiled to run on a Raspberry Pi

### Required tools

1. [Install rust](https://www.rust-lang.org/tools/install). This project was tested using the stable channel which at the time of writing installed rustc 1.70.0
2. [Install the standalone Tailwind CSS CLI](https://tailwindcss.com/blog/standalone-cli). You could also use npm but I didn't want a package.json or node_modules when this was mainly for me to learn some rust development

### Cloning the repo

Once you have the tools installed, clone this repo using your preferred method. For example...

`gh repo clone johnvanham/url-shortener-rust`

### Running

1. Start up redis using Docker: `docker run -p 6379:6379 -v redis_data:/data -it redis:latest`
2. Use cargo in the project directory `cargo run`

### Deploying

_TODO: Add instructions on how to deploy as a docker container cross-compiled to run on a Raspberry Pi_
