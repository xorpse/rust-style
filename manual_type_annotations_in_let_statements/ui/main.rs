use std::collections::HashMap;

fn main() {
    // Integer literals with various types
    let a: usize = 5;
    let b: i32 = -10;
    let c: u8 = 255;
    let d: i64 = 1_000_000;

    // Float literals
    let e: f32 = 3.14;
    let f: f64 = 2.718;

    // Hex/octal/binary literals
    let g: u32 = 0xFF;
    let h: i32 = 0o77;
    let i: u8 = 0b1010;

    // Method call - add turbofish to method
    let y: Vec<usize> = [1, 2, 3, 4, 5].iter().copied().collect();

    // Constructor call - add turbofish to type
    let m: HashMap<String, i32> = HashMap::new();

    // Chained method call - just remove annotation
    let s: String = "hello".to_string();
}
