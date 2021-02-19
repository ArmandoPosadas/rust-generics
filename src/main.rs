use std::collections::VecDeque;

struct GenericStruct<T>(T);

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Self { item }
    }
}

impl Container<u32> {
    fn sum(item: u32) -> Self {
        Self { item }
    }
}
enum Transmissions<T> {
    Signal(T),
    NoSignal,
}

fn main() {
    println!("Hello, world!");

    println!("{}", give_me("Hola"));
    println!("{}", give_me(2));

    let a: Vec<u8> = Vec::new();

    let mut v2 = Vec::new();

    v2.push("");

    let v3 = Vec::<u8>::new();

    let num_from_str = str::parse::<u8>("34").unwrap();

    println!("{}", num_from_str);

    println!("PRA QIONAAS");
    println!("PRA QIONAAS");
}

fn give_me<T>(value: T) -> T {
    let _ = value;

    value
}
