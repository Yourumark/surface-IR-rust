#!/bin/bash
MDEV="/dev/media0"
VDEV="/dev/video0"
OV_ID=39
CSI_ID=1

echo "--- Step 1: Linkage ---"
# Try alternative syntaxes for the link
media-ctl -d $MDEV -l "39:0->1:0[1]" || \
media-ctl -d $MDEV -l "'ov7251 3-0060':0->'ipu3-csi2 0':0[1]" || \
echo "FAILED to link sensor to CSI2"

echo "--- Step 2: Subdev Formats ---"
media-ctl -d $MDEV --set-v4l2 "39:0 [fmt:SBGGR10_1X10/640x480]"
media-ctl -d $MDEV --set-v4l2 "1:0 [fmt:SBGGR10_1X10/640x480]"
media-ctl -d $MDEV --set-v4l2 "1:1 [fmt:SBGGR10_1X10/640x480]"

echo "--- Step 3: Capture Node Format ---"
# IPU3 capture nodes often need v4l2-ctl to agree on the resolution
v4l2-ctl -d $VDEV --set-fmt-video=width=640,height=480,pixelformat=ip3G

echo "--- Step 4: Verification ---"
media-ctl -d $MDEV -p | grep -E "entity (39|1|4):" -A 4
v4l2-ctl -d $VDEV --get-fmt-video

