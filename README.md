## Specs

Micro controller:
https://mischianti.org/seeed-studio-xiao-rp2350-pinout-datasheet-schema-and-specifications/
https://pip-assets.raspberrypi.com/categories/1214-rp2350/documents/RP-008373-DS-2-rp2350-datasheet.pdf

Motor driver:
https://global.bttwiki.com/TMC2209.html
datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/tmc2209_datasheet_rev1.09.pdf


## Dev

The motor driver expects `EN` pin to be low to turn on.
`STEP` pin triggers on low to high.
`STEP` must remain low/high for 100ns for the driver to see it.


MS1/MS2: CONFIGURATION OF MICROSTEP RESOLUTION FOR STEP INPUT
MS2 MS1 Microstep Setting UART Address
GND GND 8 microsteps 0
GND VCC_IO 32 microsteps (different to TMC2208!) 1
VCC_IO GND 64 microsteps (different to TMC2208!) 2
VCC_IO VCC_IO 16 microsteps 3


## Basic Motor spinning

When the power is one it spins the motor infinitely.

rp -> tmc connections:
gpio26 (D0) -> `STEP`
gpio2 (D8) -> `EN`. This controls the power.
gpio3 (D10) -> `DIR`. Controls direction.


## TODO

- Spin the motor
- Sensor less homing
