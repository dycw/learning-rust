// Defining an Enum

enum IpAddrKind {
    V4,
    V6,
}

// Enum Values

fn enum_values_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn route(ip_kind: IpAddrKind) {}

fn enum_values_2() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_values_3() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {}
}

fn enum_values_4() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn main() {
    enum_values_4();
}
