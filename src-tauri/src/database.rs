use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use serde::{Deserialize, Serialize};
use tauri::{App, Manager, State};

pub struct DbConnection {
    pub db: std::sync::Mutex<Option<Connection>>,
}

pub trait DbAccess {
    fn run<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult;
}
impl DbAccess for State<'_, DbConnection> {
    fn run<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult,
    {
        let lock = self.db.lock().unwrap();
        let db = lock.as_ref().unwrap();

        operation(db)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

pub fn init_db(app_handle: &App) -> Result<Connection, Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    println!("Created app dir: {:?}", &app_dir);

    let sqlite_path = app_dir.join("db.sqlite");

    let mut db = match Connection::open(sqlite_path) {
        Ok(db) => Ok(db),
        Err(e) => {
            println!("Failed to open db: {:?}", e);
            Err(e)
        }
    }?;

    let migrations = Migrations::new(vec![
        M::up(
            "CREATE TABLE person (
                id   INTEGER PRIMARY KEY,
                name TEXT NON NULL,
                age  INTEGER NON NULL
            );
            INSERT INTO person (name, age) VALUES
                ('Bob Marley', 100),
                ('Fair Maiden', 20),
                ('Unfair Gentleman', 45);
            ",
        )
        .down("DROP TABLE person"),
        // M::up("").down("")
    ]);

    match migrations.to_latest(&mut db) {
        Ok(_) => {
            println!("Migrations applied");
            Ok(())
        }
        Err(e) => {
            println!("Failed to apply migrations: {:?}", e);
            Err(e)
        }
    }?;

    Ok(db)
}
