use snippets_app::model::Snippet;
use snippets_app::storage::sqlite::SqliteStorage;
use snippets_app::storage::SnippetStorage;

use tempfile::tempdir;

#[test]
fn sqlite_storage_save_get_list_delete() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("snippets.db");

    let mut storage = SqliteStorage::new(&db).unwrap();

    let id = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id, "t".to_string(), "code".to_string()).unwrap())
        .unwrap();

    let got = storage.get_by_title("t").unwrap().unwrap();
    assert_eq!(got.code, "code");

    let list = storage.list().unwrap();
    assert_eq!(list.len(), 1);

    let deleted = storage.delete_by_title("t").unwrap();
    assert!(deleted);

    assert!(storage.list().unwrap().is_empty());
}

#[test]
fn sqlite_upsert_by_title_updates_code() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("snippets.db");

    let mut storage = SqliteStorage::new(&db).unwrap();

    let id1 = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id1, "same".to_string(), "one".to_string()).unwrap())
        .unwrap();

    let id2 = storage.next_id().unwrap();
    storage
        .save(Snippet::new(id2, "same".to_string(), "two".to_string()).unwrap())
        .unwrap();

    let got = storage.get_by_title("same").unwrap().unwrap();
    assert_eq!(got.code, "two");
}
