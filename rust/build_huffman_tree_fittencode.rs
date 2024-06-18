use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    data: Option<char>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

fn build_tree(freq_map: &HashMap<char, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (char, freq) in freq_map {
        heap.push(Box::new(Node {
            data: Some(*char),
            frequency: *freq,
            left: None,
            right: None,
        }));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let parent_freq = left.frequency + right.frequency;

        let parent = Node {
            data: None,
            frequency: parent_freq,
            left: Some(left),
            right: Some(right),
        };

        heap.push(Box::new(parent));
    }

    heap.pop()
}

fn build_codes(node: Option<Box<Node>>, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(n) = node {
        if let Some(c) = n.data {
            codes.insert(c, prefix);
        } else {
            build_codes(n.left, prefix.clone() + "0", codes);
            build_codes(n.right, prefix + "1", codes);
        }
    }
}

fn huffman_encode(input: &str) -> (String, HashMap<char, String>) {
    let mut freq_map = HashMap::new();
    for c in input.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let tree = build_tree(&freq_map).unwrap();

    let mut codes = HashMap::new();
    build_codes(Some(tree), String::new(), &mut codes);

    let encoded: String = input.chars().map(|c| codes[&c].to_string()).collect();

    (encoded, codes)
}

fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String {
    let mut result = String::new();
    let mut current_code = String::new();

    for bit in encoded.chars() {
        current_code.push(bit);
        for (char, code) in codes {
            if code == &current_code {
                result.push(*char);
                current_code.clear();
                break;
            }
        }
    }

    result
}

fn main() {
    let input = "abracadabra";
    let (encoded, codes) = huffman_encode(input);
    let decoded = huffman_decode(&encoded, &codes);
    println!("Input: {}", input);
    println!("Encoded: {}", encoded);
    println!("Decoded: {}", decoded);
}

// Input: abracadabra
// Encoded: 01101110100010101101110
// Decoded: abracadabra