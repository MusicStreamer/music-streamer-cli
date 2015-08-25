// This file is part of music-streamer-cli.
//
// music-streamer-cli is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// music-streamer-cli is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with music-streamer-cli.  If not, see <http://www.gnu.org/licenses/>.

extern crate music_streamer;

mod auth;

use std::io;
use auth::Authentication;

fn main() {
    let mut auth_deezer = Authentication::new(auth::Service::DEEZER);
    let url = auth_deezer.get_app_auth_link();
    println!("Please copy this url to your browser:\n{}", url);

    println!("Paste here url after you authorize this application");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Can't read from command line");

    auth_deezer.authenticate_app(&input)
        .ok()
        .expect("Can't authenticate application");

    println!("Thank you");
}
