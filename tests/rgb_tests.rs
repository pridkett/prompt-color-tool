use prompt_color_tool::{Rgb, xterm_to_rgb};

#[test]
fn test_rgb_new() {
    let rgb = Rgb::new(0); // Black
    assert_eq!(rgb.r, 0);
    assert_eq!(rgb.g, 0);
    assert_eq!(rgb.b, 0);
    assert_eq!(rgb.xterm, 0);

    let rgb = Rgb::new(15); // White
    assert_eq!(rgb.r, 255);
    assert_eq!(rgb.g, 255);
    assert_eq!(rgb.b, 255);
    assert_eq!(rgb.xterm, 15);
}

#[test]
fn test_to_hex() {
    let rgb = Rgb { r: 0, g: 0, b: 0, xterm: 0 };
    assert_eq!(rgb.to_hex(), "000000");

    let rgb = Rgb { r: 255, g: 255, b: 255, xterm: 15 };
    assert_eq!(rgb.to_hex(), "ffffff");

    let rgb = Rgb { r: 205, g: 0, b: 0, xterm: 1 };
    assert_eq!(rgb.to_hex(), "cd0000");
}

#[test]
fn test_to_rgb() {
    let mut rgb = Rgb { r: 0, g: 0, b: 0, xterm: 1 };
    rgb.to_rgb();
    assert_eq!(rgb.r, 205);
    assert_eq!(rgb.g, 0);
    assert_eq!(rgb.b, 0);

    let mut rgb = Rgb { r: 0, g: 0, b: 0, xterm: 22 }; // A color from the 6x6x6 cube
    rgb.to_rgb();
    assert_eq!(rgb.r, 0);
    assert_eq!(rgb.g, 95);
    assert_eq!(rgb.b, 95);
}

#[test]
fn test_xterm_to_rgb() {
    // Test basic colors (0-15)
    assert_eq!(xterm_to_rgb(0), (0, 0, 0)); // Black
    assert_eq!(xterm_to_rgb(1), (205, 0, 0)); // Red
    assert_eq!(xterm_to_rgb(15), (255, 255, 255)); // White

    // Test color cube (16-231)
    assert_eq!(xterm_to_rgb(16), (0, 0, 0)); // Start of color cube
    assert_eq!(xterm_to_rgb(21), (0, 0, 255)); // Blue in color cube
    assert_eq!(xterm_to_rgb(46), (0, 255, 0)); // Green in color cube
    assert_eq!(xterm_to_rgb(231), (255, 255, 255)); // End of color cube

    // Test grayscale (232-255)
    assert_eq!(xterm_to_rgb(232), (8, 8, 8)); // Start of grayscale
    assert_eq!(xterm_to_rgb(243), (118, 118, 118)); // Middle of grayscale
    assert_eq!(xterm_to_rgb(255), (238, 238, 238)); // End of grayscale
}
