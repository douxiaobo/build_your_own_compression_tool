// baidu

use std::collections::BinaryHeap;  
use std::collections::HashMap;  
use std::cmp::Reverse;  

#[warn(dead_code)]
#[derive(Debug, Clone)]  
struct HuffmanNode {  
    weight: usize,  
    symbol: Option<char>,  
    left: Option<Box<HuffmanNode>>,  
    right: Option<Box<HuffmanNode>>,  
}  
  
impl HuffmanNode {  
    fn new(weight: usize, symbol: Option<char>) -> Self {  
        HuffmanNode {  
            weight,  
            symbol,  
            left: None,  
            right: None,  
        }  
    }  
}  
  
// 用于堆排序的节点，包含HuffmanNode和它的权重  
struct PriorityQueueNode(HuffmanNode, usize);  
  
impl PartialOrd for PriorityQueueNode {  
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {  
        self.1.partial_cmp(&other.1)  
    }  
}  
  
impl Ord for PriorityQueueNode {  
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {  
        self.1.cmp(&other.1)  
    }  
}  
  
impl Eq for PriorityQueueNode {}  

impl PartialEq for PriorityQueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
  
// 构建Huffman树  
fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> HuffmanNode {  
    let mut heap = BinaryHeap::new();  
  
    // 将所有字符和频率添加到堆中  
    for (&symbol, &weight) in frequencies.iter() {  
        heap.push(Reverse(PriorityQueueNode(HuffmanNode::new(weight, Some(symbol)), weight)));  
    }  
  
    // 构建Huffman树  
    while heap.len() > 1 {  
        let node1 = heap.pop().unwrap().0;  
        let node2 = heap.pop().unwrap().0;  
  
        let mut parent = HuffmanNode::new(node1.0.weight + node2.0.weight, None);  
        parent.left = Some(Box::new(node1.0));  
        parent.right = Some(Box::new(node2.0));  
  
        heap.push(Reverse(PriorityQueueNode(parent.clone(), parent.weight)));  
    }  
  
    let Reverse(PriorityQueueNode(huffman_node, _)) = heap.pop().unwrap();

    huffman_node
}  
  
// ... 这里省略了编码和解码函数的实现 ...  
  
fn main() {  
    // 示例：构建Huffman树  
    let frequencies = HashMap::from([  
        ('a', 45),  
        ('b', 13),  
        ('c', 12),  
        ('d', 16),  
        ('e', 9),  
        ('f', 5),  
    ]);  
  
    let huffman_tree = build_huffman_tree(&frequencies);  
    println!("{:?}", huffman_tree);  
  
    // ... 这里可以继续实现编码和解码函数，并使用huffman_tree ...  
}


// HuffmanNode { 
//     weight: 100, 
//     symbol: None, 
//     left: Some(
//         HuffmanNode { 
//             weight: 45, 
//             symbol: Some('a'), 
//             left: None, 
//             right: None 
//         }
//     ), 
//     right: Some(
//         HuffmanNode { 
//             weight: 55, 
//             symbol: None, 
//             left: Some(
//                 HuffmanNode { 
//                     weight: 25, 
//                     symbol: None, 
//                     left: Some(
//                         HuffmanNode { 
//                             weight: 12, 
//                             symbol: Some('c'), 
//                             left: None, right: None 
//                         }
//                     ), 
//                     right: Some(
//                         HuffmanNode { 
//                             weight: 13, 
//                             symbol: Some('b'), 
//                             left: None, 
//                             right: None 
//                         }
//                     ) 
//                 }
//             ), 
//             right: Some(
//                 HuffmanNode { 
//                     weight: 30, 
//                     symbol: None, 
//                     left: Some(
//                         HuffmanNode { 
//                             weight: 14, 
//                             symbol: None, 
//                             left: Some(
//                                 HuffmanNode { 
//                                     weight: 5, 
//                                     symbol: Some('f'), 
//                                     left: None, 
//                                     right: None 
//                                 }
//                             ), 
//                             right: Some(
//                                 HuffmanNode { 
//                                     weight: 9, 
//                                     symbol: Some('e'), 
//                                     left: None, 
//                                     right: None 
//                                 }
//                             ) 
//                         }
//                     ), 
//                     right: Some(
//                         HuffmanNode { 
//                             weight: 16, 
//                             symbol: Some('d'), 
//                             left: None, 
//                             right: None 
//                         }
//                     ) 
//                 }
//             ) 
//         }
//     ) 
// }