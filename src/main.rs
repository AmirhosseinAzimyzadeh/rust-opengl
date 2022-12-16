use glium::Api::Gl;

fn main() {
    let mut my_str = String::new();
    my_str.push_str("Hello World");
    my_str.split_whitespace().for_each(printer);
    println!("{}", my_str);
}


fn printer(x: &str) {
    println!("your str is:{}", x);
}