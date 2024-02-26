use std::error::Error;

use clap::{arg, Command};

use clap::crate_authors;

fn main() -> Result<(), Box<dyn Error>> {
    let mut url: &str = "https://isitfurvesteralready.info/";

    let args = Command::new("IsItFurversterAlready")
        .version("1.0")
        .author(crate_authors!("Maakinoh"))
        .about("Requests from https://disitfurvesteralready.info/ if it is furverster already")
        .arg(arg!(--url <VALUE> "URL for a custom Server standard is https://disitfurvesteralready.info/").required(false))
        .get_matches();

    let url_arg = args.get_one::<String>("url");
    if url_arg.is_some() {
        url = url_arg.unwrap();
    }

    let response = reqwest::blocking::get(url)?.text()?;

    let document = scraper::Html::parse_document(&response);

    let selector = scraper::Selector::parse("div.maintext>h1").unwrap();

    document
        .select(&selector)
        .for_each(|x| println!("{}", x.inner_html()));
    Ok(())
}
