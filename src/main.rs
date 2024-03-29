use clap::Parser;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::io::{self, BufRead};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Clone, Debug)]
#[command(
    author = "Cory Sabol",
    version = "0.1.0",
    about = "rrr (really rapid requesor) is a simple too to rapidly request URLs.",
    after_help = "Examples:
    cat ranges.txt | httpx | rrr -d responses 
    cat urls.txt | rrr -i 404,403,500 -o > responses.txt
    cat ranges.txt | daship | httpx | rrr -o | rg \"hackme\" > intersting.txt
    "
)]
struct Args {
    /// Optional HTTP method to use for requests
    #[clap(short, long, default_value = "GET")]
    method: String,

    /// Optional directory to save response bodies to
    #[clap(short, long, default_value = "responses")]
    directory: String,

    /// Optional list of HTTP response status codes to ignore e.g. 404,403,500
    #[clap(short, long)]
    ignore: Option<String>,

    /// Print responses to STDOUT
    #[clap(short = 'o', long, action)]
    stdout: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    let ignore_codes: Vec<u16> = args
        .ignore
        .clone()
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let stdin = io::stdin();
    let mut handles = Vec::new();

    for line in stdin.lock().lines() {
        let url = line?;

        let client = client.clone();
        let args = args.clone();
        let ignore_codes = ignore_codes.clone();
        //if let Err(e) = process_url(&client, &args, &ignore_codes, &url).await {
        //    eprintln!("Error processing {}: {}", url, e);
        //};
        let handle = tokio::spawn(async move {
            if let Err(e) = process_url(&client, &args, &ignore_codes, &url).await {
                eprintln!("Error processing {}: {}", url, e);
            }
        });
        handles.push(handle);
    }

    // await handles
    // We don't really care about the results.
    let _ = futures::future::join_all(handles).await;

    Ok(())
}

async fn process_url(
    client: &Client,
    args: &Args,
    ignore_codes: &Vec<u16>,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
        .request(
            reqwest::Method::from_bytes(args.method.as_bytes()).unwrap(),
            url,
        )
        .send()
        .await?;

    if ignore_codes.contains(&res.status().as_u16()) {
        return Ok(());
    }

    let body = res.text().await?;
    let hash = Sha256::digest(body.as_bytes());
    let hash_str = hex::encode(hash);

    if args.stdout {
        println!("{}", body);
    } else {
        tokio::fs::create_dir_all(&args.directory).await?;
        let parsed_url = url::Url::parse(url);
        let host_name = match parsed_url {
            Ok(url) => url.host_str().unwrap_or("").to_string(),
            Err(_) => "".to_string(),
        };
        let filename = format!("{}-{}.sha256", host_name, hash_str);
        let path = std::path::Path::new(&args.directory).join(filename);
        let mut file = File::create(&path).await?;
        file.write_all(body.as_bytes()).await?;
        println!("Saved response to {:?}", path);
    }

    Ok(())
}
