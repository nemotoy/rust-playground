use std::collections::HashMap;

fn main() {
    // 空のベクタ
    // let v: Vec<i32> = Vec::new();
    // 初期値を含むベクタ
    let mut v = vec![1, 2, 3];

    println!("collection {:?}", v);

    v.push(4);

    println!("collection {:?}", v);

    let third: &i32 = &v[2];
    // let does_not_exist = &v[100]; // panic

    println!("value of index third {:?}", third);

    let first = v.get(0);

    println!("value of index first {:?}", first);

    for i in &mut v {
        *i *= 2;
    }

    println!("twice collection {:?}", v);

    // === Strings ===

    // s1とs2は等価
    // let data = "initial contents";
    // let s1 = data.to_string();
    // let s2 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("strings: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // println!("strings: {}", s1 + "-" + &s2 + "-" + &s3);
    println!("strings: {}", format!("{}-{}-{}", s1, s2, s3)); // 変数の所有権を奪わない

    // 添え字記法でのアクセス不可
    let s4 = String::from("hello");
    // let h = s4[0];
    println!("strings: {}", &s4[0..1]);

    // === hash-maps ===

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("hash-maps: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("hash-maps: {:?}", scores);

    println!("hash-maps.score: {:?}", scores.get(&String::from("Blue")));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
