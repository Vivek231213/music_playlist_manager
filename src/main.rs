use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

// Song struct without album and with f32 duration
#[derive(Serialize, Deserialize, Debug)]
struct Song {
    title: String,
    singer: String,
    duration: f32,  
}

// Playlist struct to hold a list of songs
#[derive(Serialize, Deserialize, Debug)]
struct Playlist {
    name: String,
    songs: Vec<Song>,
}




// PlaylistManager struct to manage multiple playlists
#[derive(Serialize, Deserialize, Debug)]
struct PlaylistManager {
    playlists: HashMap<String, Playlist>,
}




impl PlaylistManager {
    // Initialize a new PlaylistManager
    fn new() -> Self {
        PlaylistManager {
            playlists: HashMap::new(),
        }
    }

    // Add a new playlist
    fn add_playlist(&mut self) {
        let mut playlist_name = String::new();
        println!("Enter playlist name:");

        io::stdin().read_line(&mut playlist_name).expect("Failed to read line");
        let playlist_name = playlist_name.trim().to_string();

        if self.playlists.contains_key(&playlist_name) {
            println!("Playlist '{}' already exists!", playlist_name);
        } else {
            let playlist = Playlist {
                name: playlist_name.clone(),
                songs: Vec::new(),
            };
            self.playlists.insert(playlist_name.clone(), playlist);
            println!("Playlist '{}' added!", playlist_name);
        }
    }

    // Add a new song to an existing playlist
    fn add_song(&mut self) {

        // Show all playlists
        self.show_playlists();

        let mut playlist_name = String::new();
        println!("Enter playlist name to add song to:");
        io::stdin().read_line(&mut playlist_name).expect("Failed to read line");
        let playlist_name = playlist_name.trim().to_string();

        if let Some(playlist) = self.playlists.get_mut(&playlist_name) {
            let mut song_title = String::new();
            let mut song_singer = String::new();
            let mut song_duration = String::new();

            println!("Enter song title:");
            io::stdin().read_line(&mut song_title).expect("Failed to read line");

            println!("Enter singer name:");
            io::stdin().read_line(&mut song_singer).expect("Failed to read line");

            println!("Enter song duration (in minutes and seconds):");
            io::stdin().read_line(&mut song_duration).expect("Failed to read line");

            let song_duration: f32 = song_duration.trim().parse().expect("Invalid duration input");

            let song = Song {
                title: song_title.trim().to_string(),
                singer: song_singer.trim().to_string(),
                duration: song_duration,
            };

            playlist.songs.push(song);
            println!("Song added to playlist '{}'", playlist_name);
        } else {
            println!("Playlist '{}' not found!", playlist_name);
        }
    }




    // Display all playlists
    fn show_playlists(&self) {
        println!("Available playlists:");
        for p_name in self.playlists.keys() {
            println!("- {}", p_name);
        }
    }

    // Display all songs in a playlist
    fn show_songs_in_playlist(&self) {
        let mut playlist_name = String::new();
        println!("Enter playlist name to view songs:");
        io::stdin().read_line(&mut playlist_name).expect("Failed to read line");
        let playlist_name = playlist_name.trim().to_string();

        if let Some(playlist) = self.playlists.get(&playlist_name) {
            println!("Songs in playlist '{}':", playlist_name);
            for song in &playlist.songs {
                println!("Title: {}, Singer: {}, Duration: {} minuts", song.title, song.singer, song.duration);
            }
        } else {
            println!("Playlist '{}' not found!", playlist_name);
        }
    }

    // Save playlists to a file
    fn save(&self) {
        let data = serde_json::to_string_pretty(&self).expect("Error serializing data");
        fs::write("playlists.json", data).expect("Error writing to file");
        println!("Playlists saved!");
    }

    // Load playlists from a file
    fn load() -> Self {
        let data = fs::read_to_string("playlists.json");
        match data {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| PlaylistManager::new()),
            Err(_) => PlaylistManager::new(),
        }
    }
}

fn main() {
    let mut manager = PlaylistManager::load();

    loop {
        println!("\n--- Music Playlist Manager ---");
        println!("1. Add Playlist");
        println!("2. Add Song to Playlist");
        println!("3. Show All Playlists");
        println!("4. Show Songs in Playlist");
        println!("5. Save and Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => manager.add_playlist(),
            "2" => manager.add_song(),
            "3" => manager.show_playlists(),
            "4" => manager.show_songs_in_playlist(),
            "5" => {
                manager.save();
                break;
            }
            _ => println!("Invalid choice! Please enter a valid option."),
        }
    }
}
