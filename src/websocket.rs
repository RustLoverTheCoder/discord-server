use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

pub async fn websocket() {
    let listener = TcpListener::bind("localhost:3030").await.unwrap();

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