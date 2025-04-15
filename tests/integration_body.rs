use rek2_nntp::{authenticate, body, group, quit};
use std::env;

#[tokio::test]
async fn integration_test_body() {
    // Environment variables NNTP_HOST, NNTP_USERNAME, and NNTP_PASSWORD must be set.
    let host = env::var("NNTP_HOST").expect("NNTP_HOST not set");
    let username = env::var("NNTP_USERNAME").expect("NNTP_USERNAME not set");
    let password = env::var("NNTP_PASSWORD").expect("NNTP_PASSWORD not set");

    // Authenticate using the provided credentials.
    let connection = authenticate(&host, &username, &password).await;
    assert!(
        connection.is_ok(),
        "Authentication failed: {:?}",
        connection.err()
    );
    let mut connection = connection.unwrap();

    // Select a newsgroup to ensure the context for the BODY command.
    let group_name = "hispagatos.test"; // Adjust this to a valid group on the server.
    let group_response = group(&mut connection, group_name).await;
    assert!(
        group_response.is_ok(),
        "GROUP command failed: {:?}",
        group_response.err()
    );
    let group_resp = group_response.unwrap();
    println!("GROUP response: {}", group_resp);

    // Parse the GROUP response to obtain the low article number.
    // Expected format: "211 <article_count> <low> <high> <group_name>"
    let parts: Vec<&str> = group_resp.split_whitespace().collect();
    assert!(
        parts.len() >= 4,
        "Invalid GROUP response format: {}",
        group_resp
    );
    let low_article = parts[2];
    println!("Using article number: {}", low_article);

    // Retrieve the article body using the BODY command.
    let body_result = body(&mut connection, low_article).await;
    assert!(
        body_result.is_ok(),
        "BODY command failed: {:?}",
        body_result.err()
    );
    let article_body = body_result.unwrap();
    println!("Integration test (BODY):\n{}", article_body);

    // Close the session gracefully.
    let quit_result = quit(&mut connection).await;
    assert!(
        quit_result.is_ok(),
        "Failed to quit session cleanly: {:?}",
        quit_result.err()
    );
}
