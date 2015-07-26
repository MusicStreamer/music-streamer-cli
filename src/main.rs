extern crate music_streamer;
extern crate gtk;

mod auth;

use music_streamer::deezer;
use gtk::traits::*;

fn main() {
    auth::show_browser("www.example.com");
}
