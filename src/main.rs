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

fn main() {
    println!("Please copy this url to your browser:\n{}",
             auth::get_app_auth_link());
    println!("Paste here url after you authorize this application");
    println!("Thank you");
}
