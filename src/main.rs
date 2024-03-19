use clap::Parser;
use rouille::*;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;
use std::time::Duration;
use std::{io, fs};
use std::path::PathBuf;
use glob::glob;

use FCFFh::json::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

}

#[derive(Debug, Clone)]
struct GameIdentifier {
    path: String,
    system: String,
    hashes: Vec<String>,
    names: Vec<String>,
    address: String,
}

impl GameIdentifier {
    pub fn from_game(game: Game, path: PathBuf) -> Self {
        Self {
            path: path.to_str().unwrap().to_string(),
            system: game.system,
            hashes: game.hashes,
            names: game.names,
            address: game.addr,
        }
    }
}

#[derive(Debug, Clone)]
struct Play {
    artist: String,
    title: String,
    album: String,
}

impl ToString for Play {
    fn to_string(&self) -> std::string::String {
        return format!("{} - {} (on {})", self.artist, self.title, self.album);
    }
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    // Load identifiers for all games so we can search a database when a new game starts
    let mut game_database: Vec<GameIdentifier> = vec![];

    for config_file in glob("./games/**/*.json").unwrap().map(|x| x.unwrap()) {
        let game = serde_json::from_str(&fs::read_to_string(config_file.clone()).unwrap()).unwrap();
        game_database.push(GameIdentifier::from_game(game, config_file));
    }

    let (s, r) = unbounded::<Play>();

    std::thread::spawn(|| {
        http_server(game_database, s);
    });


    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    
    let mut current_song: String = "(NONE)".to_string();

    eframe::run_simple_native("FCFFh", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let received = r.recv_timeout(Duration::from_millis(1000)).ok();
            match received {
                Some(song) => {
                    current_song = song.to_string()
                },
                None => (),
            }
            ui.heading("FCFFh");
            ui.vertical(|ui| {
                ui.label(current_song);
            });
        });
    })
}

fn http_server(game_database: Vec<GameIdentifier>, s: Sender<Play>) {
    rouille::start_server("localhost:8000", move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                // In case someone tries to just query the server
                (GET) (/) => {
                    // When viewing the home page, we return an HTML document described below.
                    rouille::Response::text("FCFFh server")
                },
                // Instruct where to read when game changes
                (GET) (/game_biz) => {
                    let system = request.get_param("system").expect("Expected system");
                    let rom_hash = request.get_param("rom_hash").expect("Expected ROM hash");
                    let rom_name = request.get_param("rom_name").expect("Expected ROM name");
                    for rom in &game_database {
                        let rom = rom.clone();
                        if rom.system == system {
                            if rom.hashes.contains(&rom_hash) || rom.names.contains(&rom_name) {
                                return rouille::Response::text(rom.address);
                            }
                        }
                    }
                    rouille::Response::text("None")
                },
                (GET) (/submit_biz) => {
                    // BizHawk system: `emu.getsystemid()`
                    let system = request.get_param("system").expect("Expected system");
                    // The read value (varies by game, generally requires an address to be read)
                    let id = request.get_param("id").expect("Expected song ID");
                    // The address read from, if applicable
                    //let address = request.get_param("address").expect("Expected address");
                    // ROM hash: `gameinfo.getromhash()`
                    let rom_hash = request.get_param("rom_hash").expect("Expected ROM hash");
                    // ROM name: `gameinfo.getromname()`
                    let rom_name = request.get_param("rom_name").expect("Expected ROM name");
                    
                    todo!();

                    rouille::Response::text("OK")
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}