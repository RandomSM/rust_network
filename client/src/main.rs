use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut tasks = vec![];

        // booucle servant a simuler 100 utilisateur
        for _ in 0..100 {
            let task = tokio::spawn(async {
                // connection au serveur
                let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

                // message a envoyer
                let data = "Hello, server!";

                // écriture du essage en bytes au serveur
                stream.write_all(data.as_bytes()).await?;

                //print du message envoyer
                println!("Sent: {}", data);

                // Buffer pour recevoir la validation de la reception du message
                let mut buffer = [0; 1024];
                let mut received = Vec::new();

                // lecture de la réponse du serveur
                let bytes_read = stream.read(&mut buffer).await?;
                received.extend_from_slice(&buffer[..bytes_read]);

                // print de la réponse
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

        // attente de la competion de toute les task
        for task in tasks {
            task.await??; // `??` propagate errors
        }

        // Delay l'execution de 1 sec
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
