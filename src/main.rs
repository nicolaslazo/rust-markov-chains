use std::fs;

use regex::Regex;

fn main() {
    let input_text = Box::new(fs::read_to_string("neuromancer.txt").unwrap());
    let token_delimiter_re = Regex::new(r"(([\.,]?( |\t|\n)+)|--|:)").unwrap();
    let token_iter = token_delimiter_re.split(&input_text);

    for token in token_iter {
	println!("{}", token);
    }
}
