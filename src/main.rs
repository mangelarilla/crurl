use structopt::StructOpt;
use reqwest::Error;

/// Simplified rust version of curl
#[derive(StructOpt)]
struct Opt {
    /// The target url to request
    url: String,
}

fn main() -> Result<(), Error> {
    let args = Opt::from_args();
    
    let client = reqwest::Client::new();
    let request = client.get(&args.url[..]);

    let response = request.send()?;

    println!("{:?}", response);

    Ok(())
}
