use regex::{Match, Matches, Regex};
use std::{fs, str};

struct TokenTransitions<'t, 'r> {
    text: &'t str,
    it: Matches<'r, 't>,
    last_match_consumed: Option<Match<'t>>,
    reported_index: usize,
}

impl<'t, 'r> TokenTransitions<'t, 'r> {
    fn new(text: &'t str, pattern: &'r Regex) -> Self {
        let mut it = pattern.find_iter(text);
        let last_match_consumed = it.next();
        TokenTransitions {
            text,
            it,
            last_match_consumed,
            reported_index: 0,
        }
    }
}

impl<'t, 'r> Iterator for TokenTransitions<'t, 'r> {
    type Item = (&'t str, &'t str);

    fn next(&mut self) -> Option<Self::Item> {  // I'm sure there's something wonky happening at the end of the input text
        if let Some(regex_match) = self.last_match_consumed {
            let last_match_consumed_start = regex_match.start();
            if self.reported_index < last_match_consumed_start {
                // ltoken in between regex matches
                let ltoken = &self.text[self.reported_index..last_match_consumed_start];
                self.reported_index = last_match_consumed_start;
		let rtoken = regex_match.as_str();
                return Some((ltoken, rtoken));
            } else if self.reported_index == last_match_consumed_start {
                // ltoken is a regex match
                let ltoken = &regex_match.as_str();
                self.reported_index = regex_match.end();
                self.last_match_consumed = self.it.next();
		if let Some(new_regex_match) = self.last_match_consumed {
		    let rtoken = &self.text[self.reported_index..new_regex_match.start()];
		    return Some((ltoken, rtoken));
		}
		else { None }  // We reached the end of the input text?
            } else {
                panic!("Should be unreachable");
            }
        } else {
            None
        }
    }
}

fn main() {
    let input_text = fs::read_to_string("neuromancer.txt").unwrap();
    let token_delimiter_re = Regex::new(r#"(([\.,:]?( |\t|\n)+)|--|")"#).unwrap();

    for transition in TokenTransitions::new(&input_text, &token_delimiter_re) {
        println!("{:?}", transition);
    }
}
