use structopt::StructOpt;
use reqwest::Error;

/// Simplified http client
#[derive(StructOpt)]
struct Opt {
	/// Response headers only
	#[structopt(short, long)]
	headers_only: bool,
    /// The target url to request
    url: String,
}

fn main() -> Result<(), Error> {
    let args = Opt::from_args();
    
    let client = reqwest::Client::new();
    let request = client.get(&args.url[..]);

    let mut response = request.send()?;

    println!("{:?}", response);

    if !args.headers_only {
	    println!("Body: {}", response.text().unwrap());
    }

    Ok(())
}
