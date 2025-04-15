use super::auth::AuthenticatedConnection;
use tokio::io::AsyncWriteExt;

pub struct Article {
    pub from: String,
    pub subject: String,
    pub body: String,
    pub newsgroups: String,
}

pub async fn post_to_group(
    connection: &mut AuthenticatedConnection,
    article: &Article,
    newsgroup: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (read_half, write_half) = tokio::io::split(&mut connection.tls_stream);
    let mut reader = tokio::io::BufReader::new(read_half);
    let mut writer = tokio::io::BufWriter::new(write_half);

    let group_command = format!("GROUP {}\r\n", newsgroup);
    println!("Group Command: {}", group_command);
    writer.write_all(group_command.as_bytes()).await?;
    writer.flush().await?;

    let group_response = crate::utils::wait_for_response(&mut reader, &["211"], 5, 3).await?;
    if !group_response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to set the newsgroup",
        )));
    }

    let post_command = "POST\r\n";
    writer.write_all(post_command.as_bytes()).await?;
    writer.flush().await?;

    let post_response = crate::utils::wait_for_response(&mut reader, &["340"], 5, 3).await?;
    if !post_response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initiate the posting",
        )));
    }

    let article_data = format!(
        "From: {}\r\nNewsgroups: {}\r\nSubject: {}\r\n\r\n{}\r\n.\r\n",
        article.from.trim(),
        article.newsgroups.trim(),
        article.subject.trim(),
        article.body.trim()
    );

    writer.write_all(article_data.as_bytes()).await?;
    writer.flush().await?;

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
