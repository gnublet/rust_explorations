fn main() {
    println!("Hello, world!");

    // Note, utf8 may be a bit more complex. 
    let s = String::from("hello kevin");

    // string literals are slices. They are immutable since &str is an immutable reference
    // let mut s2 = "hello world";


    // let hello = &s[..5];
    // let world = &s[6..];
    // println!("{}", first_word(&s));

    // let word = first_word(&s);
    // let word: &str = first_word(&s);
    let word: &str = first_word(&s[..]);

    // mutable borrow not allowed with immutable borrow on the same variable
    // s.clear();

    println!("The first word is {}", word);

    let my_string_literal = "hello world";
    let word2 = first_word(&my_string_literal);
    println!("The first word is {}", word2);
}
// fn first_word(s: &String) -> usize {
// return string slice
// not a great function since function signature doesn't allow use on both &String and &str
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             // return i;
//             return &s[0..i];
//         }
//     }

//     // s.len()
//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}