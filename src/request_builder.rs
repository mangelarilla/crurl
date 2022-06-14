use crate::parse_headers::ParseHeader;
use crate::structopt::Opt;
use reqwest::{Client, RequestBuilder};

const HEADER_DELIMITER: char = '=';

pub trait ClientRequestBuilderFromArgs {
    fn build_base_request(self, args: &Opt) -> RequestBuilder;
}

pub trait RequestBuilderFromArgs {
    fn with_body(self, body: Option<String>) -> RequestBuilder;
    fn with_headers(self, headers: Option<Vec<String>>) -> RequestBuilder;
}

impl ClientRequestBuilderFromArgs for Client {
    fn build_base_request(self, args: &Opt) -> RequestBuilder {
        let request = if args.post {
            self.post(&args.url[..])
        } else if args.put {
            self.put(&args.url[..])
        } else {
            self.get(&args.url[..])
        };

        request
    }
}

impl RequestBuilderFromArgs for RequestBuilder {
    fn with_body(self, body: Option<String>) -> RequestBuilder {
        match body {
            Some(body) => self.body(body),
            None => self,
        }
    }

    fn with_headers(self, headers: Option<Vec<String>>) -> RequestBuilder {
        match headers {
            Some(headers) => self.headers(headers.parse_headers(HEADER_DELIMITER)),
            None => self,
        }
    }
}
