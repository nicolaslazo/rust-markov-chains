use rustc_hash::FxHashMap;

pub type Counter = FxHashMap<String, i32>;

pub trait Update {
    fn update(&mut self, key: String);
}

impl Update for Counter {
    fn update(&mut self, key: String) {
        let count = self.entry(key).or_insert(0);
        *count += 1;
    }
}
