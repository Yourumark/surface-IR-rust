#!/bin/bash
MEDIA_DEV="/dev/media2"

# Get dynamic entity IDs to avoid kernel panic/instability from static IDs
OV_ID=$(media-ctl -d $MEDIA_DEV -e "ov7251 3-0060" 2>/dev/null || media-ctl -d $MEDIA_DEV -p | grep "ov7251" | awk -F: '{print $1}' | awk '{print $NF}')
CSI_ID=$(media-ctl -d $MEDIA_DEV -e "ipu3-csi2 0" 2>/dev/null || media-ctl -d $MEDIA_DEV -p | grep "ipu3-csi2 0" | awk -F: '{print $1}' | awk '{print $NF}')

if [ -z "$OV_ID" ] || [ -z "$CSI_ID" ]; then
    echo "Error: Could not find OV7251 or CSI2 entities."
    exit 1
fi

echo "Found OV7251: Entity $OV_ID"
echo "Found CSI2 0: Entity $CSI_ID"

# Link and setup
echo "Linking..."
sudo media-ctl -d $MEDIA_DEV -l "${OV_ID}:0 -> ${CSI_ID}:0[1]"

echo "Setting format..."
sudo media-ctl -d $MEDIA_DEV --set-v4l2 "${OV_ID}:0[fmt:SBGGR10_1X10/640x480]"

echo "Done. Verify with: media-ctl -d $MEDIA_DEV -p | grep -A 5 \"entity $OV_ID\""
