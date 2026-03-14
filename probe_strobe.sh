#!/bin/bash
# probe_strobe.sh: Read and write strobe registers during stream

I2C_BUS=3
ADDR=0x60

read_reg() {
    sudo i2ctransfer -f -y $I2C_BUS r1@$ADDR $(printf "0x%02x 0x%02x" $(( ($1 >> 8) & 0xff )) $(( $1 & 0xff )))
}

echo "--- Current State ---"
echo "0x3026 (Strobe Ctrl): $(read_reg 0x3026)"
echo "0x3027 (PWM Ctrl):    $(read_reg 0x3027)"
echo "0x3b00 (Strobe En):   $(read_reg 0x3b00)"
echo "0x3b02/03 (Width?):   $(read_reg 0x3b02) $(read_reg 0x3b03)"
echo "0x3b82/83 (PWM Duty?):$(read_reg 0x3b82) $(read_reg 0x3b83)"

echo "--- Testing Width Change (Exposure = 1000) ---"
echo "Setting 0x3b03 = 0xff (Max)"
sudo i2ctransfer -f -y $I2C_BUS w3@$ADDR 0x3b 0x03 0xff
sleep 2
echo "Setting 0x3b03 = 0x01 (Min)"
sudo i2ctransfer -f -y $I2C_BUS w3@$ADDR 0x3b 0x03 0x01
sleep 2

echo "--- Testing PWM Routing ---"
echo "Setting 0x3027 = 0x03 (PWM En + Out)"
sudo i2ctransfer -f -y $I2C_BUS w3@$ADDR 0x30 0x27 0x03
sleep 2
echo "Setting 0x3b83 = 0xff (Max PWM)"
sudo i2ctransfer -f -y $I2C_BUS w3@$ADDR 0x3b 0x83 0xff
sleep 2
echo "Setting 0x3b83 = 0x01 (Min PWM)"
sudo i2ctransfer -f -y $I2C_BUS w3@$ADDR 0x3b 0x83 0x01
