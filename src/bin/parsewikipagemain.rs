use scraper::{wiki, debug_with_context};
use async_std::task;
use itertools::Itertools;
use std::collections::{VecDeque, HashMap};
use async_std::sync::Arc;
use std::sync::{Mutex, RwLock};
use std::ops::Deref;

fn main() {

    let page =
        task::block_on(wiki::read_page("Gravity")).expect("error");

    // let page =
    //     wiki::read_page_sync("Gravity");

    let reverse_link_map = Arc::new(RwLock::new(HashMap::new()));
    let reverse_link_map_cloned = reverse_link_map.clone();

    let handles: Vec<_> = page.page_link_names.iter().take(10)
        .cloned()
        .map(move |page_name| task::spawn(scrape_page(page_name, reverse_link_map_cloned.clone())))
        .collect();

    handles.into_iter().for_each(|h| task::block_on(h));

    // println!("revlinkmap: {:?}", reverse_link_map.lock().unwrap().deref());
    println!("revlinkmap:");
    for entry in reverse_link_map.read().unwrap().iter() {
        for page_name in entry.1 {
            println!("  {} <- {}", entry.0, page_name);
        }
    }

    // match page {
    //     Ok(res) => {
    //         println!("{}:", res.page_name);
    //         for link in res.page_link_names {
    //             println!("  {}", link);
    //         }
    //     },
    //     Err(err) => eprintln!("{}", err),
    // };
}

async fn scrape_page(
    page_name: impl AsRef<str>,
    reverse_link_map: Arc<RwLock<HashMap<String, Vec<String>>>>) {
    debug_with_context(&format!("scraping {}", page_name.as_ref()));

    let page = wiki::read_page(page_name.as_ref()).await.expect("error");

    debug_with_context(&format!("updating map {}", page_name.as_ref()));

    let mut rlm = reverse_link_map.write().expect("could not lock");
    for link_name in page.page_link_names {
        // println!("insert {} <- {}", link_name, page_name.as_ref());
        rlm.entry(link_name).or_default().push(page_name.as_ref().to_string());
    }

}
