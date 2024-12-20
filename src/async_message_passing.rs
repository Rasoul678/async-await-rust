use crate::{channel, join_all, run, sleep, Future};
use std::{pin::Pin, time::Duration};

pub fn async_message_passing() {
    run(async {
        let (tx, mut rx) = channel::<String>();
        let tx1 = tx.clone();

        // let message = String::from("Hello World!");

        // tx.send(message).unwrap();

        // let received = rx.recv().await.unwrap();

        // println!("{received}");

        let tx_future = async move {
            let message_vec = vec![
                String::from("Hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for message in message_vec {
                tx.send(message).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_future = async {
            while let Some(message) = rx.recv().await {
                println!("received '{message}'");
            }
        };

        let tx_future1 = async move {
            let message_vec = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you!"),
            ];

            for message in message_vec {
                tx1.send(message).unwrap();
                sleep(Duration::from_millis(1500)).await;
            }
        };

        // join!(tx_future, tx_future1, rx_future);

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
            Box::pin(tx_future),
            Box::pin(tx_future1),
            Box::pin(rx_future),
        ];

        join_all(futures).await;
    });
}
