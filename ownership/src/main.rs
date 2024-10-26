fn main() {
    // let var = 1;

    // let mut s = "hello".to_string();
    // s.push_str(", world!");

    // let x = vec!["rehan".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // Change string function call
    // let mut s = String::from("hello");
    // change_string(&mut s);

    // Assignment
    let mut vect = vec![1, 3, 5, 7];

    println!("{}", eval_vec(&vect));

    vect.push(15);

    println!("{:?}", &vect);

    let mut val1 = 5;
    println!("{}", add_two(&mut val1));

}

// fn change_string(string1: &mut String) {
//     string1.push_str(", world!");
//     println!("{}", string1);
// }

fn eval_vec(val: &Vec<i32>) -> bool {
    if val[0] == 1 {
        return true;
    }
    return false;
}

fn add_two(val: &mut i8) -> i8 {
    *val += 2;
    *val
}