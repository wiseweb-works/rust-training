use rand::RngExt;

fn main() {
    let mut rnd = rand::rng();
    let some_number = rnd.random_range(1..1_000);
    println!("{}", some_number);

    if some_number % 2 == 0 {
        println!("{} is even", some_number);
    } else if some_number % 2 == 1 {
        println!("{} is odd", some_number);
    } else {
        println!("{} another situation", some_number);
    }

    loop {
        let number = rnd.random_range(1..101);
        println!("{}", number);
        if number % 19 == 0 {
            println!("I have gotcha {}", number);
            break;
        } else {
            continue;
        }
    }

    let mut try_count = 0;
    while try_count < 100 {
        let number = rnd.random_range(1..101);
        if number % 23 == 0 {
            println!("I found the number {} in try {}", number, try_count);
            break;
        } else {
            try_count += 1;
        }
    }

    let data = get_random_number(10);
    for (index, value) in data.iter().enumerate() {
        println!("index = {}\t value = {}", index, value);
    }

    check_exam_score(rnd.random_range(0..=100));
}

fn get_random_number(upper_limit: u8) -> Vec<i32> {
    let mut rnd = rand::rng();
    let mut numbers = Vec::new();

    for _i in 0..upper_limit {
        let n = rnd.random_range(1..101);
        if numbers.contains(&n) {
            continue;
        }
        numbers.push(n);
    }
    numbers
}

fn check_exam_score(point: u8) {
    if point == 0 {
        println!("Blank paper! Fails.");
    } else if point >= 70 {
        println!("{point} is enough for pass");
    } else if point < 50 {
        println!("{point} is not enough for pass");
    } else if point >= 50 && point < 70 {
        println!("{point} is greater than 50 but less than 70. Come in September!");
    }
}
