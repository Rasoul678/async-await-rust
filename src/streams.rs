use crate::{run, stream_from_iter, StreamExt};

pub fn streams() {
    println!("Streams");
    run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = stream_from_iter(iter);

        let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("{}", value);
        }
    });
}
