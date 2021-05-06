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
        *i *=2;
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

}
