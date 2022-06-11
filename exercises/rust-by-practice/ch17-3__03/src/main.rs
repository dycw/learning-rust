/* Adding trait bounds to make it work */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
    'a: 'b,
{
    y = x;
    let r: &'b &'a i32 = &&0;
}

fn main() {
    println!("Success!")
}
