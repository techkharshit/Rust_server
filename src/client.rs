use clap::{App, Arg};
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Rust Client")
        .version("1.0")
        .author("Your Name")
        .about("Sends HTTP requests to a Rust server")
        .arg(Arg::new("base-url")
            .short('u')
            .long("url")
            .value_name("URL")
            .help("Sets the base URL of the server")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("file-path")
            .short('f')
            .long("file")
            .value_name("FILE_PATH")
            .help("Sets the file path to interact with")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("operation")
            .short('o')
            .long("operation")
            .value_name("OPERATION")
            .help("Specifies the operation: 'get' or 'put'")
            .takes_value(true)
            .required(true)
            .possible_values(&["get", "put"]))
        .get_matches();

    let base_url = matches.value_of("base-url").unwrap();
    let file_path = matches.value_of("file-path").unwrap();
    let operation = matches.value_of("operation").unwrap();

    match operation {
        "get" => {
            let get_url = format!("{}/{}", base_url, file_path);
            let response = reqwest::get(&get_url).await?;

            if response.status().is_success() {
                let content = response.text().await?;
                println!("Content of {}:\n{}", file_path, content);
            } else {
                println!("Failed to get {}: {}", file_path, response.status());
            }
        }
        "put" => {
            let new_content = reqwest::Client::new()
                .put(&format!("{}/{}", base_url, file_path))
                .body(get_input("Enter content to update the file"))
                .send()
                .await?;

            if new_content.status().is_success() {
                println!("File {} updated successfully", file_path);
            } else {
                println!("Failed to update {}: {}", file_path, new_content.status());
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
