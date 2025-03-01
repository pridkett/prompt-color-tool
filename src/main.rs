use clap::{Command, Arg};
use prompt_color_tool::{
    Rgb, 
    BGCOLOR_ENV_VAR, 
    FGCOLOR_ENV_VAR,
    calculate_hostname_background_color,
    calculate_hostname_foreground_color,
    get_color_from_env,
    output_colors_to_string
};

mod theme_maps;

fn main() {
    let matches = Command::new("prompt-color-tool")
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
    let output = output_colors_to_string(bgcolor, fgcolor, verbose, fgonly, bgonly, iterm, hex);
    print!("{}", output);
}
