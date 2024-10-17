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
    }
}

fn format_url(url: &PathBuf) -> Result<Url, url::ParseError> {
    let url_str = url.to_str().ok_or_else(|| url::ParseError::IdnaError)?;
    Url::parse(url_str)

}


async fn request_handling() -> Result<()> {
    let args: Args = Args::parse();
    let url = format_url(&args.url)
        .context("Failed to parse the provided URL")?;

    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    request_handling().await?;
    Ok(())
}