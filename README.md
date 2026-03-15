# Surface Pro 5 IR Camera Pipeline (Rust)

A high-performance, stable 30FPS IR camera pipeline with hardware-synchronized IR LED control for the Microsoft Surface Pro 5 (OV7251 sensor). This project is designed specifically to bring stable, bright IR video to Linux, enabling facial recognition tools like [Howdy](https://github.com/boltgolt/howdy).

## Features

- **Stable 30FPS**: Normalizes sensor vertical blanking to guarantee a stutter-free, fluid video stream.
- **Hardware IR LED Control**: Bypasses driver limitations to manually strobe the IR LED via direct I2C injection.
- **Super-Boost Brightness**: Dynamically scales sensor exposure lines (up to 1500) and analog gain (up to 8x) for clear imagery even in pitch darkness.
- **Low-Latency ISP**: A dedicated Rust unpacker that converts Intel IPU3 10-bit raw packed frames into standard BGRA format in real-time.
- **Universal compatibility**: Outputs to a `v4l2loopback` virtual device (`/dev/video20`), making the IR camera usable by any standard Linux video application (`ffplay`, `fswebcam`, browsers).

## Prerequisites

This pipeline requires a Linux environment (tested heavily on Fedora with the linux-surface kernel). Install the necessary dependencies:

```bash
sudo dnf install rust cargo v4l-utils i2c-tools v4l2loopback
```

Ensure the `v4l2loopback` kernel module is loaded:
```bash
sudo modprobe v4l2loopback devices=1 video_nr=20 card_label="Surface IR Camera" exclusive_caps=1
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Yourumark/surface-IR-rust.git
cd surface-IR-rust
```

2. Build the main ISP and the injector daemon:
```bash
# Build the main ISP (raw frame unpacker)
cargo build --release

# Build the I2C scanner/injector daemon
cd scanner
cargo build --release
cd ..
```

## Usage

Start the pipeline using the provided shell script. You can pass a brightness level between `0` and `255`. 
- `64` is recommended for standard indoor use. 
- `180` activates "Super-Boost" for dark environments.

```bash
./run_capture.sh 180
```

Once running, you can view the live stream:
```bash
ffplay -f video4linux2 -pixel_format bgr0 -video_size 640x480 -i /dev/video20
```

## How It Works (The Technical Challenge)

The Surface Pro 5 uses the Intel IPU3 image coprocessor. The upstream Linux drivers do not officially map or support toggling the privacy/IR LED on the OV7251 sensor.

To solve this, we inject raw I2C register configurations (specifically the `0x3bxx` strobe registers) directly into the sensor on I2C bus 3. However, this presented a massive challenge:
- **The Stutter/Freeze Problem:** Continuously rewriting I2C registers to force the LED on while the IPU3 CSI-2 receiver is actively streaming causes `DPHY synchronization errors`. This results in the video queue dropping to `0` and the stream freezing periodically for ~1 second.
- **The "Stay Off" Problem:** If we inject the I2C configuration *before* the stream starts, the IPU3 driver's internal initialization routine resets the OV7251 sensor, wiping our custom IR configuration entirely.

**The Solution:** 
We implemented a **Delayed One-Shot Injector**. The `scanner` daemon waits exactly 2.5 seconds *after* the `v4l2-ctl` stream is requested. This gives the IPU3 driver enough time to finish its initialization and resets. The injector then fires its I2C configuration exactly once. 

This strict hardware synchronization guarantees that the IR LED turns on, stays on, and maintains a perfectly smooth, freeze-free 30FPS stream without any I2C bus contention!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies & Acknowledgments

This project heavily relies on the following open-source tools. We respect their respective licenses and thank the maintainers:
- [v4l-utils](https://git.linuxtv.org/v4l-utils.git) (GPL-2.0)
- [i2c-tools](https://git.kernel.org/pub/scm/utils/i2c-tools/i2c-tools.git) (GPL-2.0)
- [v4l2loopback](https://github.com/umlaeute/v4l2loopback) (GPL-2.0)
- Built with the Rust [v4l](https://crates.io/crates/v4l) crate (MIT/Apache-2.0)
