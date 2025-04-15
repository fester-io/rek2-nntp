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

// Define a struct to hold the TLS stream.
pub struct AuthenticatedConnection {
    pub tls_stream: tokio_native_tls::TlsStream<TcpStream>,
}

pub async fn authenticate(
    host: &str,
    username: &str,
    password: &str,
) -> Result<AuthenticatedConnection, Box<dyn std::error::Error>> {
    let connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let address: String = format!("{}:563", host);
    let stream = TcpStream::connect(address).await?;
    let tls_stream = connector.connect(host, stream).await?;

    let mut reader = BufReader::new(tls_stream);

    let user_command = format!("AUTHINFO USER {}\r\n", username);
    reader.get_mut().write_all(user_command.as_bytes()).await?;
    reader.get_mut().flush().await?;

    // Wait for the 381 response using our helper function
    let response = crate::utils::wait_for_response(&mut reader, &["381"], 5, 3).await?;

    if response.starts_with("381") {
        let pass_command = format!("AUTHINFO PASS {}\r\n", password);
        reader.get_mut().write_all(pass_command.as_bytes()).await?;
        reader.get_mut().flush().await?;

        // Wait for the 281 response using our helper function
        let response = crate::utils::wait_for_response(&mut reader, &["281"], 5, 3).await?;
        println!("Response after AUTHINFO PASS: {}", response);

        if response.starts_with("281") {
            // Authentication successful
            Ok(AuthenticatedConnection {
                tls_stream: reader.into_inner(),
            })
        } else {
            // Authentication failed
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Authentication failed",
            )))
        }
    } else {
        // Unexpected response to AUTHINFO USER
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unexpected response to AUTHINFO USER",
        )))
    }
}
