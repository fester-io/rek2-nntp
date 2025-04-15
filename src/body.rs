use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// Retrieves only the article body by sending the BODY command with the specified article identifier.
/// The expected server response should start with "222". The function then reads the response until the
/// termination line ".\r\n" is encountered and returns the article body.
pub async fn body(
    connection: &mut AuthenticatedConnection,
    identifier: &str,
) -> Result<String, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Construct and send the BODY command.
    let command = format!("BODY {}\r\n", identifier);
    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the expected response using the helper function.
    let response = crate::utils::wait_for_response(&mut reader, &["222"], 5, 3).await?;
    if !response.starts_with("222") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "BODY command failed: unexpected response",
        )));
    }

    let mut body = String::new();
    // Read lines until the termination line is encountered.
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line == ".\r\n" {
            break;
        }
        body.push_str(&line);
    }

    Ok(body)
}
