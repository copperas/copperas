// This will not be a part of an Engine, but part of any game
use std::fs::read_to_string;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    controlls: Controlls,
    time:      Time,
    window:    Window
}

impl Config {
    pub fn new(path: &str) -> Config {
        let path = Path::new(path);
        match read_to_string(path) {
            Ok(toml_str) => match toml::from_str(&toml_str) {
                Ok(config) => config,
                Err(e) => panic!("got: {:?}", e)
            },
            Err(_) => panic!("config.toml was not found!")
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn controlls(&self) -> &Controlls {
        &self.controlls
    }

    pub fn time(&self) -> &Time {
        &self.time
    }
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Actions {
    move_forward:  String,
    move_left:     String,
    move_backward: String,
    move_right:    String
}

#[derive(Deserialize)]
pub struct Controlls {
    actions: Actions
}

#[derive(Deserialize)]
pub struct Time {
    delta: u64
}

impl Time {
    pub fn delta(&self) -> u64 {
        self.delta
    }
}

#[derive(Deserialize)]
pub struct Window {
    width:  f64,
    height: f64,
    title:  String
}

impl Window {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
