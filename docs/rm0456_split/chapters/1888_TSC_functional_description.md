1904

Touch sensing controller (TSC)

RM0456

47.3

TSC functional description

47.3.1

TSC block diagram
The block diagram of the touch sensing controller is shown in Figure 451.
Figure 451. TSC block diagram

SYNC
Pulse generator
fHCLK

G1_IO1

Clock
prescalers

G1_IO2
G1_IO3

Spread spectrum

G1_IO4
G2_IO1
G2_IO2
Group counters

I/O control
logic

G2_IO3
G2_IO4

TSC_IOG1CR
TSC_IOG2CR
Gx_IO1
Gx_IO2
TSC_IOGxCR

Gx_IO3
Gx_IO4

MS30929V1

47.3.2

Surface charge transfer acquisition overview
The surface charge transfer acquisition is a proven, robust and efficient way to measure a
capacitance. It uses a minimum number of external components to operate with a single
ended electrode type. This acquisition is designed around an analog I/O group composed of
up to four GPIOs (see Figure 452). Several analog I/O groups are available to allow the
acquisition of several capacitive sensing channels simultaneously and to support a larger
number of capacitive sensing channels. Within a same analog I/O group, the acquisition of
the capacitive sensing channels is sequential.
One of the GPIOs is dedicated to the sampling capacitor CS. Only one sampling capacitor
I/O per analog I/O group must be enabled at a time.
The remaining GPIOs are dedicated to the electrodes and are commonly called channels.
For some specific needs (such as proximity detection), it is possible to simultaneously
enable more than one channel per analog I/O group.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Touch sensing controller (TSC)
Figure 452. Surface charge transfer analog I/O group structure
Analog
I/O group

Electrode 1

VSENSOR

RS1

Gx_IO1

CX1

Gx_IO2
CS

VCS

Electrode 2

VSENSOR

RS2

CX2

Electrode 3

VSENSOR

Gx_IO3

RS3

Gx_IO4

CX3

MSv30930V3

Note:

Gx_IOy where x is the analog I/O group number and y the GPIO number within the selected
group.
The surface charge transfer acquisition principle consists of charging an electrode
capacitance (CX) and transferring a part of the accumulated charge into a sampling
capacitor (CS). This sequence is repeated until the voltage across CS reaches a given
threshold (VIH in our case). The number of charge transfers required to reach the threshold
is a direct representation of the size of the electrode capacitance.
Table 456 details the charge transfer acquisition sequence of the capacitive sensing
channel 1. States 3 to 7 are repeated until the voltage across CS reaches the given
threshold. The same sequence applies to the acquisition of the other channels. The
electrode serial resistor RS improves the ESD immunity of the solution.

RM0456 Rev 6

<!-- pagebreak -->

