use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn cli_create_and_read_via_stdin_json() {
    let dir = tempdir().unwrap();
    let json = dir.path().join("snippets.json");
    let log = dir.path().join("log.txt");

    // create
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--name")
        .arg("hello")
        .write_stdin("println!(\"hi\");")
        .assert()
        .success()
        .stdout(predicate::str::contains("saved snippet"));

    // read
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--read")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("println!(\"hi\");"));
}

#[test]
fn cli_delete_json() {
    let dir = tempdir().unwrap();
    let json = dir.path().join("snippets.json");
    let log = dir.path().join("log.txt");

    // create
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--name")
        .arg("x")
        .write_stdin("1")
        .assert()
        .success();

    // delete
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--delete")
        .arg("x")
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    // read after delete
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--read")
        .arg("x")
        .assert()
        .success()
        .stdout(predicate::str::contains("Not found"));
}
