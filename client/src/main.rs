use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut tasks = vec![];

        // Simulate 100 users
        for _ in 0..100 {
            let task = tokio::spawn(async {
                // Connect to the server
                let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

                // Data to send to the server
                let data = "Hello, server!";

                // Write data to the server
                stream.write_all(data.as_bytes()).await?;

                println!("Sent: {}", data);

                // Buffer to store received data
                let mut buffer = [0; 1024];
                let mut received = Vec::new();

                // Read the response from the server
                let bytes_read = stream.read(&mut buffer).await?;
                received.extend_from_slice(&buffer[..bytes_read]);

                // Process and print the received data
                if let Ok(response) = std::str::from_utf8(&received) {
                    println!("Received: {}", response);
                } else {
                    println!("Received data is not valid UTF-8");
                }

                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            task.await??; // Use `??` to propagate errors
        }

        // Delay execution by 1 second
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
