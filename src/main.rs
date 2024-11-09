use lib::{
    async_counting, async_message_passing, async_pin_macro, async_race, async_with_join,
    get_page_title, timeout,
};

fn main() {
    println!("Async and Await");
    // get_page_title();
    // async_counting();
    // async_with_join();
    // async_message_passing();
    // async_pin_macro();
    // async_race();
    timeout();
}
