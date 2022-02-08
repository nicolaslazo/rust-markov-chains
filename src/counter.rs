use std::collections::HashMap;

pub type Counter = HashMap<String, i32>;

pub trait Update {
    fn update(&mut self, key: String) -> ();
}

impl Update for Counter {
    fn update(&mut self, key: String) -> () {
	let count = self.entry(key).or_insert(0);
	*count += 1;
    }
}
