use clap::Parser;
use image::{ImageBuffer, Rgb};
use show_image::{create_window, event, ImageInfo, ImageView};
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let image = image::open(&args.path).expect("todo");
    let image = image.into_rgb8();
    launch(digitize(image))
}

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn digitize(image: Image) -> Image {
    let c = 32;
    Image::from_raw(
        image.width(),
        image.height(),
        image.iter().map(|v| (v / c) * c).collect(),
    )
    .unwrap()
}

fn launch(image: Image) -> Result<(), Box<dyn std::error::Error>> {
    let image = ImageView::new(ImageInfo::rgb8(image.width(), image.height()), &image);
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    for event in window.event_channel().unwrap() {
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
