use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// Retrieves only the article headers by sending a HEAD command to the server.
/// The article is identified by its message ID or article number.
/// The function waits for a response starting with "221" and then reads the header lines until the termination line ".\r\n" is encountered.
pub async fn head(
    connection: &mut AuthenticatedConnection,
    identifier: &str,
) -> Result<String, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Build and send the HEAD command.
    let command = format!("HEAD {}\r\n", identifier);
    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the initial response, expecting a code starting with "221".
    let response = crate::utils::wait_for_response(&mut reader, &["221"], 5, 3).await?;
    if !response.starts_with("221") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "HEAD command failed with unexpected response",
        )));
    }

    let mut header = String::new();
    // Read header lines until the termination line is encountered.
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line == ".\r\n" {
            break;
        }
        header.push_str(&line);
    }

    Ok(header)
}
