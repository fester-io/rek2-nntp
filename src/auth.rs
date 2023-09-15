use native_tls::TlsConnector;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

pub enum AuthType {
    Plain,
    SslTls,
}

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

pub fn authenticate(
    server: &str,
    port: u16,
    username: &str,
    password: &str,
    auth_type: AuthType,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", server, port);

    match auth_type {
        AuthType::Plain => {
            let mut stream: Box<dyn Read + Write> = Box::new(TcpStream::connect(addr)?);
            common_authenticate(&mut stream, username, password)
        }
        AuthType::SslTls => {
            let tcp_stream = TcpStream::connect(addr)?;
            let connector = TlsConnector::new().map_err(|e| e.to_string())?;
            let tls_stream = connector
                .connect(server, tcp_stream)
                .map_err(|e| e.to_string())?;
            let mut stream: Box<dyn Read + Write> = Box::new(tls_stream);
            common_authenticate(&mut stream, username, password)
        }
    }
}

pub fn common_authenticate(
    stream: &mut (dyn Read + Write),
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);

    let user_command = format!("AUTHINFO USER {}\r\n", username);
    stream.write_all(user_command.as_bytes())?;
    stream.flush()?;

    let mut user_response = String::new();
    reader.read_line(&mut user_response)?;

    let pass_command = format!("AUTHINFO PASS {}\r\n", password);
    stream.write_all(pass_command.as_bytes())?;
    stream.flush()?;

    let mut pass_response = String::new();
    reader.read_line(&mut pass_response)?;

    Ok(())
}
