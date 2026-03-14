#!/bin/bash
# Universal Surface IR Camera Linker (Robust ID version)
# This script finds the correct media device and port, then links the sensor using integer IDs.

fmt="SBGGR10_1X10/640x480"
sensor_name="ov7251"

echo "=== Universal IR Linker (Final) ==="

for mdev in /dev/media*; do
    echo "Checking $mdev..."
    if ! media-ctl -d $mdev -p >/tmp/topology.txt 2>&1; then
        echo "  Could not read $mdev"
        continue
    fi
    
    if grep -q "$sensor_name" /tmp/topology.txt; then
        echo "  FOUND $sensor_name on $mdev!"
        
        # Get exact sensor entity ID
        sensor_id=$(grep -E "entity [0-9]+: $sensor_name" /tmp/topology.txt | sed -nE 's/.*entity ([0-9]+): *.*/\1/p' | head -n 1)
        
        if [ -z "$sensor_id" ]; then
             echo "  Warning: Could not extract sensor entity ID."
             continue
        fi
        
        echo "  Sensor ID: $sensor_id"
        
        # Try ports 0-3
        for i in 0 1 2 3; do
            csi_entity="ipu3-csi2 $i"
            csi_id=$(grep -E "entity [0-9]+: $csi_entity" /tmp/topology.txt | sed -nE 's/.*entity ([0-9]+): *.*/\1/p' | head -n 1)
            
            if [ -n "$csi_id" ]; then
                echo "  Trying link: $sensor_id:0 -> $csi_id:0..."
                sudo media-ctl -d $mdev -l "$sensor_id:0 -> $csi_id:0[1]"
                if [ $? -eq 0 ]; then
                    echo "  SUCCESS! Linked to $csi_entity (ID $csi_id)"
                    
                    echo "  Setting format on sensor ($sensor_id)..."
                    sudo media-ctl -d $mdev --set-v4l2 "$sensor_id:0 [fmt:$fmt]"
                    
                    echo "  Setting format on CSI receiver ($csi_id)..."
                    sudo media-ctl -d $mdev --set-v4l2 "$csi_id:0 [fmt:$fmt]"
                    sudo media-ctl -d $mdev --set-v4l2 "$csi_id:1 [fmt:$fmt]"
                    
                    # Find which video node corresponds to this CSI port
                    vnode=$(grep -A 10 "entity $csi_id:" /tmp/topology.txt | grep "device node name" | awk '{print $NF}')
                    echo "  Capture Node Detected: $vnode"
                    
                    echo "  Setting video node format..."
                    sudo v4l2-ctl -d $vnode --set-fmt-video=width=640,height=480,pixelformat=ip3y
                    
                    echo ""
                    echo "--- PROBE SUCCESSFUL ---"
                    echo "Use this node in your Rust code: $vnode"
                    exit 0
                fi
            fi
        done
    fi
done

echo "FAILED to establish link. Try 'sudo modprobe ipu3-cio2' first?"
exit 1
