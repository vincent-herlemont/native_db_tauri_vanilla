// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::api::Person;
use native_db::Database;
use once_cell::sync::Lazy;
use std::sync::Arc;

// Define models for the database / application
pub(crate) mod api {
    use native_db::{native_db, InnerKeyValue};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};

    pub type Person = v2::Person;

    pub mod v1 {
        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Person {
            #[primary_key]
            pub name: String,
            pub age: i32,
        }
    }

    pub mod v2 {
        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 2)]
        #[native_db]
        pub struct Person {
            #[primary_key]
            pub name: String,
            pub age: i32,
            pub address: String,
        }
    }
}

static DATABASE_BUILDER: Lazy<native_db::DatabaseBuilder> = Lazy::new(|| {
    let mut builder = native_db::DatabaseBuilder::new();
    builder
        .define::<api::v1::Person>()
        .expect("failed to define model Person v1");
    builder
        .define::<api::v2::Person>()
        .expect("failed to define model Person v2");
    builder
});

#[tauri::command]
fn save_person(person: Person, db: tauri::State<Arc<Database>>) {
    let rw = db
        .rw_transaction()
        .expect("failed to create rw transaction");
    rw.insert(person).expect("failed to save person");
    rw.commit().expect("failed to commit");
    println!("saved person successfully");
}

#[tauri::command]
fn load_people(db: tauri::State<Arc<Database>>) -> Vec<Person> {
    let r = db.r_transaction().expect("failed to create ro transaction");
    let people = r
        .scan()
        .primary()
        .expect("failed to scan people")
        .all()
        .collect();
    people
}

fn main() {
    let db = DATABASE_BUILDER
        // Create with a file path to persist the database
        .create_in_memory()
        .expect("failed to create database");

    let db = Arc::new(db);

    tauri::Builder::default()
        .manage(db.clone())
        .setup(move |app| {
            let local_window =
                tauri::WindowBuilder::new(app, "local", tauri::WindowUrl::App("index.html".into()))
                    .build()?;
            // Different setup for the webview ...

            // You can migrate the database here, that can be time consuming.
            // TODO: I don't know if a good idea regarding my lack of knowledge
            //       about Tauri.
            let rw = db
                .rw_transaction()
                .expect("failed to create rw migration transaction");
            rw.migrate::<Person>().expect("failed to migrate Person");
            rw.commit().expect("failed to commit migration");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_person, load_people])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
