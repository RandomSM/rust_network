use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("Server Error : {}", err);
    }
}

// Écoute les connexions entrantes sur l'adresse 127.0.0.1:8080 en boucle jusqu'a recevoir une data du client
// Crée un thread pour gérer chaque client de manière asynchrone print la data et renvoie une message de validation au client
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream).await {
                eprintln!("Client Error : {}", err);
            }
        });
    }

    Ok(())
}

// Initialise un tampon pour stocker les données reçues lit les données envoyées par le client dans le tampon
// Récupère les données reçues dans un tableaux en bytes, convertie le message en string
// Envoie la réponse au client et calcule le le ping puis le print
async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    let mut buffer = [0; 1024];

    loop {
        let bytes = stream.read(&mut buffer).await?;
        if bytes == 0 {
            break;
        }

        
        let received = &buffer[..bytes];
        let bytes_to_string = std::str::from_utf8(&received);

        println!("Data received : {:?}", bytes_to_string);

        let send_time = Instant::now();

        stream.write_all(received).await?;
        stream.flush().await?;

        let receive_time = Instant::now();
        let ping_time = receive_time.duration_since(send_time);

        println!("Ping : {:?}", ping_time);
    }

    Ok(())
}
