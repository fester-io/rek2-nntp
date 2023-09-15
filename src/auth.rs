use native_tls::{TlsConnector, TlsStream};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

// Define a new trait that combines BufRead and Write
pub trait StreamReadWrite: BufRead + Write {}

// Implement StreamReadWrite for all types that implement BufRead and Write
impl<T: BufRead + Write> StreamReadWrite for T {}

pub enum AuthType {
    Plain,
    Ssl,
}

pub fn authenticate(
    auth_type: AuthType,
    mut stream: TcpStream,
    username: &str,
    password: &str,
) -> Result<(), &'static str> {
    let mut reader = BufReader::new(&stream);
    match auth_type {
        AuthType::Plain => {
            write!(stream, "AUTHINFO USER {}\r\n", username).unwrap();
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();
            if !response.starts_with("381") {
                return Err("Failed to authenticate with username");
            }

            write!(stream, "AUTHINFO PASS {}\r\n", password).unwrap();
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();
            if !response.starts_with("281") {
                return Err("Failed to authenticate with password");
            }
        }
        AuthType::Ssl => {
            let connector = TlsConnector::new().unwrap();
            let mut tls_stream = connector.connect("localhost", stream).unwrap();
            let mut tls_reader = BufReader::new(&tls_stream);

            write!(tls_stream, "AUTHINFO USER {}\r\n", username).unwrap();
            let mut response = String::new();
            tls_reader.read_line(&mut response).unwrap();
            if !response.starts_with("381") {
                return Err("Failed to authenticate with username");
            }

            write!(tls_stream, "AUTHINFO PASS {}\r\n", password).unwrap();
            let mut response = String::new();
            tls_reader.read_line(&mut response).unwrap();
            if !response.starts_with("281") {
                return Err("Failed to authenticate with password");
            }
        }
    }
    Ok(())
}
