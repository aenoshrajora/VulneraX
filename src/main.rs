// Created by Aenosh Rajora
// Version: 1.0

// Dependencies
use chrono::Local; // 0.4.11
use isahc::prelude::*; // 0.9.3
use rayon::prelude::*; // 1.3.0
use regex::Regex; // 1.3.9
use std::fs::File;
use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::time::Duration;

const BANNER: &str = "\x1b[95m    https://github.com/aenoshrajora/VulneraX

\x1b[94m██╗   ██╗██╗   ██╗██╗     ███╗   ██╗███████╗██████╗  █████╗ ██╗  ██╗
██║   ██║██║   ██║██║     ████╗  ██║██╔════╝██╔══██╗██╔══██╗╚██╗██╔╝
██║   ██║██║   ██║██║     ██╔██╗ ██║█████╗  ██████╔╝███████║ ╚███╔╝ 
╚██╗ ██╔╝██║   ██║██║     ██║╚██╗██║██╔══╝  ██╔══██╗██╔══██║ ██╔██╗ 
 ╚████╔╝ ╚██████╔╝███████╗██║ ╚████║███████╗██║  ██║██║  ██║██╔╝ ██╗
  ╚═══╝   ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝\x1b[91m
Made with ❤ by Shadow Exploit\x1b[93m

                  v1.0\x1b[94m

This cannot guarantee 100% accuracy
Use this responsibly, be ethical
Happy bug hunting!
\x1b[91m════════════════════════════════════════════\x1b[0m\n";

const MENU: &str = "\x1b[92m[0]. Exit
[1]. XSS Scan\x1b[0m";

// Main function
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check internet connection
    if let Err(e) = connection() {
        eprintln!("\x1b[91m[ERROR] Connection not found: {}\x1b[0m", e);
        return Ok(()); // Exit gracefully
    }

    // Display menu
    println!("{}{}", BANNER, MENU);
    print!("\x1b[94m[ 𝓡𝓥𝓾𝓵𝓷 ] -> ");
    stdout().flush().unwrap();

    let input = input()?;
    match input.as_str() {
        "1" => xss()?,
        "0" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid Option"),
    }

    Ok(())
}

// XSS Scan function
fn xss() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    print!("\n{}[2J", 27 as char);

    // Get Target URL
    print!("\x1b[94mTarget URL: ");
    stdout().flush().unwrap();
    let target_url = input()?;

    // Get Parameters
    print!("\x1b[91mQuery Parameters: ");
    stdout().flush().unwrap();
    let params = input()?;

    // Get Wordlist Path
    print!("\x1b[93mPath to Wordlist: ");
    stdout().flush().unwrap();
    let wordlist = input()?;

    // Verbose Mode
    print!("\x1b[95mVerbose output? [y/n]: ");
    stdout().flush().unwrap();
    let verbose = matches!(input()?.to_lowercase().as_str(), "y");

    // Read wordlist and start scanning
    read_files(&target_url, &wordlist, verbose, &params)?;

    Ok(())
}

// Input handler
fn input() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

// Read from wordlist and scan
fn read_files(target_host: &str, wordlist: &str, verbose: bool, params: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
    let timeout = 15;

    let fd = File::open(wordlist)?;
    let payloads: Vec<String> = BufReader::new(fd)
        .lines()
        .filter_map(Result::ok)
        .collect();

    // Parallel request execution
    payloads.par_iter().for_each(|payload| {
        if let Err(e) = request(target_host, payload, ua, verbose, timeout, params) {
            eprintln!("\x1b[91m[ERROR] Something went wrong!\n{}\x1b[0m", e);
        }
    });

    Ok(())
}

// Send HTTP request
fn request(
    host: &str,
    payload: &str,
    ua: &str,
    verbose: bool,
    timeout: u64,
    params: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let re = Regex::new("\\$")?;
    let url = format!("{}?{}", host, params);
    let url = re.replace(&url, payload);
    let url = url_encode(&url);

    let response = Request::get(&url)
        .header("user-agent", ua)
        .timeout(Duration::new(timeout, 0))
        .body(())?
        .send()?
        .text()?;

    let source_contains_payload = response.contains(payload);
    let time = Local::now().format("%T");

    if source_contains_payload {
        println!("\x1b[92m[{}] | [+] {}\x1b[93m\n═════════════════════════════════════════════════════════", time, url);
    } else if verbose {
        println!("\x1b[91m[{}] | [-] {}\x1b[93m\n═════════════════════════════════════════════════════════", time, url);
    }

    Ok(())
}

// URL Encoding
fn url_encode(data: &str) -> String {
    let unsafe_chars = " '\"<>#%{}|\\^~[]+";
    data.chars()
        .map(|c| {
            if unsafe_chars.contains(c) {
                format!("%{:X}", c as u8)
            } else {
                c.to_string()
            }
        })
        .collect()
}

// Check for internet connection
fn connection() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let test_url = "https://www.google.com/";
    match Request::head(test_url)
        .timeout(Duration::new(15, 0))
        .body("")
        .map_err(|e| format!("Request build failed: {}", e))?
        .send()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
