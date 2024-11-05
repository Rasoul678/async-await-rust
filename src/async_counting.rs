use crate::{run, sleep, spawn_task};
use std::time::Duration;

pub fn async_counting() {
    run(async {
        let join_handle = spawn_task(async {
            for i in 0..10 {
                println!("hi number {i} from the first task!");
                sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            sleep(Duration::from_millis(500)).await;
        }

        join_handle.await.unwrap();
    });
}
