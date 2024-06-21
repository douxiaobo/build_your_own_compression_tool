use std::env;
use std::fs;
use std::collections::HashMap;
// use std::collections::BinaryHeap;
// use std::cmp::Reverse;
// use std::path::Path;

#[derive(Debug)]
struct HuffmanNode {
    ch: Option<char>,
    freq: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(ch: char, freq: usize) -> HuffmanNode {
        HuffmanNode {
            ch:Some(ch),
            freq,
            left: None,
            right: None,
        }
    }

    fn serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(b"{");
        if let Some(ch) = self.ch {
            out.extend(format!("\"ch\":\"{}\",", ch).as_bytes());
        }
        out.extend_from_slice(b"\"freq\":");
        out.extend_from_slice(&format!("{}", self.freq).as_bytes());
        out.extend_from_slice(b",\"left\":");
        self.left.as_ref().map(|node| node.serialize(out));
        out.extend_from_slice(b",\"right\":");
        self.right.as_ref().map(|node| node.serialize(out));
        out.extend_from_slice(b"}");
    }

    fn deserialize(mut bytes: &[u8]) -> Result<Self, &'static str> {
        if !bytes.starts_with(b"{") || !bytes.ends_with(b"}") {
            return Err("Invalid node format");
        }
        // bytes = &bytes[1..bytes.len()-1]; // remove curly braces
        // 修正为使用切片来跳过处理过的数据
        bytes = &bytes[Self::right_after_value(bytes)..];

        let mut ch: Option<char> = None;
        let mut freq: usize = 0;
        let mut left: Option<Box<HuffmanNode>> = None;
        let mut right: Option<Box<HuffmanNode>> = None;

        let mut key = String::new();
        let mut value = String::new();
        let mut parsing_key = true;
        let mut parsing_value = false;
        let mut in_quotes = false;

        for byte in bytes {
            match byte {
                b',' | b':' | b'}' => {
                    if !in_quotes {
                        if parsing_key {
                            parsing_key = false;
                            parsing_value = true;
                        } else {
                            parsing_key = true;
                            parsing_value = false;
                        }
                    }
                },
                b'"' => in_quotes = !in_quotes,
                _ if parsing_key => key.push(char::from(*byte)),
                _ if parsing_value => value.push(char::from(*byte)),
                _ => (), // ignore other characters
            }

            if parsing_key && key == "ch" {
                parsing_key = false;
                parsing_value = true;
                key.clear();
            } else if parsing_key && key == "freq" {
                parsing_key = false;
                parsing_value = true;
                key.clear();
            } else if parsing_key && key == "left" {
                parsing_key = false;
                parsing_value = true;
                key.clear();
                left = Some(Box::new(HuffmanNode::deserialize(bytes)?));
                bytes = &bytes[Self::right_after_value(bytes)];
            } else if parsing_key && key == "right" {
                parsing_key = false;
                parsing_value = true;
                key.clear();
                right = Some(Box::new(HuffmanNode::deserialize(bytes)?));
                bytes = &bytes[Self::right_after_value(bytes)];
            } else if parsing_value && value.parse::<usize>().is_ok() {
                parsing_key = true;
                parsing_value = false;
                freq = value.parse::<usize>().unwrap();
                value.clear();
            } else if parsing_value && ch.is_none() {
                parsing_key = true;
                parsing_value = false;
                ch = Some(value.parse::<char>().unwrap());
                value.clear();
            }
        }

        Ok(HuffmanNode { ch, freq, left, right })
    }

    fn right_after_value(bytes: &[u8]) -> usize {
        for (i, byte) in bytes.iter().enumerate() {
            if *byte == b',' || *byte == b'}' {
                return i+1;
            }
        }
        bytes.len()
    }
    fn left_after_value(bytes: &[u8]) -> usize {
        for (i, byte) in bytes.iter().enumerate() {
            if *byte == b',' {
                return i;
            }
        }
        bytes.len()
    }
}

// 序列化函数现在正确地使用了HuffmanNode类型
fn serialize_tree(root: &HuffmanNode) -> Vec<u8> {
    let mut vec = Vec::new();
    root.serialize(&mut vec);
    vec
}

fn main(){
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() <2 {
        println!("Usage: {} <input_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    // let output_file = &args[2];

    let mut file_name=String::new();
    for ch in input_file.chars() {
        if ch=='/' {
            file_name.clear();
        } else {
            file_name.push(ch);
        }
    }

    let dir_name=String::from(&file_name[0..file_name.rfind('.').unwrap()]);
    fs::create_dir_all(dir_name.clone()).expect("Error creating directory");

    // 读取输入文件
    let contents = std::fs::read_to_string(input_file).expect("Error reading input file");
    if is_binary(&contents){
        let loaded_tree = fs::read_to_string("huffman_tree.txt").expect("Error reading tree file");
        let deserialized_root = HuffmanNode::deserialize(loaded_tree.as_bytes()).expect("Error deserializing tree");
        let contents=decode(&contents,&deserialized_root);
        let output_file = format!("{0}/{0}.decoded", dir_name.clone());
        std::fs::write(output_file, contents).expect("Error writing output file");
    }else{
        // 构建 hashmap
        let hashmap=build_hashmap(&contents);
        // 构建 Huffman 树
        let frequencies = build_huffman_tree(&hashmap);
        // 构建 Huffman 编码
        let codes = build_huffman_codes(&frequencies);
        // 编码并写入输出文件
        let encoded_contents = encode_contents(&contents, &codes);

        let output_file = format!("{0}/{0}.huffman", dir_name.clone());
        std::fs::write(output_file, encoded_contents).expect("Error writing output file");

        let serialized=serialize_tree(&frequencies);
        fs::write(dir_name.clone()+".huffman.tree",serialized).expect("Error writing tree file")
    }
}

fn is_binary(contents: &str) -> bool {
    for ch in contents.chars() {
        if ch!= '0' && ch!= '1' {
            return false;
        }
    }
    true
}

fn build_hashmap(contents: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    for ch in contents.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    // for (ch, freq) in frequencies.iter_mut() {
    //     println!("char: {}, freq: {}", ch, freq)
    // }
    frequencies
}

fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<HuffmanNode> {
    let mut nodes = frequencies.into_iter().map(|(ch, freq)| HuffmanNode::new(*ch, *freq)).collect::<Vec<_>>();
    nodes.sort_by_key(|node| node.freq);

    while nodes.len() > 1 {
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let parent = HuffmanNode {
            ch: Some('\0'),
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        nodes.push(*Box::new(parent));
        nodes.sort_by_key(|node| node.freq);
    }

    Box::new(nodes.into_iter().next().unwrap())
}

fn build_huffman_codes(root: &Box<HuffmanNode>) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    let mut current_code = String::new();
    build_huffman_codes_recursive(root, &mut codes, &mut current_code);
    for (ch, code) in codes.iter_mut() {
        println!("ch: {}, code: {}",ch,code);
    }
    codes
}

fn build_huffman_codes_recursive(node: &HuffmanNode, codes: &mut HashMap<char, String>, current_code: &mut String) {
    if node.ch!= Some('\0')  {
        codes.insert(node.ch.expect("Invalid node"), current_code.clone());
        return;
    }

    current_code.push('0');
    if let Some(ref left_node) = node.left {
        build_huffman_codes_recursive(left_node, codes, current_code);
    }
    current_code.pop();

    current_code.push('1');
    if let Some(ref right_node) = node.right {
        build_huffman_codes_recursive(right_node, codes, current_code);
    }
    current_code.pop();
}

fn encode_contents(contents: &str, codes: &HashMap<char, String>) -> String {
    let mut encoded_contents = String::new();
    for ch in contents.chars() {
        encoded_contents.push_str(&codes[&ch]);
    }
    encoded_contents
}

fn decode(encoded_text: &str, root: &HuffmanNode) -> String {
    let mut node = root;
    let mut decoded_text = String::new();
    for bit in encoded_text.chars() {
        node = match bit {
            '0' => node.left.as_ref().unwrap(),
            '1' => node.right.as_ref().unwrap(),
            _ => panic!("Invalid bit"),
        };
        if node.ch.is_some() {
            decoded_text.push(node.ch.unwrap());
            node = root;
        }
    }
    decoded_text
}





