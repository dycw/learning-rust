#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
