
pub mod foo{

use reqwest::header::HeaderMap;
use reqwest::header;

/// parse the header vector to a header map
 pub fn get_headers(headers:&'static Vec<String>) -> Option<HeaderMap> {
    let mut header_map = HeaderMap::new();
    let headers = headers.iter().map(|header| {
        if !header.contains(":") {
            return ;
        }
        let header_split = header.split(":").collect::<Vec<&str>>();
        let key = header_split[0];
        let value = header_split[1];
        header_map.insert(header::HeaderName::from_static(key), header::HeaderValue::from_static(value));
    });
    Some(header_map)
}
}