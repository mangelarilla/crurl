mod parse_headers;
mod request_builder;
mod structopt;

use crate::structopt::*;
use request_builder::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Opt = Opt::from_args();

    let request = reqwest::Client::new()
        .build_base_request(&args)
        .with_body(args.body)
        .with_headers(args.headers);

    if args.show_request {
        println!("Request: \n\n{:#?}\n", request);
    }

    let response = request.send().await?;

    println!("Response: \n\nHeaders: {:#?}", response.headers());

    if !args.headers_only {
        println!("Body: {}", response.text().await.unwrap());
    }

    Ok(())
}
