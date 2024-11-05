use crate::{get, race, run, Either, Html};
use std::env::args;

pub fn get_page_title() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <url>");
        return;
    }

    run(async {
        let title_future_1 = page_title_for(&args[1]);
        let title_future_2 = page_title_for(&args[2]);

        let (url, maybe_title) = match race(title_future_1, title_future_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    });
}

async fn page_title_for(url: &str) -> (&str, Option<String>) {
    let text = get(url).await.text().await;

    let title = Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}
