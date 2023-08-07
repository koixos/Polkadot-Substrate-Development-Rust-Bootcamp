use std::{collections::HashMap, f32::consts::PI};

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn print_enum(msg: &Message) {
    match msg {
        Message::Quit => println!("I quit!"),
        Message::Move { x, y } => println!("{}, {}", x, y),
        Message::Write(str) => println!("{}", str),
        Message::ChangeColor(r, g, b) => println!("{r}, {g}, {b}"),
    }
}

/* HW */
fn hello(name: &str) -> String {
    format!("Hello {name}!")
}

fn make_double(num: i32) -> i32 {
    num*2
}

fn multiply_w_pi(num: f32) -> f32 {
    num*PI
}

fn main() {
    let mut _a = 1;
    let b = 1i32;
    let s = "test";
    _a = 5;
    let sj: String = String::from("eheh");
    println!("Hello world!\n{} + {} = {}", make_double(_a), b, _a+b);

    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 1, y: 2};
    let msg_write = Message::Write(String::from("Writing"));
    let msg_color = Message::ChangeColor(255, 0, 0);

    print_enum(&msg_quit);
    print_enum(&msg_move);
    print_enum(&msg_write);
    print_enum(&msg_color);

    let arr: [i32; 6] = [10,20,30,40,50,60]; //herhangi bir elemanı sonradan değiştiremezsin çünkü immut
    println!("array: {arr:?}");

    let slice1: &[i32] = &arr[2..4];
    println!("slice-1: {slice1:?}");

    let mut mut_arr = [1, 2, 3];
    mut_arr[1] = 5;
    println!("mut_arr: {mut_arr:?}");

    let mut vec = Vec::from([1,2,3]); //dinamik dizi
    vec.push(4);

    let mut mac_vec = vec![0; 3]; // vec! = macro vec, mac_vec = [0, 0, 0]
    mac_vec.pop();

    let mut hm:HashMap<i32, &str> = HashMap::new();
    hm.insert(42, "masa");
    hm.insert(16, "sandalye");
    println!("{:?}", hm);

    let a = 1;

    if a == 1 {
        println!("a == 1");
    } else {
        println!("a != 1");
    }

    let opt: Option<i32> = Some(65);

    if let Some(value) = opt {
        println!("opt has a value: {value}");
    } else {
        println!("opt does not have a value");
    }

    let b = 5;
    match b {
        0..=4 => println!("0 <= b <= 4"),
        5..=10 => println!("5 <= b <= 10"),
        _ => () //do nothing
    }

    let mut i = 1;
    while i <= 20 {
        println!("test");
        i += 1
    }

    for b in b..i {
        println!("{b}")
    }

    loop { // loop = while true
        println!("sonsuz");
        i += 1;
        if i > 40 {
            break
        }
    }

    let mut x = vec![3,4,5];
    while let Some(val) = x.pop() {
        println!("val: {val}")
    }

    println!("{}", hello("Zeynep"));
    println!("{}", make_double(5));
    println!("{}", multiply_w_pi(3.2));
}