use std::thread::sleep;
use std::time::Duration;

fn main() {
    let name = String::from("User");
    let name2 = "Rust World".to_string();
    println!("Hello, world!");
    println!("Hello, {name}");
    println!("Hello, {name2}");

    let mut player_score = 51;
    player_score += 1;
    println!("{player_score}");

    let _delta_time = 1.25;
    let _delta_time: f32 = 1.28;
    let _delta_time = 1.35_f32;
    let delta_time = 1.38_f64;
    println!("Current Delta time: {delta_time}");

    let _total_points: u8 = 1 + 2 + 5;

    let color_in_hex = 0xFFFFFF;
    println!("Color {color_in_hex:x}");

    let dir_permission = 0o777;
    println!("Permission {dir_permission:o}");

    let gateway: u8 = 0b1010_0100;
    println!("Gateway {gateway:b}");

    let is_active = true;
    println!("Is Active: {is_active}");

    let first_char = 'A';
    println!("First Char: {first_char}");

    let config = (640, 320, String::from("Test"), true);
    println!("Config: {config:?}");
    println!("Config is: {config:#?}");

    let width = config.0;
    let height = config.1;
    let (w, h) = (width, height);
    println!("The width is {w} and the height is {h}");

    let mut scores: [u8; 6] = [25, 57, 38, 99, 67, 13];
    println!("scores: {scores:?}");
    println!("First score = {}", scores[0]);
    println!("Scores length: {}", scores.len());
    scores[1] += 25;
    println!("First score = {}", scores[1]);

    println!("Background color: {BACKGROUND_COLOR:?}");
    sleep(Duration::from_secs(5));
}

const BACKGROUND_COLOR: (u8, u8, u8) = (0xff, 0xff, 0xff);
