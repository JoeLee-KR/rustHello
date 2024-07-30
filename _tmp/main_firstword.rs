//
// Hello~
//
//use std::io;
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    println!("the first word is: {}", s);

    let word = first_word(&s);
    //s.clear(); // 에러!
    println!("the second word is: {}", word);
    s.clear(); // 정상!

    let a = [11, 12, 13, 14, 15];
    let slice = &a[1..4];
    println!("slice test, a is {}, slice is {}", a.len(), slice.len());
    assert_eq!(slice, &[12, 13, 14]);
}