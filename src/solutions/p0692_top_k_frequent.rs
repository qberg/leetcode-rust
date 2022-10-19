use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct WordInfo {
    word: String,
    count: usize,
}


pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_vec: Vec<WordInfo> = vec![];
        let mut hash_map: HashMap<String,usize> = HashMap::new();
        for word in words.into_iter() {
            let count = hash_map.entry(word.to_string()).or_insert(0);
            // count contains a mutable reference 
            // to the value at the specified key.
            *count += 1;
        }
        // Iterating like following will tranfer the ownership
        // of the hash_map to the for loop block.
        for (w,c) in  hash_map {
            word_vec.push(
                WordInfo {word: w.to_string(),count: c}
            );
        };
        word_vec.sort_unstable_by(|a,b| {
            match b.count.cmp(&a.count) {
                Ordering::Equal => a.word.cmp(&b.word),
                x => x,
            }
        });
        word_vec
            .into_iter()
            .take(k as usize)
            .map(|wv| wv.word.to_string())
            .collect()
    }
}

