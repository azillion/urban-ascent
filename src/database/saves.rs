use idb::{
    Database, Error, IndexParams, KeyPath, ObjectStore, ObjectStoreParams, Query, TransactionMode,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

static OBJECT_STORE_NAME: &str = "saves";

#[derive(Debug, Serialize, Deserialize)]
struct Save {
    id: u32,
    name: String,
    mayor: String,
    city_name: String,
    created_at: u32,
    updated_at: u32,
}

pub struct Saves {}

impl Saves {
    pub fn new() -> Self {
        Saves {}
    }

    pub async fn add_save(&self, database: &Database, save: Save) -> Result<JsValue, Error> {
        let transaction = database.transaction(&[OBJECT_STORE_NAME], TransactionMode::ReadWrite)?;
        let object_store = transaction.object_store(OBJECT_STORE_NAME).unwrap();

        let id = object_store
            .add(
                &save.serialize(&Serializer::json_compatible()).unwrap(),
                None,
            )
            .await?;

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn get_save(&self, database: &Database, id: JsValue) -> Result<Option<Save>, Error> {
        let transaction = database.transaction(&[OBJECT_STORE_NAME], TransactionMode::ReadOnly)?;
        let object_store = transaction.object_store(OBJECT_STORE_NAME).unwrap();
        let save = object_store.get(id).await?;
        match save {
            Some(save) => {
                let save = serde_wasm_bindgen::from_value(save).unwrap();
                Ok(Some(save))
            }
            None => Ok(None),
        }
    }

    // pub async fn get_all_saves(&self) -> Result<Vec<Save>, Error> {
    //     let lower: JsValue = 1.into();
    //     let upper: JsValue = 1000.into();
    //     let key_range = idb::KeyRange::bound(&lower, &upper, Some(true), Some(true))?;
    //     let query = Query::KeyRange(key_range);
    //     let saves = self.object_store.get_all(Some(query), Some(1000)).await?;
    //     let saves: Vec<Save> = saves
    //         .into_iter()
    //         .map(|save| serde_wasm_bindgen::from_value(save).unwrap())
    //         .collect();
    //     Ok(saves)
    // }

    // pub async fn update_save(&self, save: Save, id: &JsValue) -> Result<JsValue, Error> {
    //     let id = self
    //         .object_store
    //         .put(
    //             &save.serialize(&Serializer::json_compatible()).unwrap(),
    //             Some(id),
    //         )
    //         .await?;
    //     Ok(id)
    // }

    // pub async fn delete_save(&self, id: JsValue) -> Result<(), Error> {
    //     self.object_store.delete(id).await?;
    //     Ok(())
    // }

    // pub async fn clear_saves(&self) -> Result<(), Error> {
    //     self.object_store.clear().await?;
    //     Ok(())
    // }
}

pub fn create_object_store(database: &Database) -> ObjectStore {
    let object_params = create_object_params(KeyPath::new_single("id"));
    let object_store = database
        .create_object_store(OBJECT_STORE_NAME, object_params)
        .unwrap();
    setup_index(&object_store);

    object_store
}

fn setup_index(object_store: &ObjectStore) {
    let index_params = create_index_params();
    object_store
        .create_index("name", KeyPath::new_single("name"), Some(index_params))
        .unwrap();
}

fn create_index_params() -> IndexParams {
    let mut index_params = IndexParams::new();
    index_params.unique(true);
    index_params
}

fn create_object_params(key_path: KeyPath) -> ObjectStoreParams {
    let mut object_params = ObjectStoreParams::new();
    object_params.auto_increment(true);
    object_params.key_path(Some(key_path));
    object_params
}
