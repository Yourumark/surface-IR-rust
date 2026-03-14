mod unpack;
mod mock;

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    // Configuration
    let args: Vec<String> = env::args().collect();
    let use_mock = args.contains(&"--mock".to_string());
    let use_stdin = args.contains(&"--stdin".to_string());
    
    let output_node = "/dev/video20"; // v4l2loopback sink

    println!("Starting Surface IR-Rust ISP...");
    if use_mock {
        println!("MODE: MOCK (Safe software-only test)");
    } else if use_stdin {
        println!("MODE: STDIN (Reading raw frames from stdin)");
    } else {
        println!("MODE: HARDWARE (Direct V4L2 - Might require library fix)");
    }
    println!("Output: {}", output_node);
    let openers_path = "/sys/class/video4linux/video20/openers";

    let mut out_file = match File::options().write(true).open(output_node) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open output device {}: {}. Is v4l2loopback loaded?", output_node, e);
            return Err(e);
        }
    };

    let width = 640;
    let height = 480;
    
    // IPU3 Packed format size (832 bytes per line)
    let bytes_per_line = 832;
    let frame_size = bytes_per_line * height;

    let mut raw_8bit = vec![0u8; width * height];
    let mut rotated_frame = vec![0u8; width * height];
    let mut rgba_frame = vec![0u8; width * height * 4];

    if use_mock {
        println!("ISP Engine starting (Mock)...");
        loop {
            let buf = mock::generate_mock_ipu3_frame(width, height);
            unpack::unpack_ipu3_v2(&buf, width, height, &mut raw_8bit);
            process_and_output(&mut raw_8bit, width, height, &mut rotated_frame, &mut rgba_frame, &mut out_file)?;
            std::thread::sleep(std::time::Duration::from_millis(33)); // ~30 FPS
        }
    } else if use_stdin {
        println!("ISP Engine starting (Stdin Ingestion)...");
        let mut stdin = std::io::stdin().lock();
        let mut buf = vec![0u8; frame_size];

        loop {
            if let Err(e) = stdin.read_exact(&mut buf) {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("End of stream reached.");
                    break;
                }
                return Err(e);
            }
            
            // NOTE: Hardware control (IR LED) is now strictly handled by the 'scanner' injector daemon.
            // We removed the 2s consumer check and led_control.sh calls to prevent I2C bus contention
            // and CSI-2 (DPHY) sync errors during streaming.
            
            // Use the high-precision bitstream unpacker
            unpack::unpack_ipu3_v2(&buf, width, height, &mut raw_8bit);
            
            // FOR VERIFICATION: Skip rotation, output at native 640x480 to match /dev/video20
            process_and_output_debug(&mut raw_8bit, width, height, &mut rgba_frame, &mut out_file)?;
        }
    } else {
        eprintln!("Direct hardware mode is deprecated for IPU3. Please use --stdin with v4l2-ctl.");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Use --stdin mode"));
    }

    Ok(())
}

fn process_and_output(
    raw_8bit: &mut [u8], 
    width: usize, 
    height: usize, 
    rotated: &mut [u8], 
    rgba: &mut [u8], 
    out: &mut File
) -> std::io::Result<()> {
    // 1. ISP Steps
    // Disable histogram equalization to prevent blowing out bright IR highlights
    // unpack::equalize_histogram(raw_8bit);
    unpack::rotate_90_clockwise(raw_8bit, width, height, rotated);
    
    // 2. Format conversion (Gray8 -> BGRA)
    for (i, &gray) in rotated.iter().enumerate() {
        let base = i * 4;
        rgba[base] = gray;     // B
        rgba[base + 1] = gray; // G
        rgba[base + 2] = gray; // R
        rgba[base + 3] = 255;  // A
    }

    // 3. Output to virtual camera
    out.write_all(rgba)?;
    out.flush()?;
    Ok(())
}

fn process_and_output_debug(
    raw_8bit: &mut [u8], 
    width: usize, 
    height: usize, 
    rgba: &mut [u8], 
    out: &mut File
) -> std::io::Result<()> {
    // 1. Denoising (Mean Filter 3x3)
    let mut denoised = vec![0u8; raw_8bit.len()];
    unpack::apply_mean_filter_3x3(raw_8bit, width, height, &mut denoised);

    // 2. ISP Steps
    // Disable histogram equalization to prevent blowing out bright IR highlights
    // unpack::equalize_histogram(&mut denoised);
    
    // 3. Format conversion (Gray8 -> BGRA)
    for (i, &gray) in denoised.iter().enumerate() {
        let base = i * 4;
        rgba[base] = gray;     // B
        rgba[base + 1] = gray; // G
        rgba[base + 2] = gray; // R
        rgba[base + 3] = 255;  // A
    }

    // 4. Output to virtual camera
    out.write_all(rgba)?;
    out.flush()?;
    Ok(())
}
