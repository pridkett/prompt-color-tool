use md5::{Md5, Digest};
use clap::{Command, Arg};
use std::process::Command as ProcessCommand;

mod theme_maps;

#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn to_hex(self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

fn main() {
    let matches = Command::new("prompt_color_tool")
        .version("0.1")
        .author("Patrick Wagstrom <160672+pridkett@users.noreply.github.com>")
        .about("Generates MD5 hash for a given input string")
        .arg(
            Arg::new("hostname")
                .help("The hostname to hash")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("theme")
                .help("The theme to use for color calculation")
                .long("theme")
                .num_args(1)
                .default_value("default"),
        )
        .arg(
            Arg::new("hex")
                .help("Output colors in hexadecimal format")
                .long("hex")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let hex_output = matches.get_flag("hex");

    let input_hostname = matches.get_one::<String>("hostname").map(|s| s.as_str());
    let input_theme = matches.get_one::<String>("theme").map(|s| s.as_str());

    let bgcolor = calculate_hostname_background_color(input_hostname);
    match calculate_hostname_foreground_color(bgcolor, input_theme) {
        Ok(fgcolor) => {
            if hex_output {
                let bg_rgb = xterm_to_rgb(bgcolor);
                let fg_rgb = xterm_to_rgb(fgcolor);
                println!("bgcolor: #{}, fgcolor: #{}", bg_rgb.to_hex(), fg_rgb.to_hex());
            } else {
                println!("bgcolor: {}, fgcolor: {}", bgcolor, fgcolor);
            }
        },
        Err(e) => eprintln!("Error calculating foreground color: {}", e),
    }
}

fn calculate_hostname_foreground_color(fgcolor: u8, theme: Option<&str>) -> Result<u8, String> {
    let theme = theme.unwrap_or("default");

    if !theme_maps::THEME_MAP.contains_key(theme) {
        eprintln!("Warning: Theme '{}' not found. Using default theme.", theme);
    }

    if let Some(map) = theme_maps::THEME_MAP.get(theme).or_else(|| theme_maps::THEME_MAP.get("default")) {
        if let Some(color) = map.get(&fgcolor) {
            return Ok(*color);
        }
        return Err(format!("Color not found for color: {} theme: {} ", fgcolor, theme));
    }
    return Err(format!("Key not found for theme: {}", theme));
}

fn calculate_hostname_background_color(hostname: Option<&str>) -> u8 {
    let hn = match hostname {
        Some(h) => h.to_string(),
        None => {
            let output = ProcessCommand::new("hostname")
                .output()
                .expect("Failed to execute hostname command");

            String::from_utf8_lossy(&output.stdout)
                .split('.')
                .next()
                .unwrap_or("localhost")
                .trim()
                .to_string()
        }
    };

    let mut hasher = Md5::new();
    hasher.update(hn.as_bytes());
    let hash = hasher.finalize();

    let hex_str = format!("{:x}", hash);
    let first_two_chars = &hex_str[..2];
    let color_value = u8::from_str_radix(first_two_chars, 16).unwrap() % 128;

    return color_value;
}

fn xterm_to_rgb(xterm_color: u8) -> Rgb {
    if xterm_color < 16 {
        let colors = vec![
            "000000", "cd0000", "00cd00", "cdcd00", "0000ee", "cd00cd", "00cdcd", "e5e5e5",
            "7f7f7f", "ff0000", "00ff00", "ffff00", "5c5cff", "ff00ff", "00ffff", "ffffff",
        ];

        let color = colors[xterm_color as usize];
        let r = u8::from_str_radix(&color[0..2], 16).unwrap();
        let g = u8::from_str_radix(&color[2..4], 16).unwrap();
        let b = u8::from_str_radix(&color[4..6], 16).unwrap();

        return Rgb { r, g, b };
    }

    if xterm_color < 232 {
        let idx = xterm_color - 16;
        let red = idx / 36;
        let green = (idx % 36) / 6;
        let blue = idx % 6;

        let values = vec![0, 95, 135, 175, 215, 255];
        let r_color = values[red as usize];
        let g_color = values[green as usize];
        let b_color = values[blue as usize];

        return Rgb { r: r_color, g: g_color, b: b_color };
    }

    let gray = 8 + (xterm_color - 232) * 10;
    return Rgb { r: gray, g: gray, b: gray };
}