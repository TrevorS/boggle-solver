use crate::trie::Node;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn dict(path: &str) -> Node {
    let mut trie = Node::new();
    let f = File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        match line {
            Ok(line) => {
                trie.insert(&mut line.chars());
            }
            Err(_) => {}
        }
    }
    trie
}
