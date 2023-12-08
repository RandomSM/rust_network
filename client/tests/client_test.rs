use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_server_integration() {

    // simule la connection d'un client
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    // Data a envoyer
    let data = "Hello, server!";

    // ecriture de la data au serveur
    stream.write_all(data.as_bytes()).await.unwrap();

    // Buffer pour recevoir la réponse du serveur
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    // valide la réponse recu
    let received = &buffer[..bytes_read];
    assert_eq!(received, data.as_bytes());
}
