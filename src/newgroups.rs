//! Module for retrieving new newsgroups via the NEWGROUPS command.
//!
//! Types:
//! - `Newsgroup`: represents a newly created newsgroup (name, range, status).
//!
//! Functions:
//! - `newgroups()`: sends `NEWGROUPS <date> <time> [dist]` and parses the list.
use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// Data structure representing a new newsgroup.
#[derive(Debug)]
pub struct Newsgroup {
    pub name: String,
    pub high: u32,
    pub low: u32,
    pub status: String,
}

/// Sends the NEWGROUPS command to retrieve new newsgroups created since the specified date and time.
/// The date and time should be provided in the server's expected format. An optional distribution parameter
/// may be specified. The function waits for a response code "231" and then parses the subsequent list entries.
pub async fn newgroups(
    connection: &mut AuthenticatedConnection,
    date: &str,
    time: &str,
    distribution: Option<&str>,
) -> Result<Vec<Newsgroup>, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Construct the NEWGROUPS command.
    let command = if let Some(dist) = distribution {
        format!("NEWGROUPS {} {} {}\r\n", date, time, dist)
    } else {
        format!("NEWGROUPS {} {}\r\n", date, time)
    };

    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the expected response ("231 ...") using the timeout helper.
    let response = crate::utils::wait_for_response(&mut reader, &["231"], 5, 3).await?;
    if !response.starts_with("231") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to retrieve newsgroups: unexpected response",
        )));
    }

    let mut newsgroups = Vec::new();

    // Read and parse each line until a line containing ".\r\n" is received.
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line == ".\r\n" {
            break;
        }
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 4 {
            let ng = Newsgroup {
                name: parts[0].to_string(),
                high: parts[1].parse().unwrap_or(0),
                low: parts[2].parse().unwrap_or(0),
                status: parts[3].to_string(),
            };
            newsgroups.push(ng);
        }
    }

    Ok(newsgroups)
}
