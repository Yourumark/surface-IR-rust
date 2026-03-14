#!/bin/bash
# Real Hardware Setup for OV7251 on Surface Pro 5
# This script detects IDs dynamically to avoid errors.

MDEV="/dev/media0"

# Find Entities
OV_ID=$(media-ctl -d $MDEV -p | grep "entity" | grep "ov7251" | awk '{print $3}' | tr -d ':')
CSI_ID=$(media-ctl -d $MDEV -p | grep "entity" | grep "ipu3-csi2 0" | awk '{print $3}' | tr -d ':')


echo "Linking OV7251 ($OV_ID) to CSI2 ($CSI_ID) on $MDEV..."
media-ctl -d $MDEV -l "$OV_ID:0 -> $CSI_ID:0[1]"

echo "Setting formats to 640x480 SBGGR10_1X10..."
# Identify subdevs dynamically if possible, or use the labels directly with quotes
media-ctl -d $MDEV --set-v4l2 "'ov7251 3-0060':0 [fmt:SBGGR10_1X10/640x480]"
media-ctl -d $MDEV --set-v4l2 "'ipu3-csi2 0':0 [fmt:SBGGR10_1X10/640x480]"
media-ctl -d $MDEV --set-v4l2 "'ipu3-csi2 0':1 [fmt:SBGGR10_1X10/640x480]"


echo "Setup complete. Start the ISP with: ./target/debug/surface-IR-rust"
