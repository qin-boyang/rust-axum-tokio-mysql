const BASE_URL: &str = "http://127.0.0.1:3000";

#[tokio::test]
async fn test_root() {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/", BASE_URL))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200, "Expected status OK");
    assert_eq!(response.text().await.unwrap(), "Hello, World!");
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct User {
    id: i32,
    name: String
}

#[tokio::test]
async fn test_fetch_user_by_id() {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/user/123", BASE_URL))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200, "Expected status OK");
    assert_eq!(response.json::<User>().await.unwrap(), User { id: 123, name: "Bob".to_string() })
}
