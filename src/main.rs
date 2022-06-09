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

    println!("Headers: {:#?}", response.headers());

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
