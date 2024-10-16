use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// Song struct
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Song {
    title: String,
    singer: String, // Changed from artist to singer
    album: String,
    duration: u32,
}

// Playlist struct
#[derive(Debug, Serialize, Deserialize)]
struct Playlist {
    name: String,
    songs: Vec<Song>,
}

// PlaylistManager struct
#[derive(Debug, Serialize, Deserialize)]
struct PlaylistManager {
    playlists: HashMap<String, Playlist>,
}

impl PlaylistManager {
    // Constructor for a new PlaylistManager
    fn new() -> Self {
        PlaylistManager {
            playlists: HashMap::new(),
        }
    }

    // Add a new playlist
    fn add_playlist(&mut self, name: String) {
        let playlist = Playlist {
            name: name.clone(),
            songs: Vec::new(),
        };
        self.playlists.insert(name, playlist);
        println!("Playlist created!");
    }

    // Add a song to a playlist
    fn add_song(&mut self, playlist_name: &str, song: Song) {
        if let Some(playlist) = self.playlists.get_mut(playlist_name) {
            playlist.songs.push(song);
            println!("Song added to playlist!");
        } else {
            println!("Playlist not found!");
        }
    }

    // List all playlists
    fn list_playlists(&self) {
        if self.playlists.is_empty() {
            println!("No playlists available.");
        } else {
            for (name, _) in &self.playlists {
                println!("Playlist: {}", name);
            }
        }
    }

    // List songs in a playlist
    fn list_songs(&self, playlist_name: &str) {
        if let Some(playlist) = self.playlists.get(playlist_name) {
            println!("Songs in playlist {}:", playlist_name);
            for song in &playlist.songs {
                println!(
                    "Title: {}, Singer: {}, Album: {}, Duration: {}s",
                    song.title, song.singer, song.album, song.duration
                );
            }
        } else {
            println!("Playlist not found!");
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
        println!("1. New playlist");
        println!("2. Add song");
        println!("3. Show playlists");
        println!("4. Show songs");
        println!("5. Save & Exit");

        print!("Your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Playlist name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                manager.add_playlist(name.trim().to_string());
            }
            "2" => {
                // Show available playlists before adding a song
                println!("Available playlists:");
                manager.list_playlists();
                
                print!("Choose a playlist: ");
                io::stdout().flush().unwrap();
                let mut playlist_name = String::new();
                io::stdin().read_line(&mut playlist_name).unwrap();

                print!("Song title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                print!("Singer name: ");
                io::stdout().flush().unwrap();
                let mut singer = String::new();
                io::stdin().read_line(&mut singer).unwrap();

                print!("Song album: ");
                io::stdout().flush().unwrap();
                let mut album = String::new();
                io::stdin().read_line(&mut album).unwrap();

                print!("Song duration (seconds): ");
                io::stdout().flush().unwrap();
                let mut duration = String::new();
                io::stdin().read_line(&mut duration).unwrap();

                let song = Song {
                    title: title.trim().to_string(),
                    singer: singer.trim().to_string(),
                    album: album.trim().to_string(),
                    duration: duration.trim().parse().unwrap(),
                };

                manager.add_song(playlist_name.trim(), song);
            }
            "3" => manager.list_playlists(),
            "4" => {
                print!("Playlist name: ");
                io::stdout().flush().unwrap();
                let mut playlist_name = String::new();
                io::stdin().read_line(&mut playlist_name).unwrap();
                manager.list_songs(playlist_name.trim());
            }
            "5" => {
                manager.save();
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid option! Try again."),
        }
    }
}
