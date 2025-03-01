use md5::{Md5, Digest};
use std::process::Command as ProcessCommand;

pub mod theme_maps;

pub const BGCOLOR_ENV_VAR: &str = "PLGO_HOSTNAMEBG";
pub const FGCOLOR_ENV_VAR: &str = "PLGO_HOSTNAMEFG";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub xterm: u8,
}

impl Rgb {
    pub fn new(xterm: u8) -> Self {
        let mut rgb = Rgb { r: 0, g: 0, b: 0, xterm };
        rgb.to_rgb();
        rgb
    }

    pub fn to_hex(self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_rgb(&mut self) {
        (self.r, self.g, self.b) = xterm_to_rgb(self.xterm);
    }
}

pub fn xterm_to_rgb(xterm_color: u8) -> (u8, u8, u8) {
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

pub fn calculate_hostname_background_color(hostname: Option<&str>) -> Rgb {
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

pub fn calculate_hostname_foreground_color(bgcolor: Rgb, theme: Option<&str>) -> Result<Rgb, String> {
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

pub fn get_color_from_env(varname: &str) -> Result<Rgb, String> {
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

pub fn output_colors_to_string(bgcolor: Rgb, fgcolor: Rgb, verbose: bool, fgonly: bool, bgonly: bool, iterm: bool, hex: bool) -> String {
    let (bgcolor_output, fgcolor_output) = if hex {
        (bgcolor.to_hex(), fgcolor.to_hex())
    } else {
        (bgcolor.xterm.to_string(), fgcolor.xterm.to_string())
    };

    if verbose {
        return format!("bgcolor: {}, fgcolor: {}", bgcolor_output, fgcolor_output);
    } else if fgonly {
        return fgcolor_output;
    } else if bgonly {
        return bgcolor_output;
    } else if iterm {
        let bgcolor_hex = bgcolor.to_hex();
        let red = u8::from_str_radix(&bgcolor_hex[0..2], 16).unwrap();
        let green = u8::from_str_radix(&bgcolor_hex[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&bgcolor_hex[4..6], 16).unwrap();

        return format!(
            "\x1b]6;1;bg;red;brightness;{}\x07\x1b]6;1;bg;green;brightness;{}\x07\x1b]6;1;bg;blue;brightness;{}\x07",
            red, green, blue
        );
    } else {
        return format!("{} {}", bgcolor_output, fgcolor_output);
    }
}
