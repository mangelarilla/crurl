pub use structopt::StructOpt;

/// Simplified http client
#[derive(StructOpt)]
pub struct Opt {
    /// Response headers only
    #[structopt(short, long)]
    pub headers_only: bool,
    /// Display the request
    #[structopt(short, long)]
    pub show_request: bool,
    /// Request with POST verb
    #[structopt(long)]
    pub post: bool,
    /// Request with PUT verb
    #[structopt(long)]
    pub put: bool,
    /// Set request headers. Ex: "Accept=*/*"
    #[structopt(short = "H", long)]
    pub headers: Option<Vec<String>>,
    /// Set request body
    #[structopt(short = "B", long)]
    pub body: Option<String>,
    /// The target url to request
    pub url: String,
}
