#!/bin/bash
MEDIA_DEV="/dev/media2"

echo "Attempting to link OV7251 to IPU3 CSI2 with various syntaxes..."

# Option 1: Standard name-based with flags
echo "Trial 1: Standard with [1]"
sudo media-ctl -d $MEDIA_DEV -l '"ov7251 3-0060":0 -> "ipu3-csi2 0":0[1]'

# Option 2: Name-based without flags (since it might already be enabled or flags handled differently)
echo "Trial 2: No flags"
sudo media-ctl -d $MEDIA_DEV -l '"ov7251 3-0060":0 -> "ipu3-csi2 0":0'

# Option 3: Using entity IDs (39 for sensor, 1 for CSI2 0)
echo "Trial 3: Entity IDs"
sudo media-ctl -d $MEDIA_DEV -l '39:0 -> 1:0[1]'

# Option 4: Trialing different sub-interface (CSI2 1) just in case
echo "Trial 4: CSI2 1 (Entity 10)"
sudo media-ctl -d $MEDIA_DEV -l '39:0 -> 10:0[1]'

# Option 5: No spaces in link string
echo "Trial 5: No spaces"
sudo media-ctl -d $MEDIA_DEV -l '"ov7251 3-0060":0->"ipu3-csi2 0":0[1]'

echo "Done. Please check which one (if any) succeeded by running: media-ctl -d $MEDIA_DEV -p"
