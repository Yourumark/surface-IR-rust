#!/bin/bash
# IR LED Control Helper for Surface Pro 5
# Usage: ./led_control.sh [on|off] <brightness_level: 0-255>

# Discover ov7251 subdev node
SENSOR_NODE=$(for i in /sys/class/video4linux/v4l-subdev*/name; do if grep -q "ov7251" "$i"; then echo "/dev/$(basename $(dirname "$i"))"; fi; done | head -n 1)

if [ -z "$SENSOR_NODE" ]; then
    echo "ERROR: ov7251 subdevice not found."
    exit 1
fi

MODE=$1
BRIGHTNESS=${2:-128}

if [ "$MODE" == "on" ]; then
    # Adjust sensor exposure/gain for low-light visibility
    sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=exposure=1000
    sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=analogue_gain=128 || sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=gain=128
    echo "IR Sensor tuned for ON state."
elif [ "$MODE" == "off" ]; then
    # Reset to low power
    sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=exposure=1
    sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=analogue_gain=16 || sudo v4l2-ctl -d $SENSOR_NODE --set-ctrl=gain=16
    echo "IR Sensor reset to OFF state."
else
    echo "Usage: $0 [on|off]"
    exit 1
fi
