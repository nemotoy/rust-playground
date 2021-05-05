fn main() {
    another_function(five(), 6);

    let a = [1, 2, 3, 4, 5];
    for ele in a.iter() {
        println!("The value is: {}", ele);
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
