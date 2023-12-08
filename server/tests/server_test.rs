use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_client_integration() {

    // lance le test
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (mut stream, _) = listener.accept().await.unwrap();

    // buffer les donner qui vont etre recu
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    // valide les donner recu
    let received = &buffer[..bytes_read];
    assert_eq!(received, "Hello, server!".as_bytes());

    // print les donner recu
    stream.write_all(received).await.unwrap();
}
