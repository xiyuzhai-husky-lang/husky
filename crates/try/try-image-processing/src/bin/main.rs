use show_image::{create_window, event, ImageInfo, ImageView};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width: u32 = 1024;
    let height: u32 = 1024;
    let pixel_data = new_512_colors_pixel_data();
    let image = ImageView::new(ImageInfo::rgb8(width, height), &pixel_data);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
    Ok(())
}

fn new_512_colors_pixel_data() -> Vec<u8> {
    const UNIT: usize = 2;
    let l = UNIT * 64 * 8;
    let mut pixel_data: Vec<u8> = (0..(l * l * 3)).into_iter().map(|_| 0).collect();
    for ir in 0..64 {
        for ig in 0..64 {
            for ib in 0..64 {
                let iu = (ib / 8) * 64 + ir;
                let ju = (ib % 8) * 64 + ig;
                for di in 0..UNIT {
                    for dj in 0..UNIT {
                        let i = iu * UNIT + di;
                        let j = ju * UNIT + dj;
                        pixel_data[3 * (i * l + j) + 0] = (ir as u8) * 4;
                        pixel_data[3 * (i * l + j) + 1] = (ig as u8) * 4;
                        pixel_data[3 * (i * l + j) + 2] = (ib as u8) * 4;
                    }
                }
            }
        }
    }
    pixel_data
}

#[test]
fn all_colors_are_there() {
    let pixel_data = new_512_colors_pixel_data();
    assert!(pixel_data_contains(&pixel_data, 252, 252, 0));
    assert!(pixel_data_contains(&pixel_data, 252, 0, 252));
    assert!(pixel_data_contains(&pixel_data, 0, 252, 252));
}

fn pixel_data_contains(pixel_data: &[u8], r: u8, g: u8, b: u8) -> bool {
    for i in 0..pixel_data.len() / 3 {
        if r == pixel_data[i * 3 + 0] && g == pixel_data[i * 3 + 1] && b == pixel_data[i * 3 + 2] {
            return true;
        }
    }
    false
}
