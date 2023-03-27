use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let filename = "file.elm";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // elm type and function names
    let mut identifiers = Vec::new();

    // elm module exposing list element
    let exposed_item = Regex::new(r"^\s+[,\(]\s(\w+)")?;

    let lines = reader.lines().skip(1).map(|li| li.ok().expect("lines"));

    for line in lines {
        if let Some(capture) = exposed_item.captures(&line) {
            identifiers.push(capture.get(1).expect("identifier get").as_str().to_string());
        } else {
            break;
        }
    }

    // elm docs internal bookmark link
    let anchor_link = Regex::new(r"\[(\w+)\]\(\#(\w+)\)")?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        for capture in anchor_link.captures_iter(&line) {
            let whole = capture.get(0).expect("whole-get").as_str();
            let name = capture.get(1).expect("name-get").as_str();
            let link = capture.get(2).expect("link-get").as_str();
            if &name != &link {
                println!("{}: name and link different.", whole);
            }
            if !(identifiers.iter().any(|s| &s == &link)) {
                println!("The link '{}' does not correspond to an identifier.", link);
            }
        }
    }
    Ok(())
}
