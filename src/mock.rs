//! Mock data generator for IPU3 Raw format testing.

/// Generates a dummy IPU3 packed 10-bit frame.
/// IPU3 format: 32 bytes for 25 pixels.
pub fn generate_mock_ipu3_frame(width: usize, height: usize) -> Vec<u8> {
    let num_pixels = width * height;
    let num_blocks = num_pixels / 25;
    let mut data = vec![0u8; num_blocks * 32];

    for i in 0..num_blocks {
        let block_offset = i * 32;
        // Fill MSBs with a gradient or pattern
        for j in 0..25 {
            let pixel_idx = i * 25 + j;
            let val = (pixel_idx % 256) as u8;
            data[block_offset + j] = val;
        }
        // Fill LSBs (random or constant)
        for j in 25..32 {
            data[block_offset + j] = 0xAA;
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unpack;

    #[test]
    fn test_mock_unpack() {
        let width = 640;
        let height = 480;
        let mock_data = generate_mock_ipu3_frame(width, height);
        let mut output = vec![0u8; width * height];
        
        unpack::unpack_ipu3_to_8bit(&mock_data, width, height, &mut output);
        
        // Verify a few pixels to ensure the pattern is preserved
        assert_eq!(output[0], 0);
        assert_eq!(output[1], 1);
        assert_eq!(output[24], 24);
        // Block boundaries: next pixel starts from block 1 (offset 32 in input)
        assert_eq!(output[25], 25 % 256);
    }
}
