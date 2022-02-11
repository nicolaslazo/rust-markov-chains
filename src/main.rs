use rand::{
    distributions::{Distribution, WeightedIndex},
    thread_rng,
};
use regex::{Match, Matches, Regex};
use rustc_hash::FxHashMap;
use std::{fs, str};

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
            match last_match_consumed_start {
                start if start > self.reported_index => {
                    // In between separators
                    let retval = &self.text[self.reported_index..last_match_consumed_start];
                    self.reported_index = last_match_consumed_start;
                    return Some(retval);
                }
                start if start == self.reported_index => {
                    // On a separator
                    let retval = &regex_match.as_str();
                    self.reported_index = regex_match.end();
                    self.last_match_consumed = self.it.next();
                    return Some(retval);
                }
                _ => panic!("Should be unreachable"),
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
            last_token,
        }
    }
}

impl<'t, 'r> Iterator for TokenTransitions<'t, 'r> {
    type Item = (&'t str, &'t str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_token) = self.token_it.find(|x| *x != " ") {
            let retval = (self.last_token, next_token);
            self.last_token = next_token;
            return Some(retval);
        } else {
            return None;
        }
    }
}

fn main() {
    let input_text = fs::read_to_string("neuromancer.txt")
        .unwrap()
        .to_ascii_lowercase();
    let token_delimiter_re = Regex::new(r#"(([\.,:\?;]?( |\t|\n|")+)|--)"#).unwrap();

    let mut token_it = RegexInclusiveSplit::new(&input_text, &token_delimiter_re);
    let mut transition_counts = FxHashMap::<String, Counter>::default();
    TokenTransitions::new(&mut token_it).for_each(|(ltoken, rtoken)| {
        transition_counts
            .entry(ltoken.to_string())
            .or_insert_with(Counter::default)
            .update(rtoken.to_string())
    });

    let mut token_transitions = FxHashMap::<String, Vec<&String>>::default(); // Horrible name, not to be mixed up with the iterator
    for (ltoken, rtoken_counts) in &transition_counts {
        token_transitions.insert(
            ltoken.to_string(),
            rtoken_counts.keys().collect::<Vec<&String>>(),
        );
    }

    let mut token_weights = FxHashMap::<String, WeightedIndex<_>>::default();
    for (ltoken, rtoken_counts) in &transition_counts {
        let weights = rtoken_counts.values().copied().collect::<Vec<i32>>();
        token_weights.insert(ltoken.to_string(), WeightedIndex::new(&weights).unwrap());
    }

    let mut rng = thread_rng();
    let mut current_token = "the";

    for _ in 0..90 {
        println!("{}", current_token);
        let next_index = token_weights[current_token].sample(&mut rng);
        current_token = token_transitions[current_token][next_index];
    }
}
