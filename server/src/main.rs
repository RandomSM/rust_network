use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("Server error: {}", err);
    }
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream).await {
                eprintln!("Client error: {}", err);
            }
        });
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    loop {
        let bytes = stream.read(&mut buffer).await?;
        if bytes == 0 {
            break;
        }

        let received = &buffer[..bytes];
        println!("Received data: {:?}", received);

        stream.write_all(received).await?;
        stream.flush().await?;
    }

    Ok(())
}
