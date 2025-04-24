//! Module for fetching overview data via the XOVER command.
//!
//! Types:
//! - `Overview`: article metadata (id, subject, from, date, optional message_id/references).
//!
//! Functions:
//! - `fetch_xover_range()`: sends `XOVER <start>-<end>` and returns `Vec<Overview>`.
use crate::auth::AuthenticatedConnection;
use std::error::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub struct Overview {
    pub article_id: u32,
    pub subject: String,
    pub from: String,
    pub date: String,
    pub message_id: Option<String>,
    pub references: Option<String>,
}

pub async fn fetch_xover_range(
    connection: &mut AuthenticatedConnection,
    group: &str,
    range: Option<(u32, u32)>,
) -> Result<Vec<Overview>, Box<dyn Error>> {
    let (read_half, write_half) = split(&mut connection.tls_stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    let group_cmd = format!("GROUP {}\r\n", group);
    writer.write_all(group_cmd.as_bytes()).await?;
    writer.flush().await?;
    let mut group_resp = String::new();
    reader.read_line(&mut group_resp).await?;
    if !group_resp.starts_with("211") {
        return Err(format!("GROUP failed: {}", group_resp.trim()).into());
    }

    let (start, end) = range.unwrap_or((1, 10));
    let xover_cmd = format!("XOVER {}-{}\r\n", start, end);
    writer.write_all(xover_cmd.as_bytes()).await?;
    writer.flush().await?;

    let mut first_line = String::new();
    reader.read_line(&mut first_line).await?;
    if !first_line.starts_with("224") {
        return Err(format!("XOVER failed: {}", first_line.trim()).into());
    }

    let mut overviews = Vec::new();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line.trim() == "." {
            break;
        }

        let fields: Vec<&str> = line.trim_end().split('\t').collect();
        if fields.len() < 4 {
            continue;
        }

        let article_id = fields[0].parse().unwrap_or(0);
        let subject = fields[1].to_string();
        let from = fields[2].to_string();
        let date = fields[3].to_string();

        let message_id = fields.get(4).map(|s| s.trim().to_string());
        let references = fields.get(5).map(|s| s.trim().to_string());

        overviews.push(Overview {
            article_id,
            subject,
            from,
            date,
            message_id,
            references,
        });
    }

    Ok(overviews)
}
