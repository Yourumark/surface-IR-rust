use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

// Hardcoded for Surface Pro 5 OV7251 topology
const I2C_BUS: &str = "3";
const I2C_ADDR: &str = "0x60";

/// Helper to execute i2ctransfer with multiple registers in a single transaction
fn write_i2c_batch(regs: &[(u16, u8)]) {
    let mut args = vec!["-f".to_string(), "-y".to_string(), I2C_BUS.to_string()];
    
    for (reg, val) in regs {
        let reg_h = format!("0x{:02x}", (reg >> 8) & 0xFF);
        let reg_l = format!("0x{:02x}", reg & 0xFF);
        let val_str = format!("0x{:02x}", val);
        args.push(format!("w3@{}", I2C_ADDR));
        args.push(reg_h);
        args.push(reg_l);
        args.push(val_str);
    }

    let output = Command::new("i2ctransfer").args(&args).output();

    if let Ok(out) = output {
        if !out.status.success() {
            eprintln!("[I2C Error] Batch write failed: {:?}", String::from_utf8_lossy(&out.stderr));
        }
    }
}

fn inject_optimized_strobe(pulse_width: u8) {
    // 30FPS Normalization:
    // Vertical Blanking must be ~1244 for 30FPS at default pixel rate.
    // 1244 = 0x04DC
    
    // Level 0-255 Mapping:
    // We want a steep but smooth climb.
    let level = pulse_width as f32 / 255.0;
    
    // Exposure: 10 lines (min) to 1500 lines (max for 30fps)
    let exp_lines = (10.0 + (level * 1490.0)) as u16;
    
    // Gain: 1x (0x100) to 8x (0x800)
    let gain = (0x0100 as f32 + (level * 0x0700 as f32)) as u16;

    let mut batch = Vec::new();

    // 1. Timing Normalization (Fix stuttering)
    batch.push((0x380e, 0x04)); // V-Blank HI
    batch.push((0x380f, 0xdc)); // V-Blank LO

    // 2. Hardware Strobe Pin Configuration
    batch.push((0x3005, 0x08)); // Strobe = Output
    batch.push((0x3026, 0x03)); // PWM Sync
    batch.push((0x3027, 0x02)); // PWM Route

    // 3. Brightness Injection (Super-Boost)
    batch.push((0x3501, ((exp_lines >> 8) & 0xff) as u8));
    batch.push((0x3502, (exp_lines & 0xff) as u8));
    batch.push((0x3508, ((gain >> 8) & 0xff) as u8));
    batch.push((0x3509, (gain & 0xff) as u8));

    // 4. PWM Configuration
    batch.push((0x3b80, 0x00)); // Period HI
    batch.push((0x3b81, 0xFF)); // Period LO
    batch.push((0x3b82, 0x00)); // Duty HI
    batch.push((0x3b83, pulse_width)); // Duty LO

    // 5. Logic Enable
    batch.push((0x3b00, 0x01)); 
    batch.push((0x3b03, pulse_width));

    write_i2c_batch(&batch);
    println!("[Injector] Super-Boost Lvl {} -> Exp: {} lines, Gain: 0x{:04x}", pulse_width, exp_lines, gain);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pulse_width: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(64);

    println!(">>> Surface-IR-Rust: 30FPS Delayed One-Shot Injector Level {}", pulse_width);

    // CRITICAL FIX: We must wait for `v4l2-ctl` to fully initialize the stream.
    // When the stream starts, the IPU3 driver resets the OV7251 registers. 
    // If we inject too early, our settings are wiped out (IR stays off).
    // If we inject continuously, the I2C bus contends with the IPU3 (Stuttering).
    // Solution: Wait 2.5 seconds, then fire exactly once.
    println!("[Injector] Waiting 2.5s for IPU3 stream initialization...");
    thread::sleep(Duration::from_millis(2500));
    
    inject_optimized_strobe(pulse_width);

    println!(">>> Injection complete. Stream should remain stable. Exiting injector.");
}
