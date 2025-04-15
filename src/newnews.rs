use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// Sends the NEWNEWS command to retrieve new articles posted after the specified date and time.
/// An optional distribution parameter can be provided to limit the results.
/// The command is expected to respond with a line starting with "230", followed by a list of message IDs,
/// one per line, ending with a termination line ".\r\n".
pub async fn newnews(
    connection: &mut AuthenticatedConnection,
    date: &str,
    time: &str,
    distribution: Option<&str>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    let command = if let Some(dist) = distribution {
        format!("NEWNEWS {} {} {}\r\n", date, time, dist)
    } else {
        format!("NEWNEWS {} {}\r\n", date, time)
    };

    writer.write_all(command.as_bytes()).await?;
    writer.flush().await?;

    // Wait for the response code; expected is "230" which indicates that new articles follow.
    let response = crate::utils::wait_for_response(&mut reader, &["230"], 10, 5).await?;
    if !response.starts_with("230") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "NEWNEWS command failed: unexpected response",
        )));
    }

    let mut message_ids = Vec::new();
    // Read and collect each message ID until the termination sequence is reached.
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line == ".\r\n" {
            break;
        }
        let msg_id = line.trim().to_string();
        if !msg_id.is_empty() {
            message_ids.push(msg_id);
        }
    }

    Ok(message_ids)
}
