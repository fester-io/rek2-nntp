//! Module for ending an NNTP session via the QUIT command.
//!
//! Functions:
//! - `quit()`: sends `QUIT` and waits for the "205" response.
use crate::auth::AuthenticatedConnection;
use tokio::io::{split, AsyncWriteExt, BufReader, BufWriter};

pub async fn quit(
    connection: &mut AuthenticatedConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    let quit_command = "QUIT\r\n";
    writer.write_all(quit_command.as_bytes()).await?;
    writer.flush().await?;

    let response = crate::utils::wait_for_response(&mut reader, &["205"], 5, 3).await?;
    if !response.starts_with("205") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to quit session",
        )));
    }

    Ok(())
}
