#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32),
    Move(u64, u64),
    Echo(String),
    ChangeColor { r: u8, g: u8, b: u8 },
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize(43));
    println!("{:?}", Message::Move(21, 3));
    println!("{:?}", Message::Echo(String::from("asdsadas")));
    println!("{:?}", Message::ChangeColor { r: 32, g: 32, b: 1 });
    println!("{:?}", Message::Quit);
}
