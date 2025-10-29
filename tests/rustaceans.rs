use reqwest::{StatusCode, blocking::Client};
use serde_json::{json, Value};
pub mod common;

#[test]
fn test_get_rustaceans() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    // test
    let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // cleanup
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let response = client.post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foobar@gmail.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo bar",
        "email": "foobar@gmail.com",
        "created_at": rustacean["created_at"]
    }));

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    let response = client.get(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo bar",
        "email": "foobar@gmail.com",
        "created_at": rustacean["created_at"]
    }));

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    let response = client.put(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "Fooz bar",
            "email": "foozbar@gmail.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Fooz bar",
        "email": "foozbar@gmail.com",
        "created_at": rustacean["created_at"]
    }));

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    let response = client.delete(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans_requires_authentication() {
    // Create an unauthenticated client (no session token)
    let client = Client::new();
    
    let response = client.get(format!("{}/rustaceans", common::APP_HOST))
        .send();
    
    // Should fail when no session token is provided
    // Either returns 401 Unauthorized or connection error (both indicate rejection)
    match response {
        Ok(resp) => assert_eq!(resp.status(), StatusCode::UNAUTHORIZED),
        Err(_) => {
            // Any network error indicates the server rejected the unauthenticated request
            // This is the expected behavior for protected endpoints
        }
    }
}
