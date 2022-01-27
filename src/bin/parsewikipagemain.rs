use aritexpr::wiki;
use async_std::task;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    // let page =
    //     task::block_on(wiki::read_page("http://en.wikipedia.org/wiki/Gravity"));

    let page =
        wiki::read_page_sync("Gravity");

    match page {
        Ok(res) => {
            println!("{}:", res.page_name);
            for link in res.page_link_names {
                println!("  {}", link);
            }
        },
        Err(err) => eprintln!("{}", err),
    };
}
