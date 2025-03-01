use prompt_color_tool::{Rgb, output_colors_to_string};

#[test]
fn test_output_colors_normal() {
    let bgcolor = Rgb::new(18); // A dark blue
    let fgcolor = Rgb::new(15); // White
    
    // Test normal output (no flags)
    let output = output_colors_to_string(bgcolor, fgcolor, false, false, false, false, false);
    assert_eq!(output, "18 15");
    
    // Test with hex output
    let output = output_colors_to_string(bgcolor, fgcolor, false, false, false, false, true);
    assert_eq!(output, format!("{} {}", bgcolor.to_hex(), fgcolor.to_hex()));
}

#[test]
fn test_output_colors_flags() {
    let bgcolor = Rgb::new(18); // A dark blue
    let fgcolor = Rgb::new(15); // White
    
    // Test with verbose flag
    let output = output_colors_to_string(bgcolor, fgcolor, true, false, false, false, false);
    assert_eq!(output, "bgcolor: 18, fgcolor: 15");
    
    // Test with fgonly flag
    let output = output_colors_to_string(bgcolor, fgcolor, false, true, false, false, false);
    assert_eq!(output, "15");
    
    // Test with bgonly flag
    let output = output_colors_to_string(bgcolor, fgcolor, false, false, true, false, false);
    assert_eq!(output, "18");
    
    // Test with iterm flag
    let output = output_colors_to_string(bgcolor, fgcolor, false, false, false, true, true);
    assert!(output.contains("\x1b]6;1;bg;red;brightness;"));
    assert!(output.contains("\x1b]6;1;bg;green;brightness;"));
    assert!(output.contains("\x1b]6;1;bg;blue;brightness;"));
}
