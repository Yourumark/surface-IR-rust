# Technical Architecture & Pipeline Flow: Surface Pro 5 IR Camera

This document details the hardware and software architecture of the Surface Pro 5 IR Camera pipeline, specifically outlining the initialization, streaming, and hardware control phases necessary to achieve a stable, properly exposed video output.

## Overview
The Surface Pro 5 uses an **OmniVision OV7251** IR sensor connected via MIPI CSI-2 to an **Intel IPU3** (Image Processing Unit 3). In mainline Linux kernels (via the `linux-surface` project), the IPU3 driver manages the sensor but lacks the vendor-specific algorithms to internally turn on the sensor's IR privacy LED or drive the ISP for usable user-facing video.

To overcome this, our pipeline acts as a bridge, manually managing sensor timing, hardware illumination (I2C injection), and software-based unpacking (IPU3 native 10-bit packed to standard 8-bit BGRA).

---

## The System Architecture

The pipeline consists of three discrete layers:
1. **The Kernel Data Path (V4L2):** 
   - `ov7251`: The physical subdevice node (`/dev/v4l-subdevX`).
   - `ipu3-csi2`: The MIPI receiver.
   - `ipu3-cio2`: The DMA engine outputting raw frames to `/dev/video14`.
   - `v4l2loopback`: The virtual sink presenting the final video at `/dev/video20`.
2. **The Hardware Injector (`scanner`):**
   - A Rust daemon that bypasses the kernel's V4L2 abstraction to write directly to the physical I2C bus (Bus 3, Address `0x60`). It forces the IR LED on and sets "Super-Boost" exposure/gain.
3. **The Software ISP (`surface-IR-rust`):**
   - A high-performance Rust engine that consumes raw IPU3 stream data via standard input (`stdin`), unpacks the custom 10-bit format into 8-bit grayscale, rotates the image physically, converts it to 32-bit BGRA, and streams it to the virtual loopback device.

---

## Step-by-Step Initialization & Streaming Flow

The successful operation of the camera is governed by a strict sequence of events. If any condition fails or happens out of order, the stream will freeze, or the LED will remain off.

### Phase 1: Subdevice & Topology Configuration
Before any stream begins, the physical topology must be linked and configured using `media-ctl` and `v4l2-ctl`.
*   **Action**: Link `ov7251` -> `ipu3-csi2` -> `ipu3-cio2`.
*   **Action**: Set the physical capture format on the subdevice to `SGRBG10_1X10` (Bayer 10-bit) at 640x480.
*   **Condition**: The `vertical_blanking` register on the OV7251 **must** be normalized (e.g., to 1244). If this is not set correctly or fluctuates wildly, the IPU3 CSI-2 receiver loses sync (DPHY error), causing frame drops and a highly stuttering video feed.

### Phase 2: Stream Allocation & Driver Reset
When a consumer (like `v4l2-ctl --stream-mmap` or the Rust ISP via `stdin`) requests the stream to start, the IPU3 kernel driver takes temporary, absolute control of the sensor.
*   **Action**: The IPU3 driver powers up the sensor, negotiates the MIPI CSI-2 physical link, and internally resets the OV7251 registers to their default state to prepare for frame acquisition.
*   **Critical Condition**: Because of this reset, any manual hardware hacks (like turning on the IR LED) applied *before* this phase are **wiped out**.

### Phase 3: Hardware Synchronization (The 2.5s Delay)
To circumvent the driver reset, our `scanner` daemon employs a delayed, one-shot injection strategy.
*   **Action**: The capture script (`run_capture.sh`) launches `v4l2-ctl` (which triggers Phase 2) and the `scanner` daemon simultaneously in the background.
*   **Action**: The `scanner` immediately halts and waits for exactly 2.5 seconds. This provides the IPU3 driver enough time to finish its reset routine and stabilize the stream.
*   **Action**: After 2.5 seconds, the `scanner` writes the `0x3bxx` hardware registers (Strobe PWM Sync, Pulse Width) directly via I2C (`i2ctransfer`).
*   **Condition**: The injection must execute **exactly once**. If the daemon attempts to re-inject or continuously poll the I2C bus while the stream is active, the bus contention causes the IPU3's MIPI receiver to lose sync (`DPHY synchronization error`), freezing the stream for ~1 second intervals.

### Phase 4: Data Ingestion & Unpacking (ISP)
Once the stream is flowing and illuminated, the Rust ISP processes the data.
*   **Action**: Raw binary blobs (Intel IPU3 10-bit packed format) are piped into the ISP via `stdin`. Each 640x480 frame arrives as exactly 399,360 bytes.
*   **Action**: The ISP unpacks every 5 bytes into 4 pixels of 10-bit data, immediately downshifting them to 8-bit grayscale for speed.
*   **Action**: The 8-bit raw buffer is rotated 90 degrees clockwise to correct the sensor's physical 90-degree mounting orientation on the Surface Pro 5.
*   **Action**: The 8-bit grayscale is expanded to 32-bit BGRA (Blue, Green, Red, Alpha) space.

### Phase 5: Output & Consumption
*   **Action**: The final BGRA buffer is written to `/dev/video20` (the `v4l2loopback` sink).
*   **Condition**: For video applications (`ffplay`, `fswebcam`, Howdy) to read the stream without artifacts, they must be explicitly instructed to acquire the stream in `bgr0` (or compatible 32-bit format) at exactly 640x480.

---

## Summary of Critical Conditions for Stability

If diagnosing future issues, verify these three pillars of stability:

1. **V-Blank Normalization**: If the video is stuttering rapidly right out of the gate, the V-Blank timing is wrong. Fix it via `v4l2-ctl --set-ctrl=vertical_blanking=...`.
2. **I2C Silence During Streaming**: If the video stutters periodically (e.g., every 2 seconds) and the kernel log (`dmesg`) shows `DPHY` errors, a secondary process is writing to the sensor's I2C bus concurrently. **No process other than the IPU3 driver should touch the I2C bus while streaming.**
3. **Post-Initialization Injection**: If the IR LED stays dark, the injection happened too early and was overwritten by the IPU3 sequence. Ensure the minimum 2-second delay.
