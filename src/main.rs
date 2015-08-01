extern crate music_streamer;

mod auth;

fn main() {
    println!("Please copy this url to your browser:\n{}",
             auth::get_app_auth_link());
    println!("Paste here url after you authorize this application");
    println!("Thank you");
}
