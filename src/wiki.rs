use std::collections::{VecDeque, HashSet};
use reqwest::{Client, Response};
use crate::debug_with_context;
use async_std::prelude::*;
use bytes::Bytes;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WikiPage {
    pub page_name: String,
    pub page_link_names: HashSet<String>,
}

pub async fn read_page(page_name: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    debug_with_context("read page get");
    let result = reqwest::get(to_url(&page_name)).await;
    debug_with_context("read page bytes");
    let response: Response = result?.error_for_status()?;
    // let page_link_names = read_links_stream(response.bytes_stream()).await;
    let page_link_names = read_links(response.bytes().await?.iter().copied());
    Ok(WikiPage{ page_name: page_name.as_ref().to_string(), page_link_names})
}

pub fn read_page_sync(page_name: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    let response = reqwest::blocking::get(to_url(&page_name))?.error_for_status()?;
    let page_link_names = read_links(response.bytes()?.iter().copied());
    Ok(WikiPage{ page_name: page_name.as_ref().to_string(), page_link_names })
}

pub fn to_url(page_name: impl AsRef<str>) -> String {
    "http://en.wikipedia.org/wiki/".to_string() + page_name.as_ref()
}

// async fn read_links_stream(mut bytes: impl Stream<Item=reqwest::Result<Bytes>>) -> HashSet<String> {
//     let mut link_page_names = HashSet::new();
//     while let Some(bytes) = bytes.next().await {
//         println!("{:?}", bytes);
//     }
//     link_page_names
// }

fn read_links(mut bytes: impl Iterator<Item=u8>) -> HashSet<String> {
    // this method is not exactly correct charset handling, but works for now
    let href_bytes : VecDeque<u8> = "<a href=\"/wiki/".bytes().collect();
    let mut buffer = VecDeque::with_capacity(href_bytes.len());
    let mut link_page_names = HashSet::new();
    while let Some(byte) = bytes.next() {
        if buffer.len() == href_bytes.len() {
            buffer.pop_front();
        }
        buffer.push_back(byte);

        if buffer == href_bytes {
            let mut match_bytes = Vec::new();
            while let Some(byte) = bytes.next() {
                if byte == '"' as u8 {
                    break;
                }
                match_bytes.push(byte);
            }
            let link = String::from_utf8(match_bytes).expect("invalid utf8");
            if !link.contains(":") && !link.contains("#") {
                link_page_names.insert(link);
            }
        }
    }
    link_page_names
}