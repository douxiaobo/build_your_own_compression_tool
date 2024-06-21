fn main() {
    let string="011110100110101101111101011011111000001111011011101100001001010010111011111100110101110100100001010101000101100000111101100000111100100110110100010001";
    let contents=std::fs::read_to_string("./test.huffman").expect("Unable to read file");
    if contents==string {
        println!("Equal");
    } else {
        println!("Not Equal");
    }
}