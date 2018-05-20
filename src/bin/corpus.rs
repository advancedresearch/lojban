extern crate piston_meta;

use piston_meta::*;

fn main() {
    let corpus = include_str!("../../assets/en-jbo_small.tmx");
    let rules = include_str!("../../assets/en-jbo.syntax.txt");
    
    // Parse rules with meta language and convert to rules for parsing
    // text.
    let rules = match syntax_errstr(&rules) {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(rules) => rules
    };

    let mut data = vec![];
    match parse_errstr(&rules, &corpus, &mut data) {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(()) => {}
    };
    println!("{{");
    json::print(&data);
    println!("}}");
}
