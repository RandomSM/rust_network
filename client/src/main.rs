use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

// repete 100 fois chaque seconde l'envoie de hello world au serveur puis print le réponse obtenue
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut tasks = vec![];

        for _ in 0..100 {
            let task = tokio::spawn(async {
                let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

                let data = "Hello, server!";

                stream.write_all(data.as_bytes()).await?;

                println!("Sent: {}", data);

                let mut buffer = [0; 1024];
                let mut received = Vec::new();

                let bytes_read = stream.read(&mut buffer).await?;
                received.extend_from_slice(&buffer[..bytes_read]);

                if let Ok(response) = std::str::from_utf8(&received) {
                    println!("Received: {}", response);
                    println!("Validation: Message received by the server");
                } else {
                    println!("Received data is not valid UTF-8");
                }

                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            });

            tasks.push(task);
        }

        for task in tasks {
            task.await??;
        }   

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
