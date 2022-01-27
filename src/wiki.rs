use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct WikiPage {
    pub page: String,
    pub wiki_links: Vec<String>,
}

pub async fn read_page(path: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    let response = reqwest::get(path.as_ref()).await?.error_for_status()?;
    Ok(WikiPage{page: path.as_ref().to_string(), wiki_links: Vec::default()})
}

pub fn read_page_sync(path: impl AsRef<str>) -> reqwest::Result<WikiPage> {
    let response = reqwest::blocking::get(path.as_ref())?.error_for_status()?;
    let wiki_links = read_links(response.bytes()?.iter().copied());
    Ok(WikiPage{page: path.as_ref().to_string(), wiki_links})
}

fn read_links(mut bytes: impl Iterator<Item=u8>) -> Vec<String> {
    // this method is not exactly correct charset handling, but works for now
    let href_bytes : VecDeque<u8> = "<a href=\"/wiki/".bytes().collect();
    let mut buffer = VecDeque::with_capacity(href_bytes.len());
    let mut links = Vec::new();
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
                links.push(link);
            }
        }
    }
    links
}