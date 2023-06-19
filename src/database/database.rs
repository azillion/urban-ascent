use idb::{Database, Error, Factory, IndexParams, KeyPath, ObjectStoreParams, TransactionMode};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

use super::saves;

static DATABASE_NAME: &str = "game-0.1.0";
static DATABASE_VERSION: u32 = 1;

pub struct DatabaseManager {
    database: Database,
    saves: saves::Saves,
}

pub async fn create_database_manager() -> Result<DatabaseManager, Error> {
    // Get a factory instance from global scope
    let factory = Factory::new()?;

    // Create an open request for the database
    let mut open_request = factory.open(DATABASE_NAME, Some(DATABASE_VERSION)).unwrap();

    // Add an upgrade handler for database
    open_request.on_upgrade_needed(|event| {
        // Get database instance from event
        let database = event.database().unwrap();

        // create object store
        let saves_object_store = saves::create_object_store(&database);
    });

    // `await` open request
    let database = open_request.await;
    match database {
        Ok(database) => {
            let saves = saves::Saves::new();
            Ok(DatabaseManager { database, saves })
        }
        Err(error) => Err(error),
    }
}

// async fn add_data(database: &Database) -> Result<JsValue, Error> {
//     // Create a read-write transaction
//     let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite)?;

//     // Get the object store
//     let store = transaction.object_store("employees").unwrap();

//     // Prepare data to add
//     let employee = serde_json::json!({
//         "name": "John Doe",
//         "email": "john@example.com",
//     });

//     // Add data to object store
//     let id = store
//         .add(
//             &employee.serialize(&Serializer::json_compatible()).unwrap(),
//             None,
//         )
//         .await?;

//     // Commit the transaction
//     transaction.commit().await?;

//     Ok(id)
// }

// async fn get_data(database: &Database, id: JsValue) -> Result<Option<serde_json::Value>, Error> {
//     // Create a read-only transaction
//     let transaction = database
//         .transaction(&["employees"], TransactionMode::ReadOnly)
//         .unwrap();

//     // Get the object store
//     let store = transaction.object_store("employees").unwrap();

//     // Get the stored data
//     let stored_employee: Option<JsValue> = store.get(id).await?;

//     // Deserialize the stored data
//     let stored_employee: Option<serde_json::Value> = stored_employee
//         .map(|stored_employee| serde_wasm_bindgen::from_value(stored_employee).unwrap());

//     // Wait for the transaction to complete
//     transaction.done().await?;

//     Ok(stored_employee)
// }
