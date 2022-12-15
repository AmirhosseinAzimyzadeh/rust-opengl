fn main() {
    let mut my_str = String::new();
    my_str.push_str("Hello World");
    my_str.split_whitespace().for_each(|x| {
        println!("{}", x);
    });
    println!("{}", my_str);
}
