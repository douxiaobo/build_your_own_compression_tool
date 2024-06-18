use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct HuffmanNode {
    frequency: usize,
    character: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(frequency: usize,character: Option<u8>) -> HuffmanNode {
        HuffmanNode {
            frequency:frequency,
            character,
            left: None,
            right: None,
        }
    }
}

fn calculate_frequencies(data: &[u8]) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for &byte in data.iter() {
        *frequencies.entry(byte).or_insert(0) += 1;
    }
    frequencies
}

fn build_huffman_tree(frequencies: &HashMap<u8, usize>) -> HuffmanNode {
    let mut queue = BinaryHeap::new();
    for (&byte, &freq) in frequencies.iter() {
        queue.push(Reverse(HuffmanNode::new(freq, Some(byte))));
    }

    while queue.len() > 1 {
        let node1 = queue.pop().unwrap().0;
        let node2 = queue.pop().unwrap().0;
        let sum = node1.frequency + node2.frequency;
        let parent = HuffmanNode {
            frequency: sum,
            character: None,
            left: Some(Box::new(node1)),
            right: Some(Box::new(node2)),
        };
        queue.push(Reverse(parent));
    }

    queue.pop().unwrap().0
}

fn build_huffman_codes(root: &HuffmanNode, current_code: &str, codes: &mut HashMap<u8, String>) {
    if let Some(left) = &root.left {
        build_huffman_codes(left, &format!("{}0", current_code), codes);
    }
    if let Some(right) = &root.right {
        build_huffman_codes(right, &format!("{}1", current_code), codes);
    }

    if let Some(character) = root.character {
        codes.insert(character, current_code.to_string());
    }
}

fn encode_huffman(data: &[u8]) -> (HashMap<u8, String>, String) {
    let frequencies = calculate_frequencies(data);
    let root = build_huffman_tree(&frequencies);
    let mut codes = HashMap::new();
    build_huffman_codes(&root, "", &mut codes);

    let mut encoded_data = String::new();
    for &byte in data.iter() {
        encoded_data.push_str(&codes[&byte]);
    }

    (codes, encoded_data)
}

fn main() {
    let data = vec![b'a', b'b', b'c', b'a', b'b', b'a', b'c', b'a', b'b', b'c', b'a', b'a'];
    let (codes, encoded_data) = encode_huffman(&data);

    println!("Huffman Codes:");
    for (byte, code) in codes.iter() {
        let char = *byte as char;
        println!("{} ({}): {}", char, byte, code);
    }

    println!("Encoded Data: {}", encoded_data);
}

Huffman Codes:
c (99): 01
a (97): 1
b (98): 00
Encoded Data: 100011001011000111