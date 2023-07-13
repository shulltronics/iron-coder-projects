mod system;

fn main() {
    println!("Hello, world!");
    let s = system::System::new();
    println!("{}", s.name);
}
