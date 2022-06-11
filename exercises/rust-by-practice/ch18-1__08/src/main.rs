/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {
        s.push_str(str);
        s
    };

    exec(update_string);
}

fn exec<'a, F>(mut f: F)
where
    F: FnOnce(&'a str) -> String,
{
    f("hello");
}
