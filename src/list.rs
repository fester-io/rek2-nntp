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

    let mut newsgroups = Vec::with_capacity(4_096); // tweak to your needs
    let mut buf = Vec::<u8>::with_capacity(128); // one buffer for all lines

    loop {
        buf.clear(); // re-use the allocation
        let n = reader.read_until(b'\n', &mut buf).await?;
        if n == 0 {
            break; // EOF
        }

        // NNTP terminates a multi-line response with a single “.” line
        if buf == b".\r\n" || buf == b".\n" {
            break;
        }

        // Trim trailing LF / CR
        if let Some(b'\n') = buf.last() {
            buf.pop();
        }
        if let Some(b'\r') = buf.last() {
            buf.pop();
        }

        // UTF-8 *lossy* conversion (Cow: only allocates if the line is not valid UTF-8)
        let line = String::from_utf8_lossy(&buf);

        // Split the iterator directly; no intermediate Vec<&str>
        let mut fields = line.split_whitespace();
        let (Some(name), Some(high), Some(low), Some(status)) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            continue; // malformed line
        };

        newsgroups.push(Newsgroup {
            name: name.to_owned(), // still one allocation per field
            high: high.parse()?,   // u64
            low: low.parse()?,     // u64
            status: status.to_owned(),
        });
    }

    Ok(newsgroups)
}
