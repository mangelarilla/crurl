use reqwest::header::HeaderMap;
use std::fmt::{Display, Formatter};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();

    let client = reqwest::Client::new();

    let request = if args.post {
        client.post(&args.url[..])
    } else if args.put {
        client.put(&args.url[..])
    } else {
        client.get(&args.url[..])
    };

    let response = request.send().await?;

    println!("Headers: {}", PrettyHeaders::new(&response.headers()));

    if !args.headers_only {
        println!("Body: {}", response.text().await.unwrap());
    }

    Ok(())
}

/// Simplified http client
#[derive(StructOpt)]
struct Opt {
    /// Response headers only
    #[structopt(short, long)]
    headers_only: bool,
    /// Request with POST verb
    #[structopt(long)]
    post: bool,
    /// Request with PUT verb
    #[structopt(long)]
    put: bool,
    /// The target url to request
    url: String,
}

struct PrettyHeaders<'a> {
    headers: &'a HeaderMap,
}

impl PrettyHeaders<'_> {
    fn new(headers: &HeaderMap) -> PrettyHeaders {
        PrettyHeaders { headers }
    }
}

impl Display for PrettyHeaders<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (name, value) in self.headers {
            if let Ok(value) = value.to_str() {
                writeln!(f, "{}: {}", name, value)?
            }
        }
        Ok(())
    }
}
