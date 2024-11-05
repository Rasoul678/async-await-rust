use crate::{channel, join_all, pin, run, sleep, Future};
use std::{pin::Pin, time::Duration};

pub fn async_pin_macro() {
    run(async {
        let (tx, mut rx) = channel::<String>();
        let tx1 = tx.clone();

        let tx_future = pin!(async move {
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
        });

        let rx_future = pin!(async {
            while let Some(message) = rx.recv().await {
                println!("received '{message}'");
            }
        });

        let tx_future1 = pin!(async move {
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
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx_future, tx_future1, rx_future];

        join_all(futures).await;
    });
}
