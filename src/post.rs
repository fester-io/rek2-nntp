use super::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub struct Article {
    pub from: String,
    pub subject: String,
    pub body: String,
    pub newsgroups: String, // Add a new field for Newsgroups
}

pub async fn post_to_group(
    connection: &mut AuthenticatedConnection,
    article: &Article,
    newsgroup: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (read_half, write_half) = tokio::io::split(&mut connection.tls_stream);
    let mut reader = tokio::io::BufReader::new(read_half);
    let mut writer = tokio::io::BufWriter::new(write_half);

    // Use article.from directly instead of a hardcoded formatted version.
    let group_command = format!("GROUP {}\r\n", newsgroup);
    println!("Group Command: {}", group_command);
    writer.write_all(group_command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the GROUP response (expected code "211")
    let group_response = crate::utils::wait_for_response(&mut reader, &["211"], 5, 3).await?;
    if !group_response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to set the newsgroup",
        )));
    }

    // Issue the POST command
    let post_command = "POST\r\n";
    writer.write_all(post_command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the POST response (expected code "340")
    let post_response = crate::utils::wait_for_response(&mut reader, &["340"], 5, 3).await?;
    if !post_response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initiate the posting",
        )));
    }

    // Article data to be posted, using article.from directly.
    let article_data = format!(
        "From: {}\r\nNewsgroups: {}\r\nSubject: {}\r\n\r\n{}\r\n.\r\n",
        article.from.trim(),
        article.newsgroups.trim(),
        article.subject.trim(),
        article.body.trim()
    );

    writer.write_all(article_data.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the Article data response (expected code "240" for success or "441" for error)
    let article_response =
        crate::utils::wait_for_response(&mut reader, &["240", "441"], 5, 3).await?;
    if !article_response.starts_with("240") && !article_response.starts_with("441") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to complete the posting",
        )));
    }

    println!("Article data response: {}", article_response);

    Ok(())
}
