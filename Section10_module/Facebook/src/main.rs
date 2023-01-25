extern crate Facebook;
use Facebook::*;
fn main() {
    let user = String::from("Diwakar");
    let pass = String::from("Diwakar");
    let s = Login::login(user, pass);
    if s == 1 {
        Post::post(String::from("Hello How are you"));
        Logout::logout();
    } else {
        println!("Invalid UserName or Password");
    }
}
