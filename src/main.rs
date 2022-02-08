use regex::{Match, Matches, Regex};
use std::{fs, str, collections::HashMap};

mod counter;
use counter::{Counter, Update};

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

struct TokenTransitions<'t, 'r> {
    token_it: &'t mut RegexInclusiveSplit<'t, 'r>,
    last_token: &'t str,
}

impl<'t, 'r> TokenTransitions<'t, 'r> {
    fn new(token_it: &'t mut RegexInclusiveSplit<'t, 'r>) -> Self {
	let last_token = token_it.next().unwrap();
	TokenTransitions {
	    token_it,
	    last_token
	}
    }
}
	
impl<'t, 'r> Iterator for TokenTransitions<'t, 'r> {
    type Item = (&'t str, &'t str);

    fn next(&mut self) -> Option<Self::Item> {
	while let Some(next_token) = self.token_it.next() {
	    if next_token == " " { continue };  // We're ignoring other tokens with spaces like ". " for now

	    let retval = (self.last_token, next_token);
	    self.last_token = next_token;
	    return Some(retval)
	}
	return None
    }
}

fn main() {
    let input_text = fs::read_to_string("neuromancer.txt").unwrap().to_ascii_lowercase();
    let token_delimiter_re = Regex::new(r#"(([\.,:\?]?( |\t|\n|")+)|--)"#).unwrap();
    let mut token_it = RegexInclusiveSplit::new(&input_text, &token_delimiter_re);
    let mut transition_counts = HashMap::<String, Counter>::new();

    TokenTransitions::new(&mut token_it).for_each(
	|(ltoken, rtoken)| transition_counts.entry(ltoken.to_string()).or_insert(Counter::new()).update(rtoken.to_string())
    );
    for count in transition_counts {
        println!("{:?}", count);
    }
}
