use crate::{channel, pin, run, sleep, spawn_task, ReceiverStream, Stream, StreamExt};
use std::time::Duration;

pub fn composing_streams() {
    println!("Composing Streams");

    run(async {
        // let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval #{count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        let merged = messages.merge(intervals).take(100);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(m) => println!("Message: {m}"),
                Err(e) => println!("Error: {e}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = channel();

    spawn_task(async move {
        let chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        for (idx, c) in chars.into_iter().enumerate() {
            let time_to_sleep = if idx % 2 == 0 { 100 } else { 300 };

            sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("'{c}'")) {
                eprintln!("Cannot send message '{c}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u64> {
    let (tx, rx) = channel();

    spawn_task(async move {
        let mut count = 0;

        loop {
            sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
