#!/bin/bash
# run_capture.sh: Safe unified pipeline to start and stop both the hardware-sync injector and V4L stream.

echo ">>> Starting Hardware-Synchronized IR Pipeline..."

# 1. Start the Rust I2C injector in the background
# Accept brightness (pulse width) as first arg, default to 16 (0x10)
BRIGHTNESS=${1:-16}
echo ">>> Using IR Brightness Level: $BRIGHTNESS (0-255)"
sudo ./scanner/target/release/scanner "$BRIGHTNESS" &
SCANNER_PID=$!

# 2. Setup a trap to catch Ctrl+C (SIGINT) and kill the scanner automatically
trap 'echo ""; echo ">>> Ctrl+C detected. Shutting down pipeline..."; sudo kill -9 $SCANNER_PID 2>/dev/null; exit' INT TERM EXIT

# 3. Dynamically discover the video node setup by final_ir_setup.sh
# Check if /tmp/ir_node exists
if [[ -f "/tmp/ir_node" ]]; then
    FOUND_VDEV=$(cat /tmp/ir_node)
else
    FOUND_VDEV="/dev/video14" # Fallback
fi

echo ">>> Capturing video on $FOUND_VDEV"

# 3.5 Dynamically locate the OV7251 subdevice and lower exposure/gain to prevent blowout
OV_SUBDEV=$(media-ctl -d /dev/media2 -e "ov7251 3-0060" 2>/dev/null)
if [ -n "$OV_SUBDEV" ]; then
    echo ">>> Normalizing OV7251 timing for 30FPS & Tuning Exposure..."
    sudo v4l2-ctl -d "$OV_SUBDEV" -c vertical_blanking=1244 -c exposure=1000 -c analogue_gain=128 2>/dev/null || true
fi

# 4. Launch the reliable v4l2-ctl multiplanar stream mapped into the ISP Unpacker
# Using the optimized PROD (--release) build for smooth 30+ FPS!
sudo v4l2-ctl -d "$FOUND_VDEV" --stream-mmap --stream-to=- | ./target/release/surface-IR-rust --stdin
