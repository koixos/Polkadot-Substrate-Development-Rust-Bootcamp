use core::{fmt, num};
use std::net::UdpSocket;
use std::io;

enum IpAddr {
    V4(String),
    V6(String)
}

struct Point {
    x: i64,
    y: i64
}

struct OrnekTuple(i64, i64);

struct NoData;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn yas_kac(&self) -> u8 { self.age }
    fn yas_ayarla(&mut self, yas: u8) { self.age = yas }

    fn new(name: String, age: u8) -> Self {
        Self {name, age}
    }
}

//interface = trait
trait Speaker {
    fn say_name(&self) -> String;
    fn say_age(&self) -> String;
}

impl Speaker for Person {
    fn say_name(&self) -> String { format!("My name is {}", self.name) } 
    fn say_age(&self) -> String { format!("My age is {}", self.age) }
}

fn say_your_name(speaker: &impl Speaker) {
    println!("{}", speaker.say_name())
}

/*
    # veya generic tip kullanarak:

    fn say_your_name<T: Speaker>(speaker: &T) { ... } //bu sayede sadece Speaker türü içeri girecek

    # veya run-time kontrollü &dyn kullanarak:

    fn say_your_name(speaker: &dyn Speaker) { ... } //prog boyutunu küçültür ama perf yavaşlar
*/

fn say_your_age(speaker: &impl Speaker) {
    println!("{}", speaker.say_age())
}

/*
pub enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.age)
    }
}

// attribute macros:
#[cfg(target_os = "linux")]
fn sadece_linuxta() { }

#[test]
fn test_fonk() { }

/*
#[get("/")] //rocket
fn index() -> &'static str {
    "Hello world!"
}
*/

// HW

fn to_letter_grade(num: u8) -> &'static str {
    match num {
        90..=100 => "AA",
        80..=89 => "BA",
        70..=79 => "BB",
        60..=69 => "CB",
        50..=59 => "CC",
        40..=49 => "DD",
        _ => "FF"
    }
}

enum LogLevel {
    Warning,
    Info,
    Error
}

fn log(level: LogLevel, msg: &str) -> String {
    match level {
        LogLevel::Warning => format!("[WARN]: {}", msg),
        LogLevel::Info => format!("[INFO]: {}", msg),
        LogLevel::Error => format!("[ERROR]: {}", msg)
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Unidentified
}

struct PersonInfo {
    Name: String,
    Age: u8,
    Gender: Gender
}

impl fmt::Display for PersonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} - {:?}", self.Name, self.Age, self.Gender)
    }
}

/*  # üstteki Display impl ile çakıştığı için yoruma alındı
macro_rules! display {
    ($struct_expr: ty, $format_string: literal, $property: ident) => {
        impl std::fmt::Display for $struct_expr {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $format_string, self.$property)
            }
        }
    };
}

display!(PersonInfo, "{}", Name);
*/

fn main() {
    let v = IpAddr::V6(String::from("::1"));

    let ip_str = match v {
        IpAddr::V4(s) => s,
        IpAddr::V6(s) => s
    };

    println!("{}", ip_str);

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);

    let y: Result<i32, &str> = Err("Error!");
    assert_eq!(y.is_err(), true);

    match x {
        Ok(i) => assert_eq!(i, -3),
        Err(s) => println!("{s}")
    }

    //result exp
    let socket: Result<UdpSocket, io::Error> = UdpSocket::bind("127.0.0.1:1453");

    let socket:UdpSocket = match socket {
        Ok(sock) => sock,
        Err(err) => panic!("Binding error: {}", err)
    };

    // let socket:UdpSocket = socket.unwrap(); // Ok() ise socket'i döndürür, Err() ise panic!

    // let socket:UdpSocket = socket.expect("special message");

    let p = Point { x: 1, y: 2 };

    let Point {x, y} = p;

    x; //kullanılabilir
    y;

    let mut p = Person::new(String::from("Zeynep"), 21);
    p.yas_ayarla(22);
    println!("{:?}", p); //bunu kullanabilmek için 18.satırdaki kodu eklemeliyiz

    println!("{}", p); // fmt::Display sayesinde, çıktıyı istediğin gibi özelleştirebilirsin

    println!("Harf notu: {}", to_letter_grade(78));

    println!("{}", log(LogLevel::Error, "test"));

    let person = PersonInfo {
        Name: String::from("Talha Kaymak"),
        Age: 21,
        Gender: Gender::Male
    };

    println!("{person}");

    let a = 45; // a declared here
    println!("{a}");

    let a = a + 1; //declaring a new variable with the same name "a" --> called "shadowing"
    println!("{a}");

    let numbers: [i32; 5] = [1, 34, 54, 23, 56];
    for number in numbers {
        println!("number: {}", number);
    }

    let mut counter = 0;
    loop {
        if counter == 6 {
            break;
        }
        println!("{counter}");
        counter += 1;
    }

    let a = 4;
    let result = match a {
        1 => "1",
        5 => "5",
        3 => "3",
        _ => "_",
    };
    println!("Result is {}", result);
    
}

/*  result easy syntax

    # ?'ini Result döndüren fonk.larda kısaltma olarak kullanabiliriz.
    # ? ile değeri elde edilen Result, eğer Err ise ? bulunduğu fonk.u return Err(...) çalıştırarak sonlandırır.

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:1453")?;

    let mut buf = [0; 1024];
    let (len, sender) = socket.recv_from(&mut buf)?; // ? ile error handling sağlanmış oluyor

    Ok(())
}
----------
fn main() {
    let x = Some(-3);
    assert_eq!(x.unwrap(), -3); //sorun yok, çünkü x NULL değil

    let y = None;
    assert_eq!(x.unwrap(), -3); //panic!

    match y {
        Some(i) => assert_;eq!(i, -3),
        None => println!("ERROR: No Value Exists")  //error handling
    }
    
    // YA DA
    
    if let Some(i) = x { //bu sayede None değer if'e girmeyecek
        assert_eq!(i, -3)
    }
}
*/