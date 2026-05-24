1904

Touch sensing controller (TSC)

RM0456
Table 456. Acquisition sequence summary

State

Gx_IO1
(channel)

Gx_IO2
(sampling)

#1

Input floating
with analog
switch closed

Output opendrain low with
analog switch
closed

#2
#3

Note:

Gx_IO4
(channel)

Output pushpull high

State description

Input floating with analog switch Discharge all CX and
CS
closed

Input floating

#4
#5

Gx_IO3
(channel)

Dead time

Input floating

Charge CX1

Input floating
Input floating with analog switch
closed

Dead time
Charge transfer from
CX1 to CS

Input floating

#6

Input floating

Dead time

#7

Input floating

Measure CS voltage

Gx_IOy where x is the analog I/O group number and y the GPIO number within the selected
group.
The voltage variation over the time on the sampling capacitor CS is detailed below (refer to
Figure 452 for VSENSOR and VCS definition):
Figure 453. Sampling capacitor voltage variation

VSENSOR
VCS
VDD

VIH

S1

S2

t
MSv71405V1

<!-- pagebreak -->

