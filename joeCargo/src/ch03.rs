pub fn ch03_01() {
    let  x = 5;
    {
        let  x = 7;
        println!("inner x = {} ", x);
    }
    println!("outer x = {} ", x);  // result is 5
}

pub fn ch03_02() {
    let guess: i64 = "42".parse().expect("Not a number!");
    println!("parse number is {}", guess);


    let tup = (500, 6.54321, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {_y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("The value of x.0 is: {five_hundred}");


    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!( "a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    println!("{}", months.len() );

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!( "a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    let a = [3; 5];  // let a = [3, 3, 3, 3, 3];
    println!( "a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}

pub fn ch03_03() {
    print_labeled_measurement(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("expression, is not functional returne, y={y}");

    let y = {
        let x = 21;
        x
    } ;
    println!("expression, is not functional returne, y={y}");

    let x = plus_one(31);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn ch03_05() {
    loop_test();
    multi_if_test();
    break_test();
    while_test();
    for_test();
}

fn multi_if_test(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("continuing count = {count}");
        let mut remaining = 10;

        loop {
            print!("{remaining},");
            if remaining == 7 {
                break;   // break out in-loop
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        } // in-loop
        count += 1;
    }  // loop, labeled counting_up
    println!("End count at break counting_up = {count}");
}

fn break_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!! whit last num is {}", number);
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    for idx in (1..4).rev() {
        println!("the idx is: {}, value is: {}", idx, a[idx]);
    }
}