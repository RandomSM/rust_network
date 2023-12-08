use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    if let Err(err) = run_server().await {
        eprintln!("Server Error : {}", err);
    }
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Écoute les connexions entrantes sur l'adresse 127.0.0.1:8080 (localhost)
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Boucle permettant d'accepter les connexions entrantes et de gérer chaque client dans un thread séparé
    while let Ok((stream, _)) = listener.accept().await {
        // Crée un thread pour gérer chaque client de manière asynchrone
        tokio::spawn(async move {
            // Gère le client et capture toute erreur
            if let Err(err) = handle_client(stream).await {
                eprintln!("Client Error : {}", err);
            }
        });
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Initialise un tampon pour stocker les données reçues
    let mut buffer = [0; 1024];

    // Boucle pour lire les données du client et répondre à ses messages
    loop {
        // Lit les données envoyées par le client dans le tampon
        let bytes = stream.read(&mut buffer).await?;
        if bytes == 0 {
            break;
        }

        // Récupère les données reçues dans un tableaux en bytes
        let received = &buffer[..bytes];
        //convertie le message en string
        let bytes_to_string = std::str::from_utf8(&received);

        println!("Data received : {:?}", bytes_to_string);

        // Capture le temps avant l'envoi de la réponse
        let send_time = Instant::now();

        // Envoie la réponse au client
        stream.write_all(received).await?;
        stream.flush().await?;

        // Calcule le temps de latence (ping) en mesurant la durée entre l'envoi et la réception
        let receive_time = Instant::now();
        let ping_time = receive_time.duration_since(send_time);

        println!("Ping : {:?}", ping_time);
    }

    Ok(())
}
