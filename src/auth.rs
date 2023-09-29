use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_native_tls::TlsConnector;

pub enum AuthType {
    Plain,
    SSL,
}

pub async fn authenticate(
    host: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    let connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let address: String = format!("{}:563", host);
    let stream = TcpStream::connect(address).await?;
    let tls_stream = connector.connect(host, stream).await?;

    let mut reader = BufReader::new(tls_stream);

    let user_command = format!("AUTHINFO USER {}\r\n", username);
    reader.get_mut().write_all(user_command.as_bytes()).await?;
    reader.get_mut().flush().await?;

    let mut attempts = 0;
    let max_attempts = 3;
    let mut response = String::new();

    while attempts < max_attempts {
        response.clear();
        reader.read_line(&mut response).await?;
        if response.starts_with("381") {
            break;
        }
        attempts += 1;
    }

    if response.starts_with("381") {
        let pass_command = format!("AUTHINFO PASS {}\r\n", password);
        reader.get_mut().write_all(pass_command.as_bytes()).await?;
        reader.get_mut().flush().await?;

        response.clear();
        reader.read_line(&mut response).await?;
        println!("Response after AUTHINFO PASS: {}", response);

        if response.starts_with("281") {
            // Authentication successful
        } else {
            // Authentication failed
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Authentication failed",
            )));
        }
    } else {
        // Unexpected response to AUTHINFO USER
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unexpected response to AUTHINFO USER",
        )));
    }

    Ok(())
}
