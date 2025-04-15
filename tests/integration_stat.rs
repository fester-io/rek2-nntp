use rek2_nntp::{authenticate, group, quit, stat};
use std::env;

#[tokio::test]
async fn integration_test_stat() {
    // Environment variables NNTP_HOST, NNTP_USERNAME, and NNTP_PASSWORD must be set.
    let host = env::var("NNTP_HOST").expect("NNTP_HOST not set");
    let username = env::var("NNTP_USERNAME").expect("NNTP_USERNAME not set");
    let password = env::var("NNTP_PASSWORD").expect("NNTP_PASSWORD not set");

    // Authenticate with the NNTP server.
    let connection = authenticate(&host, &username, &password).await;
    assert!(
        connection.is_ok(),
        "Authentication failed: {:?}",
        connection.err()
    );
    let mut connection = connection.unwrap();

    // Select a newsgroup to set the correct context.
    let group_name = "hispagatos.test"; // Adjust to a valid newsgroup.
    let group_response = group(&mut connection, group_name).await;
    assert!(
        group_response.is_ok(),
        "GROUP command failed: {:?}",
        group_response.err()
    );
    let group_resp = group_response.unwrap();
    println!("GROUP response: {}", group_resp);

    // Parse the GROUP response to extract the low article number.
    // Expected format: "211 <article_count> <low> <high> <group_name>"
    let parts: Vec<&str> = group_resp.split_whitespace().collect();
    assert!(
        parts.len() >= 4,
        "Invalid GROUP response format: {}",
        group_resp
    );
    let low_article = parts[2];
    println!("Using article number: {}", low_article);

    // Send the STAT command with the parsed article number.
    let stat_result = stat(&mut connection, low_article).await;
    assert!(
        stat_result.is_ok(),
        "STAT command failed: {:?}",
        stat_result.err()
    );
    let message_id = stat_result.unwrap();
    println!("STAT result (message ID): {}", message_id);

    // Gracefully close the session.
    let quit_result = quit(&mut connection).await;
    assert!(
        quit_result.is_ok(),
        "Failed to quit session cleanly: {:?}",
        quit_result.err()
    );
}
