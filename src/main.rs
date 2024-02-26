use std::error::Error;

use clap::{arg, Command};

use clap::crate_authors;
use regex::Regex;
use scraper::Html;

fn main() -> Result<(), Box<dyn Error>> {
    let mut url: &str = "https://isitfurvesteralready.info/";

    let args = Command::new("IsItFurversterAlready")
        .version("1.0")
        .author(crate_authors!("Maakinoh"))
        .about("Requests from https://isitfurvesteralready.info/ if it is furverster already")
        .arg(arg!(--url <VALUE> "URL for a custom Server standard is https://disitfurvesteralready.info/").required(false))
        .arg(arg!(--days "Returns the remaining days until furvester"))
        .get_matches();

    let url_arg = args.get_one::<String>("url");
    if url_arg.is_some() {
        url = url_arg.unwrap();
    }

    let response = reqwest::blocking::get(url)?.text()?;

    let document = scraper::Html::parse_document(&response);

    if !args.get_flag("days") {
        parse_is_it_furvester(&document);
    } else {
        parse_remaining_days(&document);
    }
    Ok(())
}

fn parse_is_it_furvester(document: &Html) {
    let selector = scraper::Selector::parse("div.maintext>h1").unwrap();

    document
        .select(&selector)
        .for_each(|x| println!("{}", x.inner_html()));
}

fn parse_remaining_days(document: &Html) {
    let selector = scraper::Selector::parse("div.footer>p").unwrap();

    document.select(&selector).for_each(|x| {
        let val = x.inner_html();

        let re = Regex::new(r#"\bin (\d+) days"#).unwrap();

        if let Some(capture) = re.captures(val.as_str()) {
            // Find first match
            if let Some(number_str) = capture.get(1) {
                // extract number
                if let Ok(number) = number_str.as_str().parse::<u32>() {
                    println!("{} days until furvester", number);
                    return;
                }
            }
        }

        let end = val.trim_start_matches("... /D+");
        println!("{}", end);
    });
}


