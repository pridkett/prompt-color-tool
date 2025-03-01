use prompt_color_tool::{Rgb, calculate_hostname_background_color, calculate_hostname_foreground_color};

#[test]
fn test_hostname_background_color() {
    // Test with specific hostnames
    let color1 = calculate_hostname_background_color(Some("localhost"));
    let color2 = calculate_hostname_background_color(Some("server1"));
    
    // Different hostnames should produce different colors
    assert_ne!(color1.xterm, color2.xterm);
    
    // Colors should be in the valid range (0-127)
    assert!(color1.xterm < 128);
    assert!(color2.xterm < 128);
    
    // Same hostname should always produce the same color
    let color3 = calculate_hostname_background_color(Some("localhost"));
    assert_eq!(color1.xterm, color3.xterm);
}

#[test]
fn test_hostname_foreground_color() {
    // Create a background color
    let bgcolor = Rgb::new(18); // A dark blue
    
    // Test with default theme
    let result = calculate_hostname_foreground_color(bgcolor, Some("default"));
    assert!(result.is_ok());
    let fgcolor = result.unwrap();
    
    // Foreground should be readable on the background (this is a simplified test)
    assert_ne!(bgcolor.xterm, fgcolor.xterm);
    
    // Test with non-existent theme
    let result = calculate_hostname_foreground_color(bgcolor, Some("nonexistent_theme"));
    // This should still work because it falls back to default theme
    assert!(result.is_ok());
}
