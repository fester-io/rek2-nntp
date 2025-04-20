use rek2_nntp::{authenticate, fetch_xover_range, group, quit};
use std::env;

#[tokio::test]
async fn integration_test_fetch_xover_range() {
    // Load credentials
    let host = env::var("NNTP_HOST").expect("NNTP_HOST not set");
    let username = env::var("NNTP_USERNAME").expect("NNTP_USERNAME not set");
    let password = env::var("NNTP_PASSWORD").expect("NNTP_PASSWORD not set");

    // Connect
    let connection = authenticate(&host, &username, &password).await;
    assert!(connection.is_ok(), "Auth failed: {:?}", connection.err());
    let mut connection = connection.unwrap();

    // GROUP
    let group_name = "hispagatos.test";
    let group_resp = group(&mut connection, group_name).await.unwrap();
    let parts: Vec<&str> = group_resp.split_whitespace().collect();
    let low: u32 = parts[2].parse().unwrap();
    let high: u32 = parts[3].parse().unwrap();

    // Test the actual API
    let range = Some((low, (low + 4).min(high)));
    let result = fetch_xover_range(&mut connection, group_name, range).await;
    assert!(
        result.is_ok(),
        "fetch_xover_range failed: {:?}",
        result.err()
    );

    let articles = result.unwrap();
    assert!(!articles.is_empty(), "No articles returned from XOVER");

    for art in &articles {
        println!(
            "Article {} — From: {} — Subject: {} — Date: {}",
            art.article_id, art.from, art.subject, art.date
        );
    }

    // Clean exit
    let _ = quit(&mut connection).await;
}
