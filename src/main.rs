use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

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

    fn visualize_to_png(&self, filename: &str) {
        let (graph, _) = self.create_graph();
        let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        let dot_file = format!("{}.dot", filename);
        let png_file = format!("{}.png", filename);

        // Write DOT content to file
        let mut file = File::create(&dot_file).expect("Failed to create DOT file");
        file.write_all(dot.as_bytes()).expect("Failed to write DOT content");

        // Convert DOT to PNG using Graphviz
        Command::new("dot")
            .args(&["-Tpng", &dot_file, "-o", &png_file])
            .output()
            .expect("Failed to execute Graphviz. Make sure it's installed and in your PATH.");

        println!("PNG visualization saved as {}", png_file);
    }

    fn create_graph(&self) -> (Graph<String, String>, NodeIndex) {
        let mut graph = Graph::new();
        let root = graph.add_node("root".to_string());
        self.add_node_to_graph(&self.root, &mut graph, root, Vec::new());
        (graph, root)
    }

    fn add_node_to_graph(&self, node: &TrieNode, graph: &mut Graph<String, String>, parent: NodeIndex, path: Vec<i32>) {
        for (&token, child) in &node.children {
            let mut new_path = path.clone();
            new_path.push(token);
            let label = if new_path.len() == self.ngram_length {
                format!("{} ({})", token, child.count)
            } else {
                token.to_string()
            };
            let child_node = graph.add_node(label);
            graph.add_edge(parent, child_node, "".to_string());
            self.add_node_to_graph(child, graph, child_node, new_path);
        }
    }
}

fn main() {
    let mut trie = NGramTrie::new(4);
    
    // Insert some n-grams with depth 4 and more branches
    trie.insert(&[1, 2, 3, 4]);
    trie.insert(&[1, 2, 3, 4]);
    trie.insert(&[1, 2, 3, 5]);
    trie.insert(&[1, 2, 4, 5]);
    trie.insert(&[2, 3, 4, 5]);
    trie.insert(&[3, 4, 5, 6]);
    trie.insert(&[4, 5, 6, 7]);
    trie.insert(&[1, 3, 5, 7]);
    trie.insert(&[2, 4, 6, 8]);
    trie.insert(&[1, 2, 3, 4]);
    
    // Generate PNG visualization
    trie.visualize_to_png("ngram_trie");
    
    // Search for n-grams
    println!("Count for [1, 2, 3, 4]: {}", trie.search(&[1, 2, 3, 4]));
    println!("Count for [1, 2, 3, 5]: {}", trie.search(&[1, 2, 3, 5]));
    println!("Count for [1, 2, 4, 5]: {}", trie.search(&[1, 2, 4, 5]));
    println!("Count for [2, 3, 4, 5]: {}", trie.search(&[2, 3, 4, 5]));
    println!("Count for [3, 4, 5, 6]: {}", trie.search(&[3, 4, 5, 6]));
    println!("Count for [4, 5, 6, 7]: {}", trie.search(&[4, 5, 6, 7]));
    println!("Count for [1, 3, 5, 7]: {}", trie.search(&[1, 3, 5, 7]));
    println!("Count for [2, 4, 6, 8]: {}", trie.search(&[2, 4, 6, 8]));
    println!("Count for [5, 6, 7, 8]: {}", trie.search(&[5, 6, 7, 8]));
}
