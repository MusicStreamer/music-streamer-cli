extern crate music_streamer;
extern crate gtk;

mod auth;

use gtk::traits::*;

fn main() {
    println!("Please copy this url to your browser:\n{}",
             auth::get_app_auth_link());
    println!("Paste here url after you authorize this application");
    println!("Thank you");
}
