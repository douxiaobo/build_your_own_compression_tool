use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

// 节点结构体表示Huffman树中的节点
#[derive(Debug, PartialEq, Eq)]
struct Node {
    character: Option<char>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// 实现PartialOrd和Ord使得Node可以放入BinaryHeap中
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency.cmp(&other.frequency))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

// 构建Huffman树
fn build_huffman_tree(frequencies: HashMap<char, usize>) -> Node {
    let mut priority_queue: BinaryHeap<Reverse<Node>> = frequencies.into_iter()
        .map(|(character, frequency)| Reverse(Node {
            character: Some(character),
            frequency,
            left: None,
            right: None,
        }))
        .collect();

    while priority_queue.len() > 1 {
        let Reverse(left) = priority_queue.pop().unwrap();
        let Reverse(right) = priority_queue.pop().unwrap();

        let combined_node = Node {
            character: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        priority_queue.push(Reverse(combined_node));
    }

    priority_queue.pop().unwrap().0 // 取出根节点
}

// 生成Huffman编码
fn generate_huffman_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(character) = node.character {
        codes.insert(character, prefix);
    } else {
        if let Some(ref left) = node.left {
            generate_huffman_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            generate_huffman_codes(right, format!("{}1", prefix), codes);
        }
    }
}

// 对文本进行编码
fn encode(text: &str, codes: &HashMap<char, String>) -> String {
    text.chars().map(|c| codes.get(&c).unwrap().clone()).collect()
}

// 对编码进行解码
fn decode(encoded_text: &str, root: &Node) -> String {
    let mut node = root;
    let mut decoded_text = String::new();
    for bit in encoded_text.chars() {
        node = match bit {
            '0' => node.left.as_ref().unwrap(),
            '1' => node.right.as_ref().unwrap(),
            _ => panic!("Invalid bit"),
        };
        if node.character.is_some() {
            decoded_text.push(node.character.unwrap());
            node = root;
        }
    }
    decoded_text
}

fn main() {
    let text = "this is an example of huffman encoding";
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    
    for c in text.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    let huffman_tree = build_huffman_tree(frequencies);
    let mut huffman_codes = HashMap::new();
    generate_huffman_codes(&huffman_tree, String::new(), &mut huffman_codes);

    println!("Huffman Codes: {:?}", huffman_codes);

    let encoded_text = encode(text, &huffman_codes);
    println!("Encoded Text: {}", encoded_text);

    let decoded_text = decode(&encoded_text, &huffman_tree);
    println!("Decoded Text: {}", decoded_text);
}

// Huffman Codes: {
//     'c': "00111", 'x': "01110", 'u': "10000", 'h': "0100", 'a': "1100", 'l': "01011", 
//     's': "0110", 'g': "10001", 'f': "1010", 'e': "1011", ' ': "111", 'm': "0010", 
//     'n': "000", 't': "01111", 'i': "1101", 'p': "01010", 'o': "1001", 'd': "00110"}
// Encoded Text: 011110100110101101111101011011111000001111011011101100001001010010111011111100110101110100100001010101000101100000111101100000111100100110110100010001
// Decoded Text: this is an example of huffman encoding