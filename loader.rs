// use std::fs::File;
use std::fs;
// use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

fn main() {
    let lines = fs::read_to_string("input.txt");
    println!("{:?}", lines)

    // input.txtファイルを開く
    // let file = match File::open("input.txt") {
    //     Ok(file) => file,
    //     Err(e) => panic!("Failed to open file: {}", e),
    // };
    // let reader = BufReader::new(file);

    // HashMapを初期化
    // let mut input_dict = HashMap::new();

    // ファイルを行ごとに読み込み、HashMapに格納する
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let parts: Vec<&str> = line.splitn(2, '=').collect(); // "="で文字列を2つに分割
    //
    //     if parts.len() == 2 {
    //         let key = parts[0].trim();
    //         let value = parts[1].trim();
    //
    //         input_dict.insert(key.to_string(), value.to_string());
    //     }
    // }

    // HashMapの値を表示する
    // println!("{:?}", input_dict);
}
