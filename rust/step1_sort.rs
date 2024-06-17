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
    let mut sorted_characters:Vec<(&char, &u32)> = character.iter().collect();
    sorted_characters.sort_by(|a,b|a.1.cmp(b.1));
    for (ch,count) in sorted_characters.iter(){
        println!("{}: {}",ch,count);
    }
}

// …: 1
// : 1
// #: 1
// %: 1
// ½: 1
// }: 1
// {: 1
// &: 1
// Ü: 2
// $: 2
// ñ: 2
// Ç: 2
// Œ: 3
// À: 5
// +: 5
// ë: 5
// /: 6
// È: 6
// ': 7
// û: 9
// Æ: 10
// ï: 18
// ù: 18
// ": 22
// |: 27
// ô: 34
// œ: 35
// ü: 37
// î: 39
// ç: 48
// â: 56
// à: 59
// Z: 60
// ê: 74
// æ: 106
// 9: 114
// 7: 132
// Q: 135
// 6: 142
// É: 146
// 4: 160
// ]: 164
// [: 164
// *: 169
// (: 170
// ): 170
// 5: 190
// 0: 205
// ‘: 224
// 2: 227
// 3: 239
// è: 293
// K: 320
// X: 333
// 8: 404
// 1: 693
// U: 904
// Y: 1239
// é: 1327
// D: 1758
// z: 1845
// _: 2070
// V: 2211
// q: 2398
// J: 2447
// L: 2454
// :: 2501
// G: 2514
// F: 2766
// ?: 2976
// N: 3060
// —: 3144
// W: 3177
// B: 3194
// j: 3391
// O: 3411
// P: 3426
// !: 3539
// R: 3541
// x: 3693
// ’: 3707
// S: 4411
// C: 4591
// -: 4700
// E: 4939
// ;: 5885
// M: 6212
// H: 6603
// ”: 7048
// “: 7119
// A: 7369
// I: 10109
// T: 12571
// k: 14107
// v: 24047
// .: 30480
// b: 34267
// y: 37944
// p: 39940
// g: 46029
// ,: 48780
// w: 53330
// f: 53421
// m: 56000
// c: 62741
// u: 67391
// l: 97113
// d: 106966
// r: 145091
// s: 157632
// i: 165211
// n: 166862
// h: 170220
// o: 180974
// a: 199732
// t: 223000
// e: 325664
//  : 516353