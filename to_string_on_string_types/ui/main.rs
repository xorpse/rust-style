fn main() {
    let a = "hello".to_string();
    let b = "hello";
    let c = &&b;

    let d = c.to_string();
    let e = d.to_string();

    let f = c.to_owned();
    let g = d.to_owned();
}
