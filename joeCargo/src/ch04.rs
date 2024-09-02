use std::ffi::c_char;
use std::sync::atomic::fence;

pub fn ch04_01() {
    ownership_movetest();
    ownership_returntest();
}

fn ownership_returntest() {
    println!("ownership return move at function call !");
    let s1= String::from("hello-test message");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    //println!("The length of '{}' and '{}' is {}.", s1, s2, len);
    // s1는 유효하지 않음으로 컴파일 에러 발생
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다
    (s, length)
}

fn ownership_movetest() {
    println!("ownership at function call !");

    let s = String::from("hello-test message");  // s가 스코프 안으로 들어옵니다
    println!("s={}", s);     // s가 살아 있으므로 유효

    takes_ownership(s);      // s의 값이 함수로 move됨. 따라서 더 이상 유효하지 않음
    // println!("s={}", s);     // s는 유효하지 않음으로 컴파일 에러 발생

    let x = 5;        // x가 스코프 안으로 들어옵니다
    makes_copy(x);    // x가 함수로 이동될 것입니다만, i32는 Copy이므로 계속 x를 사용 가능
    println!("x={}",x);

} // 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 됩니다. 그러나 s의 값이 이동되었으므로
// 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("at takes_ownership {}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
// some_string 관련 메모리가 해제됩니다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다
    println!("at makes_copy {}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.







pub fn ch04_02() {
    fn_ref_test();
    fn_ref_test2();
    dangle_test();
}

fn fn_ref_test() {
    let s1 = String::from("This is Reference test string...");
    let len = calculate_length_print(&s1);
    println!("T1) The length of s1::'{}' is {}. can use s1.", s1, len);
}

fn calculate_length_print(s: &String) -> usize {
    println!("in fn cal_len_print: s:&String=[{}], len=[{}]", s, s.len());
    s.len()
}

fn fn_ref_test2() {
let mut s1 = String::from("hello");
let len = calculate_length_print2(&mut s1);
println!("T2) The length of s1::'{}' is {}. s1 added 'world' by other function with mut, refer", s1, len);
}

fn calculate_length_print2(s: &mut String) -> usize {
    println!("in cal_len_print: s:&String=[{}], len=[{}].", s, s.len());
    s.push_str(", world");
    println!("in cal_len_print: s:&String=[{}], len=[{}].", s, s.len());
    s.len()
}

fn dangle_test() {
    println!("T3) Dangling test:::");
    let reference_to_nothing:String;
    reference_to_nothing = dangle();
    println!("test return = {}", reference_to_nothing );

    // let mut ref_to_string= String::from("HELLOWORLD");
    let mut ref_to_string= String::new();

    println!("ref_string test before = {}", ref_to_string );
    dangle2(&mut ref_to_string);
    println!("ref_string test after  = {}", ref_to_string );
}

fn dangle() -> String {
    //let s = String::from("hello");
    let mut s = String::new();
    s.push_str("pushed HELLO at func, for mut-ref test");
    // println!("in function={}", s);
    s
}

fn dangle2(sss: &mut String ) {
    //let s = String::from("hello");
    //let mut sss = String::new();
    sss.push_str("pushed HELLO to &mut String at function");
    // println!("in function={}", s);
}



pub fn ch04_03() {
    // let sss = String::from("HelloWorld is Good?");
    let sss = String::from("HelloWorld");
    println!("Ch04_03:: {} and {}.", sss, &sss);
    let sss2 = first_word(&sss);
    println!("return sss2 = {}, of sss = {}", sss2, sss);
    println!("return sss2 = {}, of sss = {}", first_word(&sss), sss);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("check[{}]={}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}