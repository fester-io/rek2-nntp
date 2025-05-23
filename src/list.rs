//! Module for listing available newsgroups via the LIST command.
//!
//! Types:
//! - `Newsgroup`: holds `name`, `low`, `high`, and `status` for each group.
//!
//! Functions:
//! - `list_newsgroups()`: sends `LIST` and returns a `Vec<Newsgroup>`.
use super::auth::AuthenticatedConnection;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub struct Newsgroup {
    pub name: String,
    pub high: u64,
    pub low: u64,
    pub status: String,
}

pub async fn list_newsgroups(
    connection: &mut AuthenticatedConnection,
) -> Result<Vec<Newsgroup>, Box<dyn std::error::Error>> {
    let (read_half, write_half) = tokio::io::split(&mut connection.tls_stream);
    let mut reader = tokio::io::BufReader::new(read_half);
    let mut writer = tokio::io::BufWriter::new(write_half);

    let list_command = "LIST\r\n";
    writer.write_all(list_command.as_bytes()).await?;
    writer.flush().await?;

    let response = crate::utils::wait_for_response(&mut reader, &["215"], 5, 3).await?;
    if !response.starts_with("215") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute LIST command",
        )));
    }

    let mut newsgroups = Vec::new();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        if line == ".\r\n" {
            break;
        }

        let parts: Vec<&str> = line.trim().split(" ").collect();
        if parts.len() >= 4 {
            let newsgroup = Newsgroup {
                name: parts[0].to_string(),
                high: parts[1].parse()?,
                low: parts[2].parse()?,
                status: parts[3].to_string(),
            };
            newsgroups.push(newsgroup);
        }
    }

    Ok(newsgroups)
}
