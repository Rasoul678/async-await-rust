use std::time::Duration;

use crate::{race, run, sleep, Either, Future};

pub fn timeout() {
    run(async {
        let slow = async {
            sleep(Duration::from_secs(5)).await;
            "I finished!"
        };

        match timeout_async(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout_async<F: Future>(future: F, max_time: Duration) -> Result<F::Output, Duration> {
    match race(future, sleep(max_time)).await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(max_time),
    }
}
