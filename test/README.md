## Setting up testing automation and application:
* Pre-requisites:
    * [Rust](https://rustup.rs/.) installed
    * [Python](https://www.python.org/downloads/) installed
    
## Building the application:
* Building the application with Dockerfile and docker compose:
    * cargo build
    * cargo run -- pytest_config.yml

## Running automation:
* pip install -r requirements.txt
* python -m pytest -m Smoke