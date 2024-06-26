set shell := ["nu", "-c"]

init:
    cargo install create-tauri-app --locked --version 3.13.17; \
    cargo install tauri-cli --locked --version 1.5.13

start:
    cd ./src-tauri; \
    cargo tauri dev

expand:
    cd ./src-tauri; \
    cargo expand | save -f --raw build.expanded.rs