fn main() {
    let mario = spawn_random_player("Super Mario".to_string());
    println!("{:?}", mario);
    println!(
        "Player name: {}, Level: {}, Active: {}, Position: {:?}",
        mario.name, mario.level, mario.is_active, mario.position
    );

    let position = Position(1.0, 2.0);
    println!("{:?}", position);

    let _x_value = position.0;
    let _y_value = position.1;

    let mut rectangle = Rectangle::new(100, 10);
    println!("Area is {}", rectangle.area());
    println!("Perimeter is {}", rectangle.perimeter());
    rectangle.resize(50, 75);
    println!("Area is {}", rectangle.area());
    println!("Perimeter is {}", rectangle.perimeter());
}

fn spawn_random_player(name: String) -> Player {
    Player {
        name,
        is_active: false,
        level: 100,
        position: Position(2.4, 5.6),
    }
}

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
    position: Position,
    is_active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

struct _Entity;
