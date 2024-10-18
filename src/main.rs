<<<<<<< HEAD
use clap::{Parser, Subcommand, builder::TypedValueParser};
use regex::Regex;
use std::{path::PathBuf, ffi::OsStr};
use std::io::BufReader;
use std::path::Path;
use std::io::Read;
use anyhow::{Context, Result};
use reqwest::{Response, Client};
use tokio::io::AsyncReadExt;
use tokio::task::spawn_blocking;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

//Defining base arguments for the script
struct Args {
    #[arg(short = 'r', long = "regex", value_parser = RegexParser)]
=======
use clap::{builder::{Str, TypedValueParser}, error::ErrorKind, Parser, Subcommand};
use regex::Regex;
use std::{ffi::OsStr, fmt::format, path::PathBuf};
use anyhow::{Context, Ok, Result};
use url::Url;
use serde_json::Value;
use reqwest::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'r', long = "regex", value_parser = RegexParser )]
>>>>>>> 21b639b (Debugging reqwest to properly get the body and the headers withing one request)
    regex: Vec<Regex>,

    #[arg(short = 'u', long = "url")]
    url: PathBuf,

    #[arg(short = 'a', long = "append", required = false)]
    append: Option<PathBuf>,
}

#[derive(Clone)]
struct RegexParser;

impl TypedValueParser for RegexParser {
    type Value = Regex;

    fn parse_ref(
<<<<<<< HEAD
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let s = value.to_str().ok_or_else(||
        clap::Error::raw(clap::error::ErrorKind::InvalidUtf8, "Invalid UTF-8 in regex")
        )?;

        Regex::new(s).map_err(|e|
        clap::Error::raw(clap::error::ErrorKind::InvalidValue, format!("Invalid regex: {}", e))
        )
=======
            &self,
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            value: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
            let s = value.to_str().ok_or_else(||
                clap::Error::raw(clap::error::ErrorKind::InvalidUtf8, "Invalid UTF-8 in regex")
                )?;
            Regex::new(s).map_err(|e|
                clap::Error::raw(clap::error::ErrorKind::InvalidValue, format!("Invalid regex: {}", e))
                )
>>>>>>> 21b639b (Debugging reqwest to properly get the body and the headers withing one request)
    }
}

fn format_url(url: &PathBuf) -> Result<Url, url::ParseError> {
    let url_str = url.to_str().ok_or_else(|| url::ParseError::IdnaError)?;
    Url::parse(url_str)
<<<<<<< HEAD

}


=======
}

>>>>>>> 21b639b (Debugging reqwest to properly get the body and the headers withing one request)
async fn request_handling() -> Result<()> {
    let args: Args = Args::parse();
    let url = format_url(&args.url)
        .context("Failed to parse the provided URL")?;
<<<<<<< HEAD

    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

=======
    let client = Client::new();
    let res = client
        .get(url)
        .text()
        .await;
    
    println!("Status: {}", res.status().to_string());
    println!("Headers:\n{:#?}", res.headers());

    let ct_type = res.headers().get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    let body = &res.text().await?;
    println!("Body:\n{}", format_body(ct_type, &body));
    
    Ok(())
}

// Format the body of the response depending on the "Content-Type" Headers, in some case, it can
// cause trouble to not make a double check to be sure that the content-type and the actual content
// are matching
fn format_body(ct_type: &str, body: &str) -> String {
    
    if ct_type.contains("application/json") {
        if let anyhow::Result::Ok(json) = serde_json::from_str::<Value>(body) {
            return serde_json::to_string_pretty(&json).unwrap_or(body.to_string());
        }
    } else if ct_type.contains("text/html") {
        return body.lines()
            .enumerate()
            .map(|(i, line)| format!("{}{}", "   ".repeat(i), line))
            .collect::<Vec<String>>()
            .join("\n");
    }
     body.to_string()
}


>>>>>>> 21b639b (Debugging reqwest to properly get the body and the headers withing one request)
#[tokio::main]
async fn main() -> Result<()> {
    request_handling().await?;
    Ok(())
<<<<<<< HEAD
}
=======
}
>>>>>>> 21b639b (Debugging reqwest to properly get the body and the headers withing one request)
