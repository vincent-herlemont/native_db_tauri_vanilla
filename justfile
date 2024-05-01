init:
    cargo install create-tauri-app --locked --version 3.13.17
    cargo install tauri-cli --locked

start:
    cd ./src-tauri && cargo tauri dev
