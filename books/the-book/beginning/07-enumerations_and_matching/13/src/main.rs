fn main() {
    #[allow(dead_code)]
    enum CardinalPoint {
        North,
        South,
        West,
        East,
    }
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => print!("NORTH"),
        _ => {}
        CardinalPoint::South => print!("SOUTH"),
    }
}
