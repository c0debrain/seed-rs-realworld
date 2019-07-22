use crate::entity::Viewer;
use seed::storage;
use serde_json;

const STORAGE_KEY: &str = "conduit";

pub fn load_viewer() -> Option<Viewer> {
    local_storage()
        .get_item(STORAGE_KEY)
        .expect("try to get local storage item failed")
        .map(|serialized_item| {
            serde_json::from_str(&serialized_item).expect("viewer deserialization failed")
        })
}

pub fn store_viewer(viewer: &Viewer) {
    storage::store_data(&local_storage(), STORAGE_KEY, viewer);
}

pub fn delete_app_data() {
    local_storage()
        .remove_item(STORAGE_KEY)
        .expect("remove item from local storage failed");
}

fn local_storage() -> storage::Storage {
    storage::get_storage().expect("get local storage failed")
}
