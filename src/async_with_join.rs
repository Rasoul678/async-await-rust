use std::time::Duration;

use crate::{join, run, sleep};

pub fn async_with_join() {
    run(async {
        let future1 = async {
            for i in 0..10 {
                println!("hi number {i} from the first task!");
                sleep(Duration::from_millis(500)).await;
            }
        };

        let future2 = async {
            for i in 0..5 {
                println!("hi number {i} from the second task!");
                sleep(Duration::from_millis(500)).await;
            }
        };

        join(future1, future2).await;
    });
}
