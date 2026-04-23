#[derive(Debug)]
pub struct MyHashSet {
    data: Vec<i32>,
}

#[allow(dead_code)]
impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet { data: vec![] }
    }

    pub fn add(&mut self, key: i32) {
        if key as usize >= self.data.len() {
            Vec::resize(&mut self.data, key as usize + 1, 0);
        }

        self.data[key as usize] += 1;
    }

    pub fn remove(&mut self, key: i32) {
        if self.data.len() < key as usize {
            return;
        }

        self.data[key as usize] -= 1;
    }

    pub fn contains(&self, key: i32) -> bool {
        if self.data.len() < key as usize || self.data[key as usize] <= 0 {
            return false;
        }

        true
    }
}

#[allow(dead_code)]
pub fn get_test_cases() {}

#[allow(dead_code)]
pub fn solve() {}

#[allow(dead_code)]
pub fn organize_result() {}
