//! IPU3 10-bit packed Bayer unpacking engine.
//! 
//! The IPU3 raw format packs 25 pixels into 32 bytes.
//! - First 25 bytes: 8 most significant bits (MSB) of each pixel.
//! - Last 7 bytes: 2 least significant bits (LSB) of each pixel, concatenated.

/// Unpacks IPU3 10-bit packed Bayer data into 8-bit grayscale.
/// Stride-aware: Handles 832 bytes per line (26 blocks of 32 bytes).
/// Official IPU3 layout: 25 pixels packed into 32 bytes (250 bits + 6 bits padding).
/// This implementation extracts the 8 most significant bits for each pixel.
pub fn unpack_ipu3_to_8bit(packed: &[u8], width: usize, height: usize, output: &mut [u8]) {
    let bytes_per_line = 832;
    // Each 32-byte block contains 25 pixels.
    // 640 pixels = 25.6 blocks. In reality, 26 blocks (832 bytes) are provided.
    
    for y in 0..height {
        let line_offset = y * bytes_per_line;
        let out_row_offset = y * width;
        
        let mut pixels_unpacked = 0;
        let mut byte_idx = 0;
        
        while pixels_unpacked < width && byte_idx < bytes_per_line {
            let src = &packed[line_offset + byte_idx..];
            
            // IPU3 bit layout (simplified for 8-bit MSB extraction):
            // Every 5 bytes contain 4 pixels (40 bits).
            // Byte 0: Pix0_low (8)
            // Byte 1: Pix1_low (6), Pix0_high (2) -> Pix0_high is MSB of Pix0
            // This suggests my previous "MSB first" was slightly off in bit-order.
            
            // Re-analyzing the kernel doc:
            // start + 0: B0000low (Bit 7-0 of Byte 0 are NOT the MSBs of Pixel 0)
            // It says "bits 1--0" of Byte 1 are "B0000high".
            // So Pix0 = (Byte 1 & 0x03) << 8 | Byte 0.
            
            if byte_idx + 5 <= bytes_per_line {
                // Pixel 0
                if pixels_unpacked < width {
                    let val = ((src[1] as u16 & 0x03) << 8) | (src[0] as u16);
                    output[out_row_offset + pixels_unpacked] = (val >> 2) as u8;
                    pixels_unpacked += 1;
                }
                // Pixel 1
                if pixels_unpacked < width {
                    let val = ((src[2] as u16 & 0x0F) << 6) | (src[1] as u16 >> 2);
                    output[out_row_offset + pixels_unpacked] = (val >> 2) as u8;
                    pixels_unpacked += 1;
                }
                // Pixel 2
                if pixels_unpacked < width {
                    let val = ((src[3] as u16 & 0x3F) << 4) | (src[2] as u16 >> 4);
                    output[out_row_offset + pixels_unpacked] = (val >> 2) as u8;
                    pixels_unpacked += 1;
                }
                // Pixel 3
                if pixels_unpacked < width {
                    let val = ((src[4] as u16) << 2) | (src[3] as u16 >> 6);
                    output[out_row_offset + pixels_unpacked] = (val >> 2) as u8;
                    pixels_unpacked += 1;
                }
                
                byte_idx += 5;
                
                // Every 25 pixels, there is a skip or wrap because it's 32-byte blocks.
                // 25 pixels = 6.25 iterations of 4-pixel groups.
                // Actually, the simplest way is to iterate blocks of 32 bytes and pixels of 25.
            } else {
                break;
            }
        }
    }
}

/// Corrected Unpacker based on linuxtv.org IPU3 spec:
/// 25 pixels in 32 bytes.
pub fn unpack_ipu3_v2(packed: &[u8], width: usize, height: usize, output: &mut [u8]) {
    let bytes_per_line = 832;
    for y in 0..height {
        let line_src = &packed[y * bytes_per_line..(y + 1) * bytes_per_line];
        let line_dst = &mut output[y * width..(y + 1) * width];
        
        // Each line has 26 blocks of 32 bytes.
        // Each block contains 25 pixels (250 bits + 6 bits padding).
        for block in 0..26 {
            let src_offset = block * 32;
            let dst_offset = block * 25;
            
            if src_offset + 32 > bytes_per_line { break; }
            
            let b = &line_src[src_offset..src_offset + 32];
            
            // Process 6 groups of 4 pixels (30 bytes)
            for g in 0..6 {
                let s = g * 5;
                let d = dst_offset + (g * 4);
                
                if d + 4 > width { break; }
                
                let p0 = (b[s] as u16) | ((b[s+1] as u16 & 0x03) << 8);
                let p1 = (b[s+1] as u16 >> 2) | ((b[s+2] as u16 & 0x0F) << 6);
                let p2 = (b[s+2] as u16 >> 4) | ((b[s+3] as u16 & 0x3F) << 4);
                let p3 = (b[s+3] as u16 >> 6) | ((b[s+4] as u16) << 2);
                
                line_dst[d] = (p0 >> 2) as u8;
                line_dst[d + 1] = (p1 >> 2) as u8;
                line_dst[d + 2] = (p2 >> 2) as u8;
                line_dst[d + 3] = (p3 >> 2) as u8;
            }
            
            // The 25th pixel (Pixel 24) is in bytes 30 and 31 (bits 0-1)
            let p24_idx = dst_offset + 24;
            if p24_idx < width {
                let p24 = (b[30] as u16) | ((b[31] as u16 & 0x03) << 8);
                line_dst[p24_idx] = (p24 >> 2) as u8;
            }
        }
    }
}

/// (Future) Unpacks IPU3 10-bit packed Bayer data into 16-bit values.
/// This will include the 2-bit LSBs if higher precision is required for ISP steps.
pub fn unpack_ipu3_to_16bit(packed: &[u8], width: usize, height: usize, output: &mut [u16]) {
    let num_blocks = (width * height) / 25;
    
    for i in 0..num_blocks {
        let block_offset = i * 32;
        let out_offset = i * 25;
        
        // Extract LSBs from the 7 bytes at the end of the block
        // Byte 25..31 contain the LSBs.
        // This is more complex and will be implemented if 8-bit is insufficient.
        for j in 0..25 {
            output[out_offset + j] = (packed[block_offset + j] as u16) << 2;
        }
    }
}

pub fn rotate_90_clockwise(input: &[u8], width: usize, height: usize, output: &mut [u8]) {
    for y in 0..height {
        let input_row_offset = y * width;
        for x in 0..width {
            // New coordinates after 90 deg rotation: (x, y) -> (height - 1 - y, x)
            // Note: Output width = Input height, Output height = Input width
            let new_x = height - 1 - y;
            let new_y = x;
            output[new_y * height + new_x] = input[input_row_offset + x];
        }
    }
}

/// Simple Histogram Equalization for 8-bit grayscale image
pub fn equalize_histogram(image: &mut [u8]) {
    let mut hist = [0usize; 256];
    for &px in image.iter() {
        hist[px as usize] += 1;
    }

    let mut cdf = [0usize; 256];
    cdf[0] = hist[0];
    for i in 1..256 {
        cdf[i] = cdf[i - 1] + hist[i];
    }

    let total_pixels = image.len() as f32;
    let min_cdf = cdf.iter().find(|&&x| x > 0).copied().unwrap_or(0) as usize;
    
    // REFINED GUARD: Only block if the image is EXTREMELY flat (pitch black).
    // A value of 3 provides some buffer for noise vs real signal.
    let max_val = image.iter().max().copied().unwrap_or(0);
    if (max_val as usize) < (min_cdf + 3) || total_pixels <= (min_cdf as f32) {
        return;
    }

    let mut lut = [0u8; 256];
    let min_cdf_f = min_cdf as f32;
    for i in 0..256 {
        lut[i] = (((cdf[i] as f32 - min_cdf_f) / (total_pixels - min_cdf_f)) * 255.0) as u8;
    }

    for px in image.iter_mut() {
        *px = lut[*px as usize];
    }
}

/// Real-time 3x3 Mean Filter for denoising.
/// Smooths out the grainy noise typical of raw sensors at high gain.
pub fn apply_mean_filter_3x3(input: &[u8], width: usize, height: usize, output: &mut [u8]) {
    // Copy input to output first (to handle borders simply)
    output.copy_from_slice(input);
    
    for y in 1..(height - 1) {
        let row_offset = y * width;
        let prev_row = (y - 1) * width;
        let next_row = (y + 1) * width;
        
        for x in 1..(width - 1) {
            let mut sum: u32 = 0;
            
            // 3x3 neighborhood
            sum += input[prev_row + x - 1] as u32;
            sum += input[prev_row + x] as u32;
            sum += input[prev_row + x + 1] as u32;
            
            sum += input[row_offset + x - 1] as u32;
            sum += input[row_offset + x] as u32;
            sum += input[row_offset + x + 1] as u32;
            
            sum += input[next_row + x - 1] as u32;
            sum += input[next_row + x] as u32;
            sum += input[next_row + x + 1] as u32;
            
            output[row_offset + x] = (sum / 9) as u8;
        }
    }
}

