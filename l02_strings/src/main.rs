fn main() {
    let hero_name = "Mr Jason".to_string();
    println!("Hero name is {}", hero_name);
    let short_name = hero_name.replace("Mr ", "");
    println!("Short name is {}", short_name);

    let position = String::from("Lead");
    println!("{}", position);

    let greetings = "Greetings dear young and creative man".to_string();
    println!("{}", greetings);
    let short_greetings = greetings.get(0..10).unwrap();
    println!("{}", short_greetings);

    let hello_world = "Hello, world!";
    println!("{}", hello_world);

    let _hello = "Hello";
    let hello_world2 = "Hello".to_string();
    let hello_ref = &hello_world2;
    println!("{}", hello_world2);
    println!("{}", hello_ref);

    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}";
    println!("{}", konnichiwa);
}
