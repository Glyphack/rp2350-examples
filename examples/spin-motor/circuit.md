## Program


The motor driver expects `EN` pin to be low to turn on.
`STEP` pin triggers on low to high.
`STEP` must remain low/high for 100ns for the driver to see it.

Step configuration:
MS1/MS2: CONFIGURATION OF MICROSTEP RESOLUTION FOR STEP INPUT
GND GND 8 microsteps 0
GND VCC_IO 32 microsteps (different to TMC2208!) 1
VCC_IO GND 64 microsteps (different to TMC2208!) 2
VCC_IO VCC_IO 16 microsteps 3

rp -> tmc connections:
gpio26 (D0) -> `STEP`
gpio2 (D8) -> `EN`. This controls the power.
gpio3 (D10) -> `DIR`. Controls direction.

## Circuit

Parts:
- USB PD power supply, provides motor power
- Wire terminal block for the supply leads
- Breadboard
- Seeed XIAO RP2350
- TMC2209 stepper driver
- Capacitor, decoupling across motor power
- Stepper motor with two coils, A and B

Breadboard notation:
- A hole is written as column then row, for example c1 or g30.
- Columns a to e are the left group, f to j are the right group.
- Holes in the same row and same group are connected.
- The two side rails are the negative rail for ground and the positive rail for motor V+.

RP2350 pin placement:
- GND at c29
- 3V3 at c28
- D0, GPIO26, STEP at g30
- D8, GPIO2, EN at c25
- D10, GPIO3, DIR at c27

TMC2209 pin placement:
- EN at c1
- MS1 at c2
- MS2 at c3
- STEP at c7
- DIR at c8
- VM at g1
- GND at g2
- VIO at g7
- Motor coil A1 at j3
- Motor coil A2 at j4
- Motor coil B1 at j5
- Motor coil B2 at j6

Capacitor:
- Positive leg at h1, shares the VM row
- Negative leg at h2, shares the GND row

Supply:
- USB PD positive to positive rail, near row 29
- USB PD negative to negative rail, near row 30

Jumper wires:
- Positive rail to j1, motor V+ to TMC VM
- Negative rail to j2, ground to TMC GND
- Negative rail to b29, ground to RP GND row
- c29 to i2, RP GND to TMC GND
- c28 to h7, RP 3V3 to TMC VIO
- g30 to c7, RP D0 to TMC STEP
- c27 to c8, RP D10 to TMC DIR
- c25 to c1, RP D8 to TMC EN
- c2 to negative rail, TMC MS1 to ground
- c3 to negative rail, TMC MS2 to ground

Microstepping:
- MS1 and MS2 both to ground selects 8 microsteps, see the table above.
