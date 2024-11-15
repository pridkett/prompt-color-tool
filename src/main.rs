use md5::{Md5, Digest};
use clap::{Command, Arg};
use std::process::Command as ProcessCommand;

mod theme_maps;

const BGCOLOR_ENV_VAR: &str = "PLGO_HOSTNAMEBG";
const FGCOLOR_ENV_VAR: &str = "PLGO_HOSTNAMEFG";

#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    xterm: u8,
}

impl Rgb {
    fn new(xterm: u8) -> Self {
        let mut rgb = Rgb { r: 0, g: 0, b: 0, xterm };
        rgb.to_rgb();
        rgb
    }

    fn to_hex(self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    fn to_rgb(&mut self) {
        (self.r, self.g, self.b) = xterm_to_rgb(self.xterm);
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
            Arg::new("verbose")
                .help("Use more verbose output")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fgonly")
                .help("Only output the foreground color")
                .long("fgonly")
                .short('f')
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bgonly")
                .help("Only output the background color")
                .long("bgonly")
                .short('b')
                .action(clap::ArgAction::SetTrue),
        )
        .arg (
            Arg::new("iterm")
                .help("Output background color as iterm escape code to set tab background color")
                .long("iterm")
                .action(clap::ArgAction::SetTrue),
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
        .arg(
            Arg::new("noenv")
                .help("Do not use environment variables for input")
                .long("noenv")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bgcolor")
                .help("The background color to use")
                .long("bgcolor")
                .num_args(1)
                .default_value(None),
        )
        .arg(
            Arg::new("fgcolor")
                .help("The foreground color to use")
                .long("fgcolor")
                .num_args(1)
                .default_value(None),
        )
        .get_matches();

    let verbose_output = matches.get_flag("verbose");
    let fgonly_output = matches.get_flag("fgonly");
    let bgonly_output = matches.get_flag("bgonly");
    let iterm_output: bool = matches.get_flag("iterm");
    let hex_output = matches.get_flag("hex") || iterm_output;
    let noenv = matches.get_flag("noenv");

    if (fgonly_output as u8 + bgonly_output as u8 + verbose_output as u8 + iterm_output as u8) >= 2 {
        eprintln!("Error: Cannot specify more than one of --fgonly, --bgonly, --iterm, and --verbose");
        return;
    }

    let input_hostname = matches.get_one::<String>("hostname").map(|s| s.as_str());
    let input_theme = matches.get_one::<String>("theme").map(|s| s.as_str());

    let input_bgcolor = matches.get_one::<String>("bgcolor").map(|s| s.as_str());
    let mut bgcolor: Option<Rgb> = None;
    let input_fgcolor = matches.get_one::<String>("fgcolor").map(|s| s.as_str());
    let mut fgcolor: Option<Rgb> = None;
   
    if !noenv {
        if let Ok(env_bgcolor) = get_color_from_env(BGCOLOR_ENV_VAR) {
            bgcolor = Some(env_bgcolor);
        }
    }

    if let Some(bg) = input_bgcolor {
        bgcolor = Some(Rgb::new(bg.parse::<u8>().expect("Failed to parse background color")));
    }
    if !noenv && bgcolor.is_none() {
        if let Ok(env_bgcolor) = get_color_from_env(BGCOLOR_ENV_VAR) {
            bgcolor = Some(env_bgcolor);
        }
    }
    if bgcolor.is_none() {
            bgcolor = Some(calculate_hostname_background_color(input_hostname));
    }
    
    if bgcolor.is_none() {
        eprintln!("Error: Unstable state - failed to calculate background color");
        return;
    }
    let bgcolor = bgcolor;

    if let Some(fg) = input_fgcolor {
        fgcolor = Some(Rgb::new(fg.parse::<u8>().expect("Failed to parse foreground color")));
    }
    if !noenv && fgcolor.is_none() {
        if let Ok(env_fgcolor) = get_color_from_env(FGCOLOR_ENV_VAR) {
            fgcolor = Some(env_fgcolor);
        }
    }
    if fgcolor.is_none() {
            fgcolor = Some(calculate_hostname_foreground_color(bgcolor.unwrap(), input_theme).expect("Failed to calculate foreground color"));
    }
    
    if fgcolor.is_none() {
        eprintln!("Error: Unstable state - failed to calculate background color");
        return;
    }
    let fgcolor = fgcolor;

    output_colors(bgcolor.unwrap(), fgcolor.unwrap(), verbose_output, fgonly_output, bgonly_output, iterm_output, hex_output);
}

fn output_colors(bgcolor: Rgb, fgcolor: Rgb, verbose: bool, fgonly: bool, bgonly: bool, iterm: bool, hex: bool) {

    let (bgcolor_output, fgcolor_output) = if hex {
        (bgcolor.to_hex(), fgcolor.to_hex())
    } else {
        (bgcolor.xterm.to_string(), fgcolor.xterm.to_string())
    };

    if verbose {
        println!("bgcolor: {}, fgcolor: {}", bgcolor_output, fgcolor_output);
    } else if fgonly {
        println!("{}", fgcolor_output);
    } else if bgonly {
        println!("{}", bgcolor_output);
    } else if iterm {
        let bgcolor_hex = bgcolor.to_hex();
        let red = u8::from_str_radix(&bgcolor_hex[0..2], 16).unwrap();
        let green = u8::from_str_radix(&bgcolor_hex[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&bgcolor_hex[4..6], 16).unwrap();

        print!("\x1b]6;1;bg;red;brightness;{}\x07", red);
        print!("\x1b]6;1;bg;green;brightness;{}\x07", green);
        print!("\x1b]6;1;bg;blue;brightness;{}\x07", blue);
    } else {
        println!("{} {}", bgcolor_output, fgcolor_output);
    }
}

fn get_color_from_env(varname: &str) -> Result<Rgb, String> {
    let env_color = std::env::var(varname).ok();
    if let Some(bgcolor) = env_color {
        let xterm_color = match bgcolor.parse::<u8>() {
            Ok(color) => color,
            Err(_) => return Err(format!("Unable to parse value of environment variable {}={}", varname, bgcolor)),
        };
        return Ok(Rgb::new(xterm_color));
    }
    return Err(format!("Environment variable {} is not set", varname));
}

fn calculate_hostname_foreground_color(bgcolor: Rgb, theme: Option<&str>) -> Result<Rgb, String> {
    let theme = theme.unwrap_or("default");

    if !theme_maps::THEME_MAP.contains_key(theme) {
        eprintln!("Warning: Theme '{}' not found. Using default theme.", theme);
    }

    if let Some(map) = theme_maps::THEME_MAP.get(theme).or_else(|| theme_maps::THEME_MAP.get("default")) {
        if let Some(color) = map.get(&bgcolor.xterm) {
            return Ok(Rgb::new(*color));
        }
        return Err(format!("Color not found for color: {} theme: {} ", bgcolor.xterm, theme));
    }
    return Err(format!("Key not found for theme: {}", theme));
}

fn calculate_hostname_background_color(hostname: Option<&str>) -> Rgb {
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

    return Rgb::new(color_value);
}

fn xterm_to_rgb(xterm_color: u8) -> (u8, u8, u8) {
    // println!("xterm_color: {}", xterm_color);
    if xterm_color < 16 {
        let colors = vec![
            "000000", "cd0000", "00cd00", "cdcd00", "0000ee", "cd00cd", "00cdcd", "e5e5e5",
            "7f7f7f", "ff0000", "00ff00", "ffff00", "5c5cff", "ff00ff", "00ffff", "ffffff",
        ];

        let color = colors[xterm_color as usize];
        let r = u8::from_str_radix(&color[0..2], 16).unwrap();
        let g = u8::from_str_radix(&color[2..4], 16).unwrap();
        let b = u8::from_str_radix(&color[4..6], 16).unwrap();

        return  (r, g, b);
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

        return (r_color, g_color, b_color);
    }

    let gray = 8 + (xterm_color - 232) * 10;
    return (gray, gray, gray);
}