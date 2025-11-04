#[tokio::test]
async fn test_hello_and_create_item() {
    // spawn server on background task (bind to 127.0.0.1:0 for random port)
    // For brevity, you can test handlers directly by calling the functions,
    // but integration tests should hit the HTTP server in a real test.
    // Example: call hello handler directly:
    use hello_axum::handlers::hello;
    use axum::extract::Path;
    let resp = hello(Path("Sivaji".to_string())).await;
    // resp is axum::Json...
    assert!(resp.0.get("message").unwrap().as_str().unwrap().contains("Sivaji"));
}
