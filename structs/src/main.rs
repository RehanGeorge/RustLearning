/*
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple struct
struct Coordinates(i32, i32, i32);

// Unit struct
struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_area(&self) {
        println!("The area of the square is: {}", self.area());
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let user1 = User{
        active: true,
        username: String::from("Rehan"),
        sign_in_count: 0,
    };

    println!("{}", user1.username);

    let user2 = build_user(String::from("George"));
    println!("{}", user2.username);

    let coords = Coordinates(1, 2, 3);

    let mut sq = Square { width: 10, height: 10 };
    sq.whats_my_area();

    sq.change_width(20);
    sq.whats_my_area();

    lifetimes();
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
 */

/*
// Lifetimes

struct MyString<'a> {
    text: &'a str,
}

fn main() {

    let str1 = String::from("This is my string");
    let x = MyString{ text: str1.as_str() };
    let s = &'static str = "This is a static string";

    let r;
    {
        let x = 5;
        r = &x
    }

    println!("{}", r);
}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
*/

// Assignment
struct Car {
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car {
    fn set_mpg(&mut self, new_mpg: u32) {
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: u32) {
        self.top_speed = new_top_speed;
    }
}

fn main() {
    let mut my_car = Car {
        mpg: 25,
        color: String::from("Red"),
        top_speed: 120,
    };

    println!("My car has an mpg of: {}", my_car.mpg);
    println!("My car is: {}", my_car.color);
    println!("My car has a top speed of: {}", my_car.top_speed);

    my_car.set_mpg(30);
    my_car.set_color(String::from("Blue"));
    my_car.set_top_speed(130);

    println!("My car now an mpg of: {}", my_car.mpg);
    println!("My car is now: {}", my_car.color);
    println!("My car now has a top speed of: {}", my_car.top_speed);
}
