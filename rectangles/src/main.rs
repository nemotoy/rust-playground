fn main() {
    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}
