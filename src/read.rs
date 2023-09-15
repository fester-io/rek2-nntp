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

    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("211") {
        return Err("Failed to select group");
    }

    for i in 1..=10 {
        let article_command = format!("ARTICLE {}\r\n", i);
        stream.write_all(article_command.as_bytes()).unwrap();
        response.clear();
        reader.read_line(&mut response).unwrap();

        if response.starts_with("220") {
            // Read and parse the article here
            let mut article_lines = Vec::new();
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == ".\r\n" {
                    break;
                }
                article_lines.push(line);
            }

            let subject = article_lines
                .iter()
                .find(|&line| line.starts_with("Subject: "))
                .unwrap_or(&"Subject: Unknown".to_string())[9..]
                .trim()
                .to_string();

            let body_start = article_lines
                .iter()
                .position(|line| line.trim().is_empty())
                .unwrap_or(0);

            let body = article_lines[body_start + 1..].join("").trim().to_string();

            articles.push(Article { subject, body });
        }
    }

    Ok(articles)
}
