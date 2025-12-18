use assert_cmd::Command;
use httpmock::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn cli_download_creates_snippet() {
    let server = MockServer::start();

    let body = "downloaded snippet body";
    let m = server.mock(|when, then| {
        when.method(GET).path("/raw");
        then.status(200).body(body);
    });

    let dir = tempdir().unwrap();
    let json = dir.path().join("snippets.json");
    let log = dir.path().join("log.txt");
    let url = format!("{}/raw", server.base_url());

    // create with download
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--name")
        .arg("from_url")
        .arg("--download")
        .arg(&url)
        .assert()
        .success()
        .stdout(predicate::str::contains("saved snippet"));

    m.assert();

    // read and check content
    Command::cargo_bin("snippets-app")
        .unwrap()
        .env("SNIPPETS_APP_STORAGE", format!("JSON:{}", json.display()))
        .env("SNIPPETS_APP_LOG_LEVEL", "info")
        .env("SNIPPETS_APP_LOG_PATH", &log)
        .arg("--read")
        .arg("from_url")
        .assert()
        .success()
        .stdout(predicate::str::contains(body));
}
