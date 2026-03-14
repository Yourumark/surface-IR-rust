#!/bin/bash
# IPU3 Hardware Linkage Prober
MDEV="/dev/media0"
VDEV="/dev/video0"

# Target entities from topology
SENSOR="'ov7251 3-0060'"
CSI_0="'ipu3-csi2 0'"
CSI_1="'ipu3-csi2 1'"
CSI_2="'ipu3-csi2 2'"
CSI_3="'ipu3-csi2 3'"

echo "Attempting exhaustive linkage probe..."

# Try different receivers
for CSI in "$CSI_0" "$CSI_1" "$CSI_2" "$CSI_3"; do
    echo "Probing link $SENSOR -> $CSI..."
    sudo media-ctl -d $MDEV -l "$SENSOR:0 -> $CSI:0[1]" 2>/tmp/link_err.txt
    if [ $? -eq 0 ]; then
        echo "SUCCESS! Linked to $CSI"
        break
    else
        cat /tmp/link_err.txt
    fi
done

echo "Setting formats on linked path..."
sudo media-ctl -d $MDEV --set-v4l2 "$SENSOR:0 [fmt:SBGGR10_1X10/640x480]"
sudo media-ctl -d $MDEV --set-v4l2 "$CSI_0:0 [fmt:SBGGR10_1X10/640x480]"
sudo media-ctl -d $MDEV --set-v4l2 "$CSI_0:1 [fmt:SBGGR10_1X10/640x480]"

v4l2-ctl -d $VDEV --set-fmt-video=width=640,height=480,pixelformat=ip3G

echo "Final Topology Verification:"
media-ctl -d $MDEV -p
