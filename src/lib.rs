mod async_counting;
mod async_with_join;
mod get_page_title;
mod async_message_passing;

pub use async_counting::async_counting;
pub use async_with_join::async_with_join;
pub use get_page_title::get_page_title;
pub use async_message_passing::async_message_passing;

use std::{future::Future, pin::pin};

use futures::future;

pub use futures::{
    future::{join, join3, join_all, Either},
    join,
};

pub use tokio::{
    fs::read_to_string,
    runtime::Runtime,
    sync::mpsc::{
        unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
    },
    task::{spawn as spawn_task, yield_now, JoinHandle},
    time::{interval, sleep},
};

pub use tokio_stream::{
    iter as stream_from_iter,
    wrappers::{IntervalStream, UnboundedReceiverStream as ReceiverStream},
    Stream, StreamExt,
};

pub fn run<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

pub async fn race<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    let f1 = pin!(f1);
    let f2 = pin!(f2);

    match future::select(f1, f2).await {
        Either::Left((a, _f1)) => Either::Left(a),
        Either::Right((b, _f2)) => Either::Right(b),
    }
}

pub struct Response(reqwest::Response);

impl Response {
    pub async fn text(self) -> String {
        self.0.text().await.unwrap()
    }
}

pub async fn get(url: &str) -> Response {
    Response(reqwest::get(url).await.unwrap())
}

pub struct Html {
    inner: scraper::Html,
}

impl Html {
    pub fn parse(source: &str) -> Html {
        Html {
            inner: scraper::Html::parse_document(source),
        }
    }
    pub fn select_first<'a>(&'a self, selector: &'a str) -> Option<scraper::ElementRef<'a>> {
        let selector = scraper::Selector::parse(selector).unwrap();

        self.inner.select(&selector).nth(0)
    }
}
