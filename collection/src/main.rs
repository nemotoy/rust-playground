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
}
