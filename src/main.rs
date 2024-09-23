use std::collections::HashMap;

struct TrieNode {
    children: HashMap<i32, Box<TrieNode>>,
    count: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            count: 0,
        }
    }
}

struct NGramTrie {
    root: TrieNode,
    ngram_length: usize,
}

impl NGramTrie {
    fn new(ngram_length: usize) -> Self {
        NGramTrie {
            root: TrieNode::new(),
            ngram_length,
        }
    }

    fn insert(&mut self, ngram: &[i32]) {
        assert_eq!(ngram.len(), self.ngram_length, "N-gram length must match the specified length");
        let mut node = &mut self.root;
        for &token in ngram {
            node = node.children.entry(token).or_insert_with(|| Box::new(TrieNode::new()));
            node.count += 1;
        }
    }

    fn search(&self, ngram: &[i32]) -> i32 {
        assert_eq!(ngram.len(), self.ngram_length, "N-gram length must match the specified length");
        let mut node = &self.root;
        for &token in ngram {
            match node.children.get(&token) {
                Some(child) => node = child,
                None => return 0,
            }
        }
        node.count
    }
}

fn main() {
    let mut trie = NGramTrie::new(3);
    
    // Insert some n-grams
    trie.insert(&[1, 2, 3]);
    trie.insert(&[1, 2, 3]);
    trie.insert(&[1, 2, 4]);
    trie.insert(&[2, 3, 4]);
    
    // Search for n-grams
    println!("Count for [1, 2, 3]: {}", trie.search(&[1, 2, 3]));
    println!("Count for [1, 2, 4]: {}", trie.search(&[1, 2, 4]));
    println!("Count for [2, 3, 4]: {}", trie.search(&[2, 3, 4]));
    println!("Count for [3, 4, 5]: {}", trie.search(&[3, 4, 5]));

}
