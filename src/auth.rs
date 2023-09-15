use native_tls::TlsConnector;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub enum AuthType {
    Plain,
    SslTls,
}

pub fn authenticate(
    server: &str,
    port: u16,
    username: &str,
    password: &str,
    auth_type: AuthType,
) -> Result<(), Box<dyn Error>> {
    let addr = format!("{}:{}", server, port);

    // Establishing the connection based on the auth_type
    let mut stream: Box<dyn Write + BufRead> = match auth_type {
        AuthType::Plain => {
            let tcp_stream = TcpStream::connect(addr)?;
            Box::new(BufReader::new(tcp_stream))
        }
        AuthType::SslTls => {
            let tcp_stream = TcpStream::connect(addr)?;
            let connector = TlsConnector::new()?;
            let tls_stream = connector.connect(server, tcp_stream)?;
            Box::new(BufReader::new(tls_stream))
        }
    };

    // Send the AUTHINFO USER command
    let user_command = format!("AUTHINFO USER {}\\r\\n", username);
    stream.write_all(user_command.as_bytes())?;
    stream.flush()?;

    // Read server response for AUTHINFO USER
    let mut user_response = String::new();
    stream.read_line(&mut user_response)?;
    if !user_response.starts_with("381") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "AUTHINFO USER failed",
        )));
    }

    // Send the AUTHINFO PASS command
    let pass_command = format!("AUTHINFO PASS {}\\r\\n", password);
    stream.write_all(pass_command.as_bytes())?;
    stream.flush()?;

    // Read server response for AUTHINFO PASS
    let mut pass_response = String::new();
    stream.read_line(&mut pass_response)?;
    if !pass_response.starts_with("281") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "AUTHINFO PASS failed",
        )));
    }

    Ok(())
}
