use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ecoute sur l'adresse 127.0.0.1 port 8080 et attend le message hello world de la part du client
// puis valide le message
#[tokio::test]
async fn test_client_integration() {

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (mut stream, _) = listener.accept().await.unwrap();

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    let received = &buffer[..bytes_read];
    assert_eq!(received, "Hello, server!".as_bytes());

    stream.write_all(received).await.unwrap();
}
