extern crate music_streamer;

use music_streamer::deezer;

mod constants;

pub fn get_app_auth_link() -> String {
    deezer::auth::get_authorize_link(constants::APP_ID, constants::REDIRECT_URL)
}
