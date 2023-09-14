use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Article {
    pub subject: String,
    pub body: String,
}

pub fn read_from_group(
    stream: &mut TcpStream,
    group: &str,
    range: Option<(u32, u32)>,
) -> Result<Vec<Article>, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut articles = Vec::new();

    // Select the group first
    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("211") {
        return Err("Failed to select group");
    }

    // Implement reading articles based on the range
    // For simplicity, let's assume we read articles 1 to 10
    for i in 1..=10 {
        let article_command = format!("ARTICLE {}\r\n", i);
        stream.write_all(article_command.as_bytes()).unwrap();
        response.clear();
        reader.read_line(&mut response).unwrap();

        if response.starts_with("220") {
            // Read and parse the article here
            // For now, let's just create a dummy article
            articles.push(Article {
                subject: format!("Subject {}", i),
                body: format!("Body {}", i),
            });
        }
    }

    Ok(articles)
}
