struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn ch05_01() {
    println!("ch05_01...");
    let mut user1 = User {
        active: true,
        username: String::from("userhelloname"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("1) {}", user1.email);
    user1.email = String::from("setanotheremail@example.com");
    println!("1) {}", user1.email);

    let user2 = User {
        email: String::from("user2mail@example.com"),
        username: user1.username,  // 이 줄은 이런 방식을 축약한 아래 표현으로 필요 없다.
        ..user1
    };

    let user3 = User {
        email: String::from("user3mail@example.com"),
        username: String::from("newusername"),
        ..user1
    };

    println!("2) user2:: {}, {}", user2.username, user2.email);
    println!("2) user3:: {}, {}", user3.username, user3.email);
    // println!("2) user1:: {}, {}", user1.username, user1.email);


    let  user4 = User {
        active: true,
        username: String::from("userLTname"),
        email: String::from("userLT@example.com"),
        sign_in_count: 1,
    };
    println!("3) user4:: {}, {}", user4.username, user4.email);

} // ch05_01

// use case: LifeTime parameter
// refer at chapter 10

struct UserLT<'a> {
    active: bool,
    username: &'a str ,
    email: &'a str,
    sign_in_count: u64,
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn ch05_02() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("ch05_02...");
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        //width: dbg!(30 * scale),
        width: 30 * scale,
        height: 50,
    };

    dbg!(&rect1);
}

// ch05-03


impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

pub fn ch05_03() {
    let rect1 = Rectangle {
        width: 30,
        height: 5,
    };

    println!(
        "Ch05-03) The area of the rectangle is {} square pixels. with {}*{}.",
        rect1.area(), rect1.width, rect1.height
    );
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn ch05_03_01() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Ch05-03-01, Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Ch05-03-01, Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}