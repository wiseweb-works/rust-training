fn main() {
    greetings("Jack de Jack");
    let value_1 = 10.6;
    let value_2 = 5.7;
    let total = sum(value_1, value_2);
    println!("Sum of value: {}", total);

    let numbers: Vec<i32> = (0..20).collect();
    let odd_numbers = get_odds(&numbers);
    let even_numbers = get_evens(&numbers);
    println!(
        "Even numbers: {:?}, Odd Numbers: {:?}",
        even_numbers, odd_numbers
    );

    let (x1, y1) = (20_f32, 30_f32);
    println!("x1: {}, y1: {}", x1, y1);
    let (x2, y2) = move_position(x1, y1, 10_f32);
    println!("x2: {}, y2: {}", x2, y2);
}

fn greetings(name: &str) {
    println!("Hello, {}!", name);
}

fn sum(x: f32, y: f32) -> f32 {
    x + y
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = Vec::new();
    for n in numbers {
        if n % 2 == 0 {
            odds.push(*n);
        }
    }
    odds
}

fn get_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&n| n % 2 != 0).cloned().collect()
}

fn move_position(mut x: f32, mut y: f32, acceleration: f32) -> (f32, f32) {
    x += acceleration;
    y += acceleration;
    (x, y)
}
