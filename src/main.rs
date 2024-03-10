use std::error::Error;

use clap::{arg, Arg, ArgAction, Command};

use clap::crate_authors;
use regex::Regex;
use scraper::Html;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse{
    #[serde(rename = "daysRemaining")]
    days_remaining: i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut base_url: &str = "https://isitfurvesteralready.info/";

    let use_scraper = Arg::new("use-scraper")
        .long("use-scraper")
        .action(ArgAction::SetTrue)
        //.value_name("FILE")
        .help("Specify that the scraper should be used instead of the api");
    let args = Command::new("IsItFurversterAlready")
        .version("1.0")
        .author(crate_authors!("Maakinoh"))
        .about("Requests from https://isitfurvesteralready.info/ if it is furverster already")
        .arg(arg!(--url <VALUE> "URL for a custom Server standard is https://isitfurvesteralready.info/").required(false))
        .arg(arg!(--days "Returns the remaining days until furvester"))
        //.arg(arg!(--use-scraper ))
        .arg(use_scraper)
        .get_matches();

    let url_arg = args.get_one::<String>("url");
    if url_arg.is_some() {
        base_url = url_arg.unwrap();
    }

    // If the old scraper should be used
    if args.get_flag("use-scraper") {
        let response = reqwest::blocking::get(base_url)?.text()?;

        let document = scraper::Html::parse_document(&response);

        if !args.get_flag("days") {
            parse_is_it_furvester(&document);
        } else {
            parse_remaining_days(&document);
        }
    } else {
        // Use the api instead

        let result : ApiResponse = reqwest::blocking::get(base_url.to_owned() +"api/v1/getDaysRemaining")?.json()?;
        if !args.get_flag("days") {
            if result.days_remaining > 0 {
                println!("No");
            }
        } else {
            println!("{} days remaining", result.days_remaining);
        }

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
