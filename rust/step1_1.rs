use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
fn main(){
    let path=Path::new("../135-0.txt");
    let file=File::open(path).expect("Failed to open file");
    let reader=BufReader::new(file);
    let mut character:HashMap<char,u32> = HashMap::new();
    for line in reader.lines(){
        if let Ok(line_contents)=line{
            for ch in line_contents.chars(){
                if character.contains_key(&ch){
                    character.insert(ch,character.get(&ch).unwrap()+1);
                }else{
                    character.insert(ch,1);
                }
            }
        }    
    }
    println!("X: {},t: {}",character[&'X'],character[&'t']);
}

