use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// simule la connection d'un client, envoie une data
// l'ecris et check le reponsse du serveur
#[tokio::test]
async fn test_server_integration() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let data = "Hello, server!";

    stream.write_all(data.as_bytes()).await.unwrap();

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    let received = &buffer[..bytes_read];
    assert_eq!(received, data.as_bytes());
}
