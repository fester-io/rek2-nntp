use rek2_nntp::{authenticate, list_newsgroups};
use std::env;
use tokio;

#[tokio::test]
async fn integration_test_auth_list() {
    // Retrieve credentials and host from environment variables.
    // Set these in your environment (or in a .env file loaded by your test runner):
    // NNTP_HOST, NNTP_USERNAME, NNTP_PASSWORD
    let host = env::var("NNTP_HOST").expect("NNTP_HOST not set");
    let username = env::var("NNTP_USERNAME").expect("NNTP_USERNAME not set");
    let password = env::var("NNTP_PASSWORD").expect("NNTP_PASSWORD not set");

    // Attempt to authenticate
    let connection = authenticate(&host, &username, &password).await;
    assert!(
        connection.is_ok(),
        "Authentication failed: {:?}",
        connection.err()
    );
    let mut connection = connection.unwrap();

    // Test listing newsgroups using the authenticated connection
    let groups_result = list_newsgroups(&mut connection).await;
    assert!(
        groups_result.is_ok(),
        "Failed to list newsgroups: {:?}",
        groups_result.err()
    );
    let groups = groups_result.unwrap();

    println!("Integration test: Retrieved {} newsgroups.", groups.len());
    // Optionally, list some newsgroups to the output for further verification.
    for group in groups.iter().take(5) {
        println!(
            "Newsgroup: {} (range: {}-{}, status: {})",
            group.name, group.low, group.high, group.status
        );
    }
}
