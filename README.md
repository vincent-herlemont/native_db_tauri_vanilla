# Native DB + Tauri V2 + Vanilla

This is a simple example of a Tauri app using a [`native_db`](https://github.com/vincent-herlemont/native_db).

Key points:
- Define an api: 1 model `Person` with 2 versions, *source [lib.rs#L8-L51](https://github.com/vincent-herlemont/native_db_tauri_vanilla/blob/28a4b2d40fec115d5e50a71a3b3a227c52be1310/src-tauri/src/lib.rs#L8-L51)*.
- Use `native_db` as a [Tauri managed state](https://tauri.app/v1/guides/features/command/#accessing-managed-state), *source [lib.rs#L61-L85](https://github.com/vincent-herlemont/native_db_tauri_vanilla/blob/28a4b2d40fec115d5e50a71a3b3a227c52be1310/src-tauri/src/lib.rs#L61-L85)*.
- Migrate the database during the app setup, *source [lib.rs#L115-L122](https://github.com/vincent-herlemont/native_db_tauri_vanilla/blob/28a4b2d40fec115d5e50a71a3b3a227c52be1310/src-tauri/src/lib.rs#L115-L122)*.

# How to use it?

It's super simple:

1. Prerequisites:
  - Have Rust and Cargo installed
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Have Tauri CLI installed
    - `cargo install tauri-cli`

2. Clone the repository

```bash
git clone https://github.com/vincent-herlemont/native_db_tauri_vanilla.git
# Navigate to the project folder
cd native_db_tauri_vanilla/
```

3. Run the project

```bash
cargo tauri dev
```

There you go!
