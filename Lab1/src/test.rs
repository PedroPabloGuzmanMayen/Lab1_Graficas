#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex(0xFF8040);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.to_hex(), 0xFF8040);
    }

    #[test]
    fn test_color_add() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(50, 100, 55);
        let result = color1 + color2;
        assert_eq!(result, Color::new(150, 250, 255)); // RGB values are clamped to 255
    }

    #[test]
    fn test_color_mul() {
        let color = Color::new(100, 150, 200);
        let result = color * 2; // changed to an integer factor
        assert_eq!(result, Color::new(200, 255, 255)); // RGB values are clamped to 255
    }

    #[test]
    fn test_color_display() {
        let color = Color::new(255, 128, 64);
        assert_eq!(format!("{}", color), "Color(r: 255, g: 128, b: 64)");
    }

    #[test]
    fn framebuffer_correctly_initialized() {
        let frame = FrameBuffer::new(25, 25);
        let color = Color::new(255, 255, 255);
        assert_eq!(color, frame.current_color);
    }

    #[test]
    fn test_clear() {
        let mut frame = FrameBuffer::new(10, 10);
        frame.set_background_color(Color::new(0, 0, 0));
        frame.clear();
        for color in frame.buffer.iter() {
            assert_eq!(*color, frame.background_color);
        }
    }

    #[test]
    fn test_point() {
        let mut frame = FrameBuffer::new(10, 10);
        frame.set_current_color(Color::new(255, 0, 0));
        frame.point(1, 1);
        assert_eq!(frame.buffer[11], frame.current_color);
    }

    #[test]
    fn test_set_background_color() {
        let mut frame = FrameBuffer::new(10, 10);
        let new_color = Color::new(0, 0, 0);
        frame.set_background_color(new_color);
        assert_eq!(frame.background_color, new_color);
    }

    #[test]
    fn test_set_current_color() {
        let mut frame = FrameBuffer::new(10, 10);
        let new_color = Color::new(0, 255, 0);
        frame.set_current_color(new_color);
        assert_eq!(frame.current_color, new_color);
    }
}
