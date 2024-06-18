use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    symbol: char,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(symbol: char, frequency: u32) -> Self {
        Node {
            symbol,
            frequency,
            left: None,
            right: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for a min-heap (smaller frequencies come first)
        other.frequency.cmp(&self.frequency)
    }
}

fn build_huffman_tree(symbols: &HashMap<char, u32>) -> Box<Node> {
    let mut heap = BinaryHeap::new();

    for (symbol, frequency) in symbols.iter() {
        heap.push(Box::new(Node::new(*symbol, *frequency)));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let new_node = Box::new(Node {
            symbol: '\0',
            frequency: left.frequency + right.frequency,
            left: Some(left),
            right: Some(right),
        });

        heap.push(new_node);
    }

    heap.pop().unwrap()
}

fn traverse(node: &Node, code: &mut Vec<u8>, codes: &mut HashMap<char, Vec<u8>>) {
    if node.symbol != '\0' {
        codes.insert(node.symbol, code.clone());
        return;
    }

    code.push(0);
    if let Some(left) = &node.left {
        traverse(left, code, codes);
    }

    code.pop();
    code.push(1);
    if let Some(right) = &node.right {
        traverse(right, code, codes);
    }
    code.pop();
}

fn generate_huffman_codes(tree: &Node, codes: &mut HashMap<char, Vec<u8>>) {
    let mut code = Vec::new();
    traverse(tree, &mut code, codes);
}

// fn generate_huffman_codes(tree: &Box<Node>, codes: &mut HashMap<char, Vec<u8>>) {   //不行
//     let mut traverse = |node: &Box<Node>, code: &mut Vec<u8>| {
//         if node.symbol != '\0' {
//             codes.insert(node.symbol, code.clone());
//             return;
//         }

//         code.push(0);
//         traverse(&node.left.as_ref().unwrap(), code);

//         code.pop();
//         code.push(1);
//         traverse(&node.right.as_ref().unwrap(), code);
//     };

//     traverse(tree, &mut Vec::new());
// }

fn encode(text: &str, codes: &HashMap<char, Vec<u8>>) -> Vec<u8> {
    let mut encoded_bits = Vec::new();

    for c in text.chars() {
        encoded_bits.extend(&codes[&c]);
    }

    encoded_bits
}

fn decode(encoded_bits: &[u8], tree: &Box<Node>) -> String {
    let mut current_node = tree;
    let mut decoded_text = String::new();

    for bit in encoded_bits {
        match *bit {
            0 => current_node = &current_node.left.as_ref().unwrap(),
            1 => current_node = &current_node.right.as_ref().unwrap(),
            _ => panic!("Invalid bit"),
        }

        if current_node.symbol != '\0' {
            decoded_text.push(current_node.symbol);
            current_node = tree;
        }
    }

    decoded_text
}

fn main() {
    let text = "This is an example text to be compressed using Huffman coding.";

    let mut symbols = HashMap::new();

    for c in text.chars() {
        *symbols.entry(c).or_insert(0) += 1;
    }

    let tree = build_huffman_tree(&symbols);
    let mut codes = HashMap::new();
    generate_huffman_codes(&tree, &mut codes);

    let encoded_bits = encode(text, &codes);
    println!("Encoded bits: {:?}", encoded_bits);

    let decoded_text = decode(&encoded_bits, &tree);
    println!("Decoded text: {}", decoded_text);
}

// Encoded bits: [0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1]
// Decoded text: This is an example text to be compressed using Huffman coding.
