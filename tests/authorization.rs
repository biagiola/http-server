use std::process::Command;
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
pub mod common;

#[test]
fn test_login() {
    // Setup
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();


    let client = Client::new();
    
    // make login request
    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "1234"
        }))
        .send()
        .unwrap();

    // validate http status
    assert_eq!(response.status(), StatusCode::OK);

    // validate body response
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    // make login request with wrong credentials
    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "wrong password"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);


    // Cleanup: Delete the created user
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("delete")
        .arg("test_admin")
        .output()
        .expect("Failed to execute command.");

    assert!(output.status.success());
}
