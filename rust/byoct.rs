use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
// use std::path::Path;

struct HuffmanNode {
    ch: char,
    freq: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(ch: char, freq: usize) -> HuffmanNode {
        HuffmanNode {
            ch,
            freq,
            left: None,
            right: None,
        }
    }
}

//原来是BinaryHeap，所以要实现Ord和PartialOrd，Eq和PartialEq

use std::cmp::Ordering;

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 由于BinaryHeap默认为最大堆，这里我们反向比较以实现最小堆
        other.freq.cmp(&self.freq)
            .then_with(|| self.ch.cmp(&other.ch)) // 在频率相同时，按字符进行比较作为平局处理
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.ch == other.ch
    }
}

impl Eq for HuffmanNode {}

fn main(){
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     println!("Usage: {} <input_file> <output_file>", args[0]);
    //     return;
    // }
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
    // 使用create_dir_all替代create_dir，这样如果目录已存在也不会抛出错误
    // if let Err(e) = fs::create_dir_all(dir_name.clone()) {
    //     match e.kind() {
    //         std::io::ErrorKind::AlreadyExists => {
    //             // 如果目录已存在，打印提示信息或直接继续执行后续逻辑
    //             println!("Directory '{}' already exists, proceeding...", dir_name);
    //         },
    //         _ => {
    //             // 其他错误则依然panic
    //             panic!("Error creating directory: {}", e);
    //         },
    //     }
    // }
    

    // println!("input:{} output:{} file_name:{} dir_name:{}", input_file, output_file,file_name,dir_name);

    // 读取输入文件
    let contents = std::fs::read_to_string(input_file).expect("Error reading input file");
    if is_binary(&contents){}else{
        // 构建 hashmap
        let hashmap=build_hashmap(&contents);
        // 构建 Huffman 树
        let frequencies = build_huffman_tree(&hashmap);
        // 构建 Huffman 编码
        let codes = build_huffman_codes(&frequencies);
        // 编码并写入输出文件
        let encoded_contents = encode_contents(&contents, &codes);

        let output_file = format!("{}/{}.huffman", dir_name, file_name);
        std::fs::write(output_file, encoded_contents).expect("Error writing output file");
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
    frequencies
}

fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<HuffmanNode> {
    let mut heap = BinaryHeap::new();
    for (ch, freq) in frequencies.iter() {
        // heap.push(Box::new(HuffmanNode {
        //     ch: *ch,
        //     freq: *freq,
        //     left: None,
        //     right: None,
        // }));
        heap.push(Reverse(HuffmanNode::new(*ch, *freq)));

    }

    while heap.len() > 1 {
        let Reverse(left) = heap.pop().unwrap();
        let Reverse(right) = heap.pop().unwrap();
        // if left.freq > right.freq {
        //     std::mem::swap(&mut left, &mut right);
        // }
        let parent = Box::new(HuffmanNode {
            ch: '\0',
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
        // 再次使用Reverse包装后推入堆中
        heap.push(Reverse(*parent));
    }

    // 最终从堆中取出的是Reverse(HuffmanNode)，需要解包
    let Reverse(root) = heap.pop().unwrap();
    Box::new(root) // 返回的是正确的Box<HuffmanNode>
}

fn build_huffman_codes(root: &Box<HuffmanNode>) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    let mut current_code = String::new();
    build_huffman_codes_recursive(root, &mut codes, &mut current_code);
    codes
}

fn build_huffman_codes_recursive(node: &HuffmanNode, codes: &mut HashMap<char, String>, current_code: &mut String) {
    if node.ch!= '\0' {
        codes.insert(node.ch, current_code.clone());
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







