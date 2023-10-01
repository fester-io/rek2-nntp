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
) -> Result<(), Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Format From: so is accepted by NNTP
    let formatted_from = format!(
        "rek2_nntp@hispagatos.bot (Mastodon: {}))",
        article.from.trim()
    );

    let group_command = format!("GROUP {}\r\n", newsgroup);
    println!("Group Command: {}", group_command);
    writer.write_all(group_command.as_bytes()).await?;
    writer.flush().await?;

    let mut response = String::new();
    let mut attempts = 0;
    let max_attempts = 3;

    // Wait for GROUP response
    while attempts < max_attempts {
        response.clear();
        reader.read_line(&mut response).await?;
        if response.starts_with("211") {
            break;
        }
        attempts += 1;
    }

    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to set the newsgroup",
        )));
    }

    // POST command
    let post_command = "POST\r\n";
    writer.write_all(post_command.as_bytes()).await?;
    writer.flush().await?;

    attempts = 0; // Reset the attempts counter

    // Wait for POST response
    while attempts < max_attempts {
        response.clear();
        reader.read_line(&mut response).await?;
        if response.starts_with("340") {
            break;
        }
        attempts += 1;
    }

    if !response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initiate the posting",
        )));
    }

    // Article data
    let article_data = format!(
        "From: {}\r\nNewsgroups: {}\r\nSubject: {}\r\n\r\n{}\r\n.\r\n",
        formatted_from,
        article.newsgroups.trim(),
        article.subject.trim(),
        article.body.trim()
    );

    writer.write_all(article_data.as_bytes()).await?;
    writer.flush().await?;

    attempts = 0; // Reset the attempts counter

    // Wait for Article data response
    while attempts < max_attempts {
        response.clear();
        reader.read_line(&mut response).await?;
        if response.starts_with("240") || response.starts_with("441") {
            break;
        }
        attempts += 1;
    }

    if !response.starts_with("240") && !response.starts_with("441") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to complete the posting",
        )));
    }

    println!("Article data response: {}", response);

    Ok(())
}
