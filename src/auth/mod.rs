extern crate music_streamer;

use music_streamer::auth::deezer::AuthDeezer;
use music_streamer::auth::AuthMethods;

mod constants;

pub fn get_app_auth_link() -> String {
    let auth_deezer = AuthDeezer::new();
    
    auth_deezer.get_authorize_link(constants::APP_ID, constants::REDIRECT_URL)
}
