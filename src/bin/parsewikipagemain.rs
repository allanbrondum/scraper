use aritexpr::wiki;
use async_std::task;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    // let page =
    //     task::block_on(wiki::read_page("https://en.wikipedia.org/wiki/Gravity"));

    let page =
        wiki::read_page_sync("https://en.wikipedia.org/wiki/Gravity");

    println!("{:?}", page);

    match page {
        Ok(res) => {
            println!("{}:", res.page);
            for link in res.wiki_links {
                println!("  {}", link);
            }
        },
        Err(err) => eprintln!("{}", err),
    };

    let a : VecDeque<u8> = [1u8, 2u8].iter().copied().collect();
    let mut b : VecDeque<u8> = VecDeque::with_capacity(2);
    b.push_back(2u8);
    b.push_back(1u8);
    b.pop_front();
    b.push_back(2u8);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a == b);

}
