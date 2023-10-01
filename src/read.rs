use super::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub struct Article {
    pub header: String,
    pub body: String,
}

pub async fn read_from_group(
    connection: &mut AuthenticatedConnection,
    group: &str,
    range: Option<(u32, u32)>,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    let group_command = format!("GROUP {}\r\n", group);
    writer.write_all(group_command.as_bytes()).await?;
    writer.flush().await?;

    let mut response = String::new();
    reader.read_line(&mut response).await?;

    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to select group",
        )));
    }

    let mut articles = Vec::new();
    let (start, end) = range.unwrap_or((1, 10));

    for i in start..=end {
        let article_command = format!("ARTICLE {}\r\n", i);
        writer.write_all(article_command.as_bytes()).await?;
        writer.flush().await?;

        let mut article_response = String::new();
        reader.read_line(&mut article_response).await?;

        if article_response.starts_with("220") {
            let mut article = Article {
                header: String::new(),
                body: String::new(),
            };

            let mut is_header = true;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).await?;

                if line == "\r\n" {
                    is_header = false;
                    continue;
                }

                if line == ".\r\n" {
                    break;
                }

                if is_header {
                    article.header.push_str(&line);
                } else {
                    article.body.push_str(&line);
                }
            }

            articles.push(article);
        }
    }

    Ok(articles)
}
