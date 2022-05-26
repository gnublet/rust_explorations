fn main() {

    // HEAP DATA
    // string literal (hard coded string, text is hardcoded directly into final executable)
    // let s = "hello"; 

    // String type (manages data allocated on heap and is able to store text unknown to us at compile time)
    // memory allocated at runtime
    let mut s1 = String::from("hello"); 
    s1.push_str(", world!"); // appends a literal to a String
    println!("{}", s1);

    // length = how much memory (in bytes), the contents of String is using

    // capacity = total memory (in bytes), the String has received from the allocator

    // move = shallow copy + invalidating previous variable. 
    // "s1 was moved into s2"
    let s2 = s1;
    // borrowing - to ensure memory saftey, s1 is no longer valid
    // println!("{}", s1);

    println!("{}", s2);

    // deep copy: heap data is copied
    let s3 = s2.clone();
    println!("{}", s3);

    // STACK DATA
    // no difference between deep and shallow copy. No need to move. copies are quick to make

    // REFERENCES AND BORROWING
    // unlike a pointer, a reference is guaranteed to point to a valid value of a particular type
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);

    // what happens if we try to modify something after borrowing?
    // It will give an error, but if we make the string and the argument of change mutable, we can fix this using a mutable reference
    // Note: we can only have 1 mutable reference for a given var at a time to prevent data races
    let mut s5 = String::from("hello");
    change(&mut s5);

    // multiple immutable refs are allowed since no one who is just reading the data can affect anyone else
    // however, we cannot have a mutable ref and a immutable ref since the immutable ref user doesn't expect the value to change.
    // let mut s6 = String::from("hello");
    // let r1 = &s6;
    // let r2 = &s6;
    // let r3 = &mut s6;
    // println!("{}, {}, {}", r1, r2, r3);

    // a ref's scope starts from where it is introduced and continues through the last time the ref is used
    // Non-lexical Lifetimes (NLL) is the ability for the compiler to tell that a reference is no longer being used at a point before end of scope
    let mut s7 = String::from("hello");
    let r1 = &s7;
    let r2 = &s7;
    println!("{} {}", r1, r2);

    let r3 = &mut s7;
    println!("{}", r3);

    // DANGLING REF
    // let reference_to_nothing = dangle();
    let reference_to_something  = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s;
// }

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
}