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
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}