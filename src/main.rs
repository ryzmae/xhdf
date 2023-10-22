use reqwest;
use structopt::StructOpt;
use std::error::Error;
use ansi_term::Colour::{Green, Blue};

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "GET")]
    Get {
        url: String,
        #[structopt(short = "H", long = "headers", parse(try_from_str = parse_header))]
        headers: Vec<(String, String)>,
    },

    #[structopt(name = "POST")]
    Post {
        url: String,
        body: String,
        #[structopt(short = "H", long = "headers", parse(try_from_str = parse_header))]
        headers: Vec<(String, String)>,
    },

    #[structopt(name = "PUT")]
    Put {
        url: String,
        body: String,
        #[structopt(short = "H", long = "headers", parse(try_from_str = parse_header))]
        headers: Vec<(String, String)>,
    },

    #[structopt(name = "DELETE")]
    Delete {
        url: String,
        #[structopt(short = "H", long = "headers", parse(try_from_str = parse_header))]
        headers: Vec<(String, String)>,
    },
}

fn parse_header(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err(String::from("Header must be in the form key=value"));
    }
    Ok((String::from(parts[0]), String::from(parts[1])))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    match args.cmd {
        Command::Get { url, headers } => {
            let url = reqwest::Url::parse(&url)?;
            let mut request_builder = reqwest::Client::new().get(url);
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
            let response = request_builder.send().await?;
            println!("{} - {} - {}", Green.bold().paint("GET"), Blue.bold().paint("HTTP/1.1"), Green.bold().paint(response.status().to_string()));
            println!("{}: */*", Blue.bold().paint("Accept"));
            let text = response.text().await?;
            println!("{}:\n{}", Blue.bold().paint("Data"), text)
        }
        Command::Post { url, body, headers } => {
            let url = reqwest::Url::parse(&url)?;
            let mut request_builder = reqwest::Client::new()
                .post(url)
                .body(body);
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
            let response = request_builder.send().await?;
            println!("{} - {} - {}", Green.bold().paint("POST"), Blue.bold().paint("HTTP/1.1"), Green.bold().paint(response.status().to_string()));
            println!("{}: */*", Blue.bold().paint("Accept"));
            let text = response.text().await?;
            println!("{}:\n{}", Blue.bold().paint("Data"), text)
        }

        Command::Put { url, body, headers } => {
            let url = reqwest::Url::parse(&url)?;
            let mut request_builder = reqwest::Client::new()
                .put(url)
                .body(body);
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
            let response = request_builder.send().await?;
            println!("{} - {} - {}", Green.bold().paint("PUT"), Blue.bold().paint("HTTP/1.1"), Green.bold().paint(response.status().to_string()));
            println!("{}: */*", Blue.bold().paint("Accept"));
            let text = response.text().await?;
            println!("{}:\n{}", Blue.bold().paint("Data"), text)
        }

        Command::Delete { url, headers } => {
            let url = reqwest::Url::parse(&url)?;
            let mut request_builder = reqwest::Client::new()
                .delete(url);
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
            let response = request_builder.send().await?;
            println!("{} - {} - {}", Green.bold().paint("PUT"), Blue.bold().paint("HTTP/1.1"), Green.bold().paint(response.status().to_string()));
            println!("{}: */*", Blue.bold().paint("Accept"));
            let text = response.text().await?;
            println!("{}:\n{}", Blue.bold().paint("Data"), text)
        }
    } 
    Ok(())
}
