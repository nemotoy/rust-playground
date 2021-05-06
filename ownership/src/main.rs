fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    println!("{}", x); // i32はCopyなので、xを使用できる。
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringがスコープを抜け、`drop`が呼ばれる。メモリが解放される。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integerがスコープを抜ける。
