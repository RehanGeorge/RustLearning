fn main() {
    /* 
    let x: i8 = 10;
    println!("The value of x is {}", x);

    let y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;

    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("The value of y is {}", y);
    println!("The value of decimal is {}", decimal);
    println!("The value of hex is {}", hex);
    println!("The value of octal is {}", octal);
    println!("The value of binary is {}", binary);

    let byte = b'A';
    println!("The value of byte is {}", byte);

    let x = 2.0; //f64 default because on modern CPUs it's the fastest and same as f32
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';

    println!("The value of x is {}", x);
    */

    let val1 = 5;
    let val2 = 2;

    let ans = val1 % val2;

    println!("The value of val1 modulo val2 is {}", ans);

    let mut vec = vec![2,4,6,8,10];

    for i in &vec {
        println!("The value of i is {}", i);
    }

    vec.pop();
    vec.push(12);

    for i in &vec {
        println!("The value of i is {}", i);
    }

    fn concat_string(s1: &str) -> String {
        let s2 = " World!";
        s1.to_string() + s2
    }
    let s = "Hello".to_string();

    println!("{}", concat_string(&s));

    let val3 = 5;
    
    fn control_flow(int: i32) {
        if int == 1 {
            println!("The value of int is 1");
        } else if int < 25 {
            println!("The value of int is less than 25");
        } else if int < 50{
            println!("The value of int is greater than 25 but less than 50");
        } else {
            println!("The value of int is greater than 50");
        }
    }

    control_flow(val3);
}
