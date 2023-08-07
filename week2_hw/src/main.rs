fn main() {
    let string1 = String::from("string 1");
    let string2 = String::from(" - string 2");
    let str = concentenate_strings(&string1[0..string1.len()-4], &string2[0..string2.len()-6]);
    println!("{}", str);
}

fn concentenate_strings(slice1: &str, slice2: &str) -> String {
    let string1 = slice1.to_string();
    let string2 = slice2.to_string();
    let mut result = string1;
    result.push_str(&string2);
    result
}