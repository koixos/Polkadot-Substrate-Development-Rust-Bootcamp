fn main() {
    // creating two String var.s
    let string1 = String::from("string 1");
    let string2 = String::from(" - string 2");

    // calling "concentenate_strings" func & giving random slices of the created Strings
    let concentenated_string = concentenate_strings(&string1[0..string1.len()-4], &string2[0..string2.len()-6]);

    // printing final String
    println!("{}", concentenated_string);
}

// func to concentenate two given strings
fn concentenate_strings(slice1: &str, slice2: &str) -> String {
    // to be able to use inp str's, we should turn them into String var.s
    let string1 = slice1.to_string();
    let string2 = slice2.to_string();

    // result var. must be initialized as muttable since it'll be expended
    let mut result = string1;
    result.push_str(&string2);
    
    result // returning
}
