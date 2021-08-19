use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    // Making a TCP echo server, waiting for client to connect
    // 1. TCP listener
    // await is rust keyword, tells the rust compiler to suspend the function until the future resolves and is ready and has an item that is ready to do some processing on.
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        //2. calling the accept method on our tcp listener
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }

                }
            }
        });
    }

}