extern create my_crete
use my_crete::kinds::PrimaryColor;
use my_crete::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
