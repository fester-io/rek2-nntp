use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Article {
    pub header: String,
    pub body: String,
}

pub fn read_from_group(
    stream: &mut TcpStream,
    group: &str,
    range: Option<(u32, u32)>,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to select group",
        )));
    }

    let mut articles = Vec::new();
    let (start, end) = range.unwrap_or((1, 10));

    for i in start..=end {
        let article_command = format!("ARTICLE {}\r\n", i);
        stream.write_all(article_command.as_bytes())?;
        stream.flush()?;

        let mut article_response = String::new();
        reader.read_line(&mut article_response)?;

        if article_response.starts_with("220") {
            let mut article = Article {
                header: String::new(),
                body: String::new(),
            };

            let mut is_header = true;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line)?;

                if line == "\r\n" {
                    is_header = false;
                    continue;
                }

                if line == ".\r\n" {
                    break;
                }

                if is_header {
                    article.header.push_str(&line);
                } else {
                    article.body.push_str(&line);
                }
            }

            articles.push(article);
        }
    }

    Ok(articles)
}
