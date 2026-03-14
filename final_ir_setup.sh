#!/bin/bash
# Dynamic IR Camera Setup for Surface Pro 5
# This script finds the ov7251 sensor, links it to an IPU3 CSI2 port,
# and configures the resulting video node.

echo "=== Dynamic IR Setup Starting ==="

# 0. Ensure all drivers are loaded
echo "0. Loading camera drivers..."
sudo modprobe ov5693 2>/dev/null
sudo modprobe ov8865 2>/dev/null
sudo modprobe ov7251 2>/dev/null
sudo modprobe ipu3-cio2 2>/dev/null
sleep 1

SENSOR_NAME="ov7251"
FMT="Y10_1X10/640x480"
IP3Y_FMT="ip3y"

# 1. Discover Topology
echo "1. Discovering Topology..."
FOUND_VDEV=""
for mdev in /dev/media*; do
    if ! media-ctl -d $mdev -p > /tmp/topo.txt 2>/dev/null; then continue; fi
    
    if grep -q "$SENSOR_NAME" /tmp/topo.txt; then
        echo "   Found $SENSOR_NAME on $mdev"
        
        # Get Sensor Entity ID
        SENSOR_ID=$(grep -E "entity [0-9]+: $SENSOR_NAME" /tmp/topo.txt | sed -nE 's/.*entity ([0-9]+): *.*/\1/p' | head -n 1)
        
        # Try finding an available CSI-2 port (usually it's 2 on Pro 5)
        for port in 2 0 1 3; do
            CSI_ENTITY="ipu3-csi2 $port"
            CSI_ID=$(grep -E "entity [0-9]+: $CSI_ENTITY" /tmp/topo.txt | sed -nE 's/.*entity ([0-9]+): *.*/\1/p' | head -n 1)
            
            if [ -n "$CSI_ID" ]; then
                echo "   Attempting link: $SENSOR_NAME ($SENSOR_ID) -> $CSI_ENTITY ($CSI_ID)"
                sudo media-ctl -d $mdev -l "$SENSOR_ID:0 -> $CSI_ID:0[1]" 2>/dev/null
                
                if [ $? -eq 0 ]; then
                    echo "   Link Successful!"
                    # Set formats
                    sudo media-ctl -d $mdev --set-v4l2 "$SENSOR_ID:0 [fmt:$FMT]"
                    sudo media-ctl -d $mdev --set-v4l2 "$CSI_ID:0 [fmt:$FMT]"
                    sudo media-ctl -d $mdev --set-v4l2 "$CSI_ID:1 [fmt:$FMT]"
                    
                    # Find downstream ipu3-cio2 node
                    CIO2_ENTITY="ipu3-cio2 $port"
                    CIO2_ID=$(grep -E "entity [0-9]+: $CIO2_ENTITY" /tmp/topo.txt | sed -nE 's/.*entity ([0-9]+): *.*/\1/p' | head -n 1)
                    FOUND_VDEV=$(grep -A 10 "entity $CIO2_ID:" /tmp/topo.txt | grep "device node name" | head -n 1 | awk '{print $NF}')
                    break 2
                fi
            fi
        done
    fi
done

if [ -z "$FOUND_VDEV" ]; then
    echo "ERROR: Could not establish IR camera link dynamically."
    exit 1
fi

echo "2. Configuring Discovered Video Node: $FOUND_VDEV"
sudo v4l2-ctl -d $FOUND_VDEV --set-fmt-video=width=640,height=480,pixelformat=$IP3Y_FMT

# Save for reference
echo "$FOUND_VDEV" > /tmp/ir_node

echo ""
echo "=== Setup Complete ==="
echo "IR Hardware path: $FOUND_VDEV"
echo ""
echo "To start the high-reliability pipeline, run:"
echo "--------------------------------------------------"
echo "sudo ./scanner/target/release/scanner &"
echo "sudo v4l2-ctl -d $FOUND_VDEV --stream-mmap --stream-to=- | ./target/debug/surface-IR-rust --stdin"
echo "--------------------------------------------------"
echo ""
echo "Visual output will be available on /dev/video20"

