use regex::{Match, Matches, Regex};
use std::{fs, str};

struct RegexInclusiveSplit<'t, 'r> {
    text: &'t str,
    it: Matches<'r, 't>,
    last_match_consumed: Option<Match<'t>>,
    reported_index: usize,
}

impl<'t, 'r> RegexInclusiveSplit<'t, 'r> {
    fn new(text: &'t str, pattern: &'r Regex) -> Self {
        let mut it = pattern.find_iter(text);
        let last_match_consumed = it.next();
        RegexInclusiveSplit {
            text,
            it,
            last_match_consumed,
            reported_index: 0,
        }
    }
}

impl<'t, 'r> Iterator for RegexInclusiveSplit<'t, 'r> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(regex_match) = self.last_match_consumed {
            let last_match_consumed_start = regex_match.start();
            if self.reported_index < last_match_consumed_start {
                // In between separators
                let retval = &self.text[self.reported_index..last_match_consumed_start];
                self.reported_index = last_match_consumed_start;
                return Some(retval);
            } else if self.reported_index == last_match_consumed_start {
                // On a separator
                let retval = &regex_match.as_str();
                self.reported_index = regex_match.end();
                self.last_match_consumed = self.it.next();
                return Some(retval);
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

    for token in RegexInclusiveSplit::new(&input_text, &token_delimiter_re) {
        println!("{}", token);
    }
}
