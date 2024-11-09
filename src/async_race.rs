use crate::{race, run, sleep, yield_now};
use std::thread;
use std::time::{Duration, Instant};

pub fn async_race() {
    println!("Async Race");

    run(async {
        let slow = async {
            println!("Slow task starting...");
            sleep(Duration::from_secs(1)).await;
            println!("Slow task done");
        };

        let fast = async {
            println!("Fast task starting...");
            sleep(Duration::from_millis(500)).await;
            println!("Fast task done");
        };

        race(slow, fast).await;

        let a_ = async {
            println!("'a' started.");
            slow_thread("a", 30);
            yield_now().await;
            slow_thread("a", 10);
            slow_thread("a", 20);
            // sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b_ = async {
            println!("'b' started.");
            slow_thread("b", 75);
            slow_thread("b", 10);
            slow_thread("b", 15);
            slow_thread("b", 350);
            yield_now().await;
            // sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        race(a_, b_).await;
        benchmark_yield_sleep().await;
    })
}

fn slow_thread(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn benchmark_yield_sleep() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();

    async {
        for _ in 1..1000 {
            sleep(one_ns).await;
        }
    }
    .await;

    let time = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        time.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            yield_now().await;
        }
    }
    .await;

    let time = Instant::now() - start;

    println!(
        "'yield' version finished after {} seconds.",
        time.as_secs_f32()
    );
}
