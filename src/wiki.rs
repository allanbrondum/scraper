use std::collections::{VecDeque, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WikiPage {
    pub page_name: String,
    pub page_link_names: HashSet<String>,
}

pub async fn read_page(page_name: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    let response = reqwest::get(to_url(&page_name)).await?.error_for_status()?;
    Ok(WikiPage{ page_name: page_name.as_ref().to_string(), page_link_names: HashSet::default()})
}

pub fn read_page_sync(page_name: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    let response = reqwest::blocking::get(to_url(&page_name))?.error_for_status()?;
    let wiki_links = read_links(response.bytes()?.iter().copied());
    Ok(WikiPage{ page_name: page_name.as_ref().to_string(), page_link_names: wiki_links })
}

pub fn to_url(page_name: impl AsRef<str>) -> String {
    "http://en.wikipedia.org/wiki/".to_string() + page_name.as_ref()
}

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