use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub trait ParseHeader {
    fn parse_headers(self, delimiter: char) -> HeaderMap;
}

impl ParseHeader for Vec<String> {
    fn parse_headers(self, delimiter: char) -> HeaderMap {
        self.iter()
            .filter_map(|header| {
                header
                    .split_once(delimiter)
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            })
            .fold(HeaderMap::new(), |mut headers, (h_key, h_value)| {
                if let Ok(value) = h_value.parse::<HeaderValue>() {
                    headers.insert(HeaderName::from_bytes(h_key.as_bytes()).unwrap(), value);
                }
                headers
            })
    }
}
