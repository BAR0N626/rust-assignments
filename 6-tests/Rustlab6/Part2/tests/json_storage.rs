use snippets_app::model::Snippet;
use snippets_app::storage::json::JsonStorage;
use snippets_app::storage::SnippetStorage;

use tempfile::tempdir;

#[test]
fn json_storage_empty_list_when_file_missing() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snippets.json");

    let storage = JsonStorage::new(path);
    let list = storage.list().unwrap();

    assert!(list.is_empty());
}

#[test]
fn json_storage_save_and_get_by_title() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snippets.json");
    let mut storage = JsonStorage::new(path);

    let id = storage.next_id().unwrap();
    let snip = Snippet::new(id, "hello".to_string(), "println!(\"hi\");".to_string()).unwrap();
    storage.save(snip).unwrap();

    let loaded = storage.get_by_title("hello").unwrap().unwrap();
    assert_eq!(loaded.title, "hello");
    assert_eq!(loaded.code, "println!(\"hi\");");
}

#[test]
fn json_storage_upsert_by_title_replaces_code() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snippets.json");
    let mut storage = JsonStorage::new(path);

    let id1 = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id1, "same".to_string(), "one".to_string()).unwrap())
        .unwrap();

    let id2 = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id2, "same".to_string(), "two".to_string()).unwrap())
        .unwrap();

    let loaded = storage.get_by_title("same").unwrap().unwrap();
    assert_eq!(loaded.code, "two");
}

#[test]
fn json_storage_delete() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snippets.json");
    let mut storage = JsonStorage::new(path);

    let id = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id, "bye".to_string(), "x".to_string()).unwrap())
        .unwrap();

    let deleted = storage.delete_by_title("bye").unwrap();
    assert!(deleted);

    assert!(storage.get_by_title("bye").unwrap().is_none());
}

#[test]
fn json_storage_next_id_increments() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snippets.json");
    let mut storage = JsonStorage::new(path);

    let id1 = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id1, "a".to_string(), "1".to_string()).unwrap())
        .unwrap();

    let id2 = storage.next_id().unwrap();
    assert_eq!(id2, id1 + 1);
}
