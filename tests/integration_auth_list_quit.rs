use rek2_nntp::{authenticate, list_newsgroups, quit};
use std::env;

#[tokio::test]
async fn integration_test_auth_list_and_quit() {
    // Environment variables NNTP_HOST, NNTP_USERNAME, NNTP_PASSWORD must be set.
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

    // Retrieve a list of newsgroups.
    let groups_result = list_newsgroups(&mut connection).await;
    assert!(
        groups_result.is_ok(),
        "Failed to list newsgroups: {:?}",
        groups_result.err()
    );
    let groups = groups_result.unwrap();

    println!("Integration test: Retrieved {} newsgroups.", groups.len());
    // Display a subset of newsgroups for verification.
    for group in groups.iter().take(5) {
        println!(
            "Newsgroup: {} (range: {}-{}, status: {})",
            group.name, group.low, group.high, group.status
        );
    }

    // Send the QUIT command to gracefully close the session.
    let quit_result = quit(&mut connection).await;
    assert!(
        quit_result.is_ok(),
        "Failed to quit session: {:?}",
        quit_result.err()
    );
}
