fn main() {
    // OWNERSHIP CONCEPT

    let s1 = String::from("test");
    let mut s2 = &s1; //giving ref of s1 to s2, both can be used

    println!("s1: {}, s2: {}", s1, s2);

    let binding = String::from("changed");
    s2 = &binding;

    println!("s1: {}, s2: {}", s1, s2);

    let mut _s3 = s1; //transfering ownership of s1 to s3, cannot use s1 anymore

    // println!("s1: {}, s3: {}", s1, s3); //borrow of moved value error: OWNERSHIP TRANSFER ERROR

    let mut s4 = s2.clone();

    println!("s2: {}, s4: {}", s2, s4);

    s4 = String::from("changed again");

    println!("s2: {}, s4: {}", s2, s4);

    // only one can have the ownership at a time
    // C/C++ aksine Rust'ta aynı adrese işaret eden birden fazla ptr bulunamaz

    // steak --> fixed , used for small size var.s - int.s, chars etc - compile-time - more systemical
    // heap --> dinamic , used for larger size var.s - str.s etc - less systemical - run-time
    // ownership determines whether a val. is stored on heap or stack, it is determined by the size of the val

    // BORROWING & REFERENCES

    // sometimes ownership does not required
    // borrowing is a way to temporiraly access an address
    // unlike owned val.s ref.s r lightweight n efficient

    let mut str = String::from("test");
    let _my_ref = &str; //this is immut.

    print_str(&str);
    change_str(&mut str);
    print_str(&str);

    // multiple immut or mut var.s can be had
    let first_immut_ref = &str;
    let sec_immut_ref = &str;

    println!("first immut ref: {}, sec immut ref: {}", first_immut_ref, sec_immut_ref);

    // cannot have both mut and immut var.s to a var. at the same time
    let first_mut_ref = &mut str; //this line will not give any error

    println!("first mut ref: {}", first_mut_ref);

    //but if we try the print an immut var, prog will throw an error to first_mut_ref var.
    //println!("first immut ref again: {}", first_immut_ref);

    let sec_mut_ref = &mut str;

    // this line will be compiled without any error
    println!("sec mut ref: {}", sec_mut_ref);

    //but if we try to use the immut ones, then the problem above will occur, since it creates a conflict
    //also we cannot use the first created mut ref after the secondly created one, it will give same err

    // DANGLING REF.S
    // a ptr which is pointing a loc that has been deallocated, 'causes unexpecting behav.s
    // can happen when a ref is still in scope after the obj it points dropped

    let new_str = String::from("test");
    let new_str_ref = return_ref(&new_str);

    println!("new str: {}", new_str); //runs without an error

    let newer_str = new_str;

    //println!("new str ref: {}", new_str_ref); //gives error, the loc it pointed has been transferred to newer_str
    
    /*
        rules to use ref.s

        #1: either have one mut ref or any num of immut refs
        #2: cannot have both mut & immut ref.s to a var. at the same time
        #3: ref.s always must be valid & point to a valid mem loc
        #4: ref.s automatically expires at the end of their scope 
    */
}

fn print_str(str: &String) {
    println!("{}", str)
}

fn change_str(str: &mut String) {
    str.push_str(" changed")
}

fn return_ref(str: &String) -> &String {
    str
}