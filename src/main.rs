use core::{
    rand::{rand, range, vec_range},
    terminal::{rgb, Element},
};
use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use sprites::Firework;

mod core;
mod sprites;

fn main() {
    use ffmpeg_next as ffmpeg;

    // Initialize ffmpeg
    ffmpeg::init().unwrap();

    let input_path = "never.mp4";

    // Open the input file
    let mut ictx = ffmpeg::format::input(&input_path).unwrap();

    // Find the best video stream
    let video_stream_index = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")
        .unwrap()
        .index();

    // Get the codec context for the video stream
    let params = ictx.streams().next().unwrap().parameters();
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(params).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    // .ok_or("Failed to get video stream")
    // .unwrap()
    // .codec()
    // .decoder()
    // .video()
    // .unwrap();

    // Create a scaler to convert frames to RGB
    let mut scaler = ffmpeg::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flag::Flags::BILINEAR,
    )
    .unwrap();

    let (cw, ch) = core::terminal::size().unwrap();

    // Decode frames
    let mut frame_index = 0;
    let mut frames_rgb = Vec::new();

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();

            let mut frame = ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut frame).is_ok() {
                // Scale the frame to RGB
                let mut rgb_frame = ffmpeg::frame::Video::empty();
                scaler.run(&frame, &mut rgb_frame).unwrap();

                // Convert the frame data to Vec<Vec<(u8, u8, u8)>>
                let width = rgb_frame.width() as usize;
                let height = rgb_frame.height() as usize;
                let data = rgb_frame.data(0);

                let mut rng = vec_range::<u8>(40, 100, cw * ch).into_iter();

                let mut frame_data = Vec::with_capacity(height);
                for y in 0..height {
                    if y >= ch {
                        break;
                    }
                    let row_start = y * width * 3;
                    let mut row = Vec::with_capacity(width);
                    for x in 0..width {
                        if x >= cw {
                            break;
                        }
                        let scale = |value: u8| (value as u16 * 5 / 256) as u8;

                        let r = data[row_start + x * 3];
                        let g = data[row_start + x * 3 + 1];
                        let b = data[row_start + x * 3 + 2];
                        let r = scale(r);
                        let g = scale(g);
                        let b = scale(b);
                        row.push(Element::new(rng.next().unwrap() as char, rgb(r, g, b), 0))
                    }
                    frame_data.extend(row);
                }

                frames_rgb.push(frame_data);
                println!("Processed frame {}", frame_index);
                frame_index += 1;
            }
        }
    }

    decoder.send_eof().unwrap();
    println!("Extracted {} frames", frames_rgb.len());

    // initialize terminal
    let handle = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let x = range(50, 150);
    let y = range(50, 80);
    let r = range(0, 5);
    let g = range(0, 5);
    let b = range(0, 5);

    let mut vec = vec![];
    for _ in 0..3 {
        let x = range(50, 150);
        let y = range(50, 80);
        let r = range(0, 5);
        let g = range(0, 5);
        let b = range(0, 5);

        let firework = Firework::new(x, y, r, g, b);
        vec.push(firework);
    }

    let mut input = [0u8; 1]; // Buffer for reading one byte at a time
    let mut i = 0;
    loop {
        let x = Instant::now();
        // // // Read one byte from stdin (should be non-blocking and immediate in raw mode)
        // std::io::Read::read_exact(&mut std::io::stdin(), &mut input).expect("Failed to read input");

        // // Check the pressed key and break if it's 'q'
        // if input[0] == b'q' {
        //     println!("\nYou pressed 'q'. Exiting...");
        //     break;
        // }

        // if input[0] == b'f' {
        //     println!("\nYou pressed 'f'. Fireworking...");
        // }

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        //let mut page = Vec::new();
        let fg = 10;
        let bg = 0;
        let mut rng = vec_range::<u8>(40, 100, width * height).into_iter();
        let perlin = core::rand::Perlin::new();

        core::terminal::display_raw(&frames_rgb[i]).unwrap();
        //core::terminal::display(&mut vec).unwrap();
        i += 1;

        let x = 1000 - x.elapsed().as_millis();
        // Print the raw key that was pressed (no echo in raw mode)
        std::thread::sleep(Duration::from_millis((x as f64 / 30.0) as u64));
    }

    drop(handle);
}
