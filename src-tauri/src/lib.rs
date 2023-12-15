use tauri::{Manager, State};

mod database;

use database::{DbAccess, DbConnection, Person};

#[tauri::command]
fn query(db: State<DbConnection>, name: String) -> Result<Vec<Person>, String> {
    db.run(|db| -> Result<Vec<Person>, rusqlite::Error> {
        let mut stmt = db.prepare("SELECT id, name, age FROM person WHERE name LIKE ?1")?;

        let person_iter = stmt.query_map([format!("%{}%", name)], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })?;

        let mut persons = vec![];

        for person in person_iter {
            persons.push(person?);
        }

        Ok(persons)
    })
    .map_err(|e| format!("{:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbConnection {
            db: Default::default(),
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![query])
        .setup(|app| {
            let db = database::init_db(&app)?;

            let db_state: State<DbConnection> = app.state();
            *db_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
