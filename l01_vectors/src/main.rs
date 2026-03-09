fn main() {
    let mut scores: Vec<u16> = vec![10, 20, 30, 40];
    scores.push(50);
    scores.push(100);
    println!("{:?}", scores);

    for s in scores.iter() {
        print!("{} ", s);
    }
    let last_score = scores.pop().unwrap();
    println!("\nLast score is: {last_score}");

    let mut colors = Vec::new();
    colors.push(String::from("green"));
    colors.push(String::from("red"));
    colors.push(String::from("purple"));
    println!("{:?}", colors);
    colors.reverse();
    println!("{:?}", colors);

    let codes :Vec<u8> = (50..=65).collect();
    println!("{:?}", codes);

    let numbers = (1..=9).collect::<Vec<i32>>();
    let first_two = numbers[0..2].to_vec();
    println!("{:?}", first_two);
}
