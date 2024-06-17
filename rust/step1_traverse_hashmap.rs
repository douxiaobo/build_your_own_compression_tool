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
    // the first
    // for (key,value) in character.iter(){
    //     println!("Key: {}, Value: {}",key,value);
    // }
    
    // the second
    // for (key,value) in character.into_iter(){
    //     println!("Key: {}, Value: {}",key,value);
    // }

    // the third
    // for (key,value) in character.drain(){
    //     println!("Key: {}, Value: {}",key,value);
    // }
}

// Key: ”, Value: 7048
// Key: $, Value: 2
// Key: }, Value: 1
// Key: ], Value: 164
// Key: |, Value: 27
// Key: l, Value: 97113
// Key: 6, Value: 142
// Key: “, Value: 7119
// Key: A, Value: 7369
// Key: ù, Value: 18
// Key: u, Value: 67391
// Key: Z, Value: 60
// Key: Œ, Value: 3
// Key: N, Value: 3060
// Key: n, Value: 166862
// Key: ., Value: 30480
// Key: 2, Value: 227
// Key: o, Value: 180974
// Key: P, Value: 3426
// Key: f, Value: 53421
// Key: K, Value: 320
// Key: {, Value: 1
// Key: L, Value: 2454
// Key: p, Value: 39940
// Key: D, Value: 1758
// Key: /, Value: 6
// Key: ), Value: 170
// Key: V, Value: 2211
// Key: ,, Value: 48780
// Key: è, Value: 293
// Key: Ç, Value: 2
// Key: 3, Value: 239
// Key: æ, Value: 106
// Key: …, Value: 1
// Key: ﻿, Value: 1
// Key: *, Value: 169
// Key: 0, Value: 205
// Key: ç, Value: 48
// Key: J, Value: 2447
// Key: a, Value: 199732
// Key: Q, Value: 135
// Key: È, Value: 6
// Key: I, Value: 10109
// Key: 7, Value: 132
// Key: (, Value: 170
// Key: î, Value: 39
// Key: ½, Value: 1
// Key: j, Value: 3391
// Key: &, Value: 1
// Key: C, Value: 4591
// Key: ï, Value: 18
// Key: +, Value: 5
// Key: i, Value: 165211
// Key: -, Value: 4700
// Key: g, Value: 46029
// Key: s, Value: 157632
// Key: E, Value: 4939
// Key: ;, Value: 5885
// Key: à, Value: 59
// Key: ", Value: 22
// Key: H, Value: 6603
// Key: v, Value: 24047
// Key: â, Value: 56
// Key: é, Value: 1327
// Key: G, Value: 2514
// Key: 1, Value: 693
// Key: ô, Value: 34
// Key: ë, Value: 5
// Key: œ, Value: 35
// Key: z, Value: 1845
// Key: R, Value: 3541
// Key: #, Value: 1
// Key: q, Value: 2398
// Key: [, Value: 164
// Key: !, Value: 3539
// Key: Æ, Value: 10
// Key: c, Value: 62741
// Key: y, Value: 37944
// Key: B, Value: 3194
// Key: 9, Value: 114
// Key: 5, Value: 190
// Key: —, Value: 3144
// Key: k, Value: 14107
// Key: ', Value: 7
// Key: W, Value: 3177
// Key: 4, Value: 160
// Key: w, Value: 53330
// Key: h, Value: 170220
// Key: x, Value: 3693
// Key: ?, Value: 2976
// Key: 8, Value: 404
// Key: :, Value: 2501
// Key: U, Value: 904
// Key: b, Value: 34267
// Key: Ü, Value: 2
// Key: _, Value: 2070
// Key: ü, Value: 37
// Key: m, Value: 56000
// Key:  , Value: 516353
// Key: T, Value: 12571
// Key: Y, Value: 1239
// Key: %, Value: 1
// Key: ê, Value: 74
// Key: O, Value: 3411
// Key: r, Value: 145091
// Key: t, Value: 223000
// Key: S, Value: 4411
// Key: F, Value: 2766
// Key: M, Value: 6212
// Key: ‘, Value: 224
// Key: ñ, Value: 2
// Key: X, Value: 333
// Key: À, Value: 5
// Key: d, Value: 106966
// Key: ’, Value: 3707
// Key: e, Value: 325664
// Key: É, Value: 146
// Key: û, Value: 9
