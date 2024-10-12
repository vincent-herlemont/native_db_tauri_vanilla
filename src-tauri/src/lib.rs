use crate::api::Person;
use itertools::Itertools;
use native_db::{Database, Models};
use once_cell::sync::Lazy;
use tauri::Manager;

// Define models for the database / application
pub(crate) mod api {
    use native_db::{native_db, ToKey};
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

        impl From<super::v1::Person> for Person {
            fn from(person: super::v1::Person) -> Self {
                Self {
                    name: person.name,
                    age: person.age,
                    address: "".to_string(),
                }
            }
        }
    }
}

static DATABASE_MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<api::v1::Person>().unwrap();
    models.define::<api::v2::Person>().unwrap();
    models
});


#[tauri::command]
fn save_person(person: Person, db: tauri::State<Database>) {
    println!("saving person: {:?}", person);
    let rw = db
        .rw_transaction()
        .expect("failed to create rw transaction");
    rw.insert(person).expect("failed to save person");
    rw.commit().expect("failed to commit");
    println!("saved person successfully");
}

#[tauri::command]
fn load_people(db: tauri::State<Database>) -> Vec<Person> {
    let r = db.r_transaction().expect("failed to create ro transaction");

    let people = r
        .scan()
        .primary()
        .expect("fail to scan primary") // Instead of .expect(...) method prefer to use ? operator.
        .all()
        .expect("failed to scan all") // Same as above
        .try_collect()
        .expect("failed to collect"); // Same as above
    people
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create a database in memory
    let db = native_db::Builder::new()
        .create_in_memory(&DATABASE_MODELS)
        .unwrap();

    // Insert some data
    let rw = db
        .rw_transaction()
        .expect("failed to create rw transaction");
    rw.insert(Person {
        name: "John".to_string(),
        age: 30,
        address: "123 Main St".to_string(),
    })
    .expect("failed to insert person");
    rw.commit().expect("failed to commit");

    tauri::Builder::default()
        .setup(move |app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
              let window = app.get_webview_window("main").unwrap();
              window.open_devtools();
              window.close_devtools();
            }

            // You can migrate the database here, that can be time consuming.
            let rw = db
                .rw_transaction()
                .expect("failed to create rw migration transaction");
            rw.migrate::<Person>().expect("failed to migrate Person");
            rw.commit().expect("failed to commit migration");

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_person, load_people])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
