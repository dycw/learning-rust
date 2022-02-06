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

fn main() {
    enum_values_3();
}
