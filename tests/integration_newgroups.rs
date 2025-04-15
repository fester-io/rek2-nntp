use rek2_nntp::{authenticate, newgroups, quit};
use std::env;

#[tokio::test]
async fn integration_test_newgroups() {
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

    // Call the NEWGROUPS command with example date and time.
    let date = "20230101"; // Example: January 1, 2023.
    let time = "000000"; // Example: Midnight.
    let newgroups_result = newgroups(&mut connection, date, time, None).await;
    assert!(
        newgroups_result.is_ok(),
        "Failed to retrieve new groups: {:?}",
        newgroups_result.err()
    );
    let groups = newgroups_result.unwrap();

    println!(
        "Integration test (NEWGROUPS): Retrieved {} new groups.",
        groups.len()
    );
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
