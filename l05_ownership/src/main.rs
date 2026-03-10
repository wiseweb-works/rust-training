fn main() {
    {
        let player_score = 42;
        println!("Player score is {}", player_score);
        let _greetings = "Hello, World!".to_string();
        println!("{}", _greetings);
    }
    move_rule();
    // println!("{}", player_score);
    // println!("{}", greetings);

    let first_name = String::from("John");
    let full_name = first_name + " Doe";
    println!("Full Name: {}", full_name);
    // println!("{}", first_name);

    let book_title = String::from("The Book of Rust Programming");
    search_title(&book_title);
    println!("{}", book_title);

    let department = String::from("Finance");
    let _department = check_department(department);

    only_one_mutable();
}
fn only_one_mutable() {
    // let mut greetings = String::from("Hello, How are you?");
    let greetings = String::from("Hello, How are you?");

    let ref1 = &greetings;
    let ref2 = &greetings;

    // let mut_ref = &mut greetings;
    println!("ref1 = {}, ref2 = {}", ref1, ref2);
    // println!("mut_ref = {}", mut_ref);
}
fn check_department(department: String) -> String {
    println!("Department: {}", department);
    department
}
fn search_title(book_title: &String) {
    println!("{} is searching...", book_title);
}

fn move_rule() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let colors_again = colors;
    // println!("Colors: {:?}", colors);

    let colors_clone = colors_again.clone();
    println!("Colors: {:?}", colors_again);
    println!("Colors clone: {:?}", colors_clone);

    let number: u8 = 10;
    let mut other_number = number;
    other_number += 1;
    println!("Number: {}", number);
    println!("Other number: {}", other_number);
}
