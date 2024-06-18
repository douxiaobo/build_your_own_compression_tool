use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

// 定义一个结构体来表示树中的节点
#[derive(Debug, Clone, Eq)]
struct Node {
    character: Option<char>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// 实现PartialOrd, PartialEq, Eq, Ord来对节点进行排序
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
            .then_with(|| self.character.cmp(&other.character))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.character == other.character
    }
}

// 构建Huffman树
fn build_huffman_tree(text: &str) -> Node {
    let mut frequency = HashMap::new();
    
    // 计算每个字符的频率
    for c in text.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }

    // 将字符频率转换为优先队列
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    for (char, freq) in frequency {
        heap.push(Node {
            character: Some(char),
            frequency: freq,
            left: None,
            right: None,
        });
    }

    // 构建Huffman树
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let merged = Node {
            character: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        heap.push(merged);
    }

    heap.pop().unwrap()
}

// 生成Huffman编码
fn generate_huffman_codes(node: &Node, codes: &mut HashMap<char, String>, code: String) {
    if let Some(character) = &node.character {
        codes.insert(character.clone(), code.clone());
    }

    if let Some(ref left) = node.left {
        generate_huffman_codes(&left, codes, format!("{}0", code));
    }

    if let Some(ref right) = node.right {
        generate_huffman_codes(&right, codes, format!("{}1", code));
    }
}

fn main() {
    let text = "this is an example of a huffman tree";
    let root = build_huffman_tree(text);
    let mut codes = HashMap::new();
    generate_huffman_codes(&root, &mut codes, String::new());

    // 打印Huffman编码
    for (char, code) in &codes {
        println!("'{}': {}", char, code);
    }
}

// ' ': 111
// 'm': 1001
// 's': 1011
// 'l': 11001
// 'f': 1101
// 'e': 000
// 'u': 01011
// 'i': 0110
// 'a': 001
// 'r': 01000
// 'o': 11000
// 'p': 01001
// 'h': 0111
// 'n': 1000
// 't': 1010
// 'x': 01010