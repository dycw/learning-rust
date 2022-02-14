fn exporting_a_convenient_public_api_with_pub_use_1() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

use publishing_a_crate_to_crates_io::mix;
use publishing_a_crate_to_crates_io::PrimaryColor;

fn exporting_a_convenient_public_api_with_pub_use_2() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

fn main() {
    exporting_a_convenient_public_api_with_pub_use_2();
}
