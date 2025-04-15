use rek2_nntp::{authenticate, group, head, quit};
use std::env;

#[tokio::test]
async fn integration_test_head_group() {
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

    // Select a newsgroup before calling HEAD.
    let group_name = "hispagatos.test"; // Adjust to a valid group available on the server.
    let group_response = group(&mut connection, group_name).await;
    assert!(
        group_response.is_ok(),
        "GROUP command failed: {:?}",
        group_response.err()
    );
    let group_resp = group_response.unwrap();
    println!("GROUP response: {}", group_resp);

    // Parse the GROUP response to extract the low article number.
    // The expected format is "211 <article_count> <low> <high> <group_name>"
    let parts: Vec<&str> = group_resp.split_whitespace().collect();
    assert!(
        parts.len() >= 4,
        "Invalid GROUP response format: {}",
        group_resp
    );
    let low_article = parts[2];
    println!("Using article number: {}", low_article);

    // Call the HEAD command with the parsed article number.
    let header_result = head(&mut connection, low_article).await;
    assert!(
        header_result.is_ok(),
        "HEAD command failed: {:?}",
        header_result.err()
    );
    let header = header_result.unwrap();
    println!("Integration test (HEAD):\n{}", header);

    // Send the QUIT command to gracefully close the session.
    let quit_result = quit(&mut connection).await;
    assert!(
        quit_result.is_ok(),
        "Failed to quit session cleanly: {:?}",
        quit_result.err()
    );
}

