

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}

fn main() {
    let vector = Vector2 { x: 3.0, y: 5.0 };
    let vector2 = vector;

    println!("{:?}", vector);
}




