fn main() {
    let s = String::from("hello s");
    takes_ownership(s);
    // println!("Still here? {}", s);

    let s1 = gives_ownership();

    println!("Still Here? {}", s1);

    let s2 = String::from("hello s2");


    let s3 = takes_and_gives_back(s2);

    // println!("Still Here? {}", s2);
    println!("Still Here? {}", s3);

    let x = 5;

    makes_copy(x);

    println!("Still here? {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string );
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello giver");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
