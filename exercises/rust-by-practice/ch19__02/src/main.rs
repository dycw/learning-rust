/* Make it workd */
struct Meters(u32);

impl Meters {
    fn pow(self, n: u32) -> u32 {
        self.0.pow(n)
    }
}

fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    // The `pow` method is defined on `u32` type, we can't directly call it
    assert_eq!(n.pow(2), 4);
}
