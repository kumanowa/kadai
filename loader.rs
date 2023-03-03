use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    // config.iniファイルを開く
    let file = File::open("config.ini").unwrap();
    let reader = BufReader::new(file);

    // HashMapを初期化
    let mut config = HashMap::new();

    // ファイルを行ごとに読み込み、HashMapに格納する
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.splitn(2, '=').collect(); // "="で文字列を2つに分割

        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();

            config.insert(key.to_string(), value.to_string());
        }
    }

    // HashMapの値を表示する
    println!("{:?}", config);
}
