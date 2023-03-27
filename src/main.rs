use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::RegexSet;

fn main() {
    let filename = "file.elm";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut functions_and_types = Vec::new();

    let set = RegexSet::new(&[
        r"^(\w*) : ",         // elm function name
        r"^type (\w*)",       // elm type name
        r"^type alias (\w*)", // elm type alias name
    ])  

    for line in reader.lines() {

    }

    let mut broken_links = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        for link in line.matches(|c| !c.is_ascii_alphanumeric() && c != '_') {
            if link.starts_with("#") {
                let name = &link[1..];
                if !functions_and_types.contains(&name.to_string()) {
                    broken_links.push(link.to_string());
                }
            }
        }
    }

    if !broken_links.is_empty() {
        println!("Broken links:");
        for link in broken_links {
            println!("{}", link);
        }
    }
}
