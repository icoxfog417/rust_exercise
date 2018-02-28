fn main() {
    let hello = "Hellow"; // it is &str
    let mut world = String::new();
    let world_chars: &[char] = &['W', 'o', 'r', 'l', 'd'];

    for c in world_chars {
        world.push(*c);
    }

    // Comment is written by double slash
    println!("{} {}", hello, world);
}
