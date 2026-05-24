1599

Multi-function digital filter (MDF)

39.2

RM0456

MDF main features
•

AHB interface

•

up to 6 serial digital inputs:
–

configurable SPI interface to connect various digital sensors

–

configurable Manchester coded interface support

–

compatible with PDM interface to support digital microphones

•

2 common clocks input/output for Σ∆ modulators

•

Flexible matrix (BSMX) for connection between filters and digital inputs

•

2 inputs to connect the internal ADCs

•

up to 6 flexible digital filter paths, including:
–

A configurable CIC filter:
- Can be split into 2 CIC filters: high-resolution filter and out-off limit detector
- Can be configured in Sinc4 filter
- Can be configured in Sinc5 filter
- Adjustable decimation ratio

39.3

–

A reshape filter to improve the out-off band rejection and in-band ripple

–

A high-pass filter to cancel the DC offset

–

An offset error cancellation

–

Gain control

–

Saturation blocks

–

An out-off limit detector

•

Short-circuit detector

•

Clock absence detector

•

16- or 24-bit signed output data resolution

•

Continuous or single conversion

•

Possibility to delay independently each bitstream

•

Various trigger possibilities

•

Break generation on out-of limit or short-circuit detector events

•

Autonomous functionality in Stop mode(s)

•

DMA can be used to read the conversion data

•

Interrupts services

MDF implementation
The devices embed one MDF instance and one ADF instance, both being digital filters with
common features.
Table 368. ADF/MDF features(1)
Mode or feature

ADF1
MDF1
all devices STM32U535/545

Number of filters (DFLTx) and serial
interfaces (SITFx)

1

<!-- pagebreak -->

RM0456 Rev 6

2

MDF1
STM32U59x/5Ax
5Fx/5Gx

MDF1
STM32U575/585
6

RM0456

Multi-function digital filter (MDF)
Table 368. ADF/MDF features(1) (continued)
Mode or feature

ADF1
MDF1
all devices STM32U535/545

MDF1
STM32U59x/5Ax
5Fx/5Gx

MDF1
STM32U575/585

MDF_CKIy/ADF_CKI0 connected to pins

-

Sound activity detection (SAD)

X

-

X

RXFIFO depth (number of 24-bit words)

4

4

ADC connected to ADCITF1

-

ADC1

ADC connected to ADCITF2

-

Motor dedicated features (SCD, OLD,
OEC, INT, snapshot, break)

-

Main path with CIC4, CIC5

X

Main path with CIC1,2, 3 or FastSinc

-

RSFLT, HPF, SAT, SCALE, DLY, Discard
functions

X

Autonomous in Stop modes

X(2)

-

-

ADC2

X

X(3)

1. ‘X’ = supported, ‘-’ = not supported.
2. Only Stop 0, Stop 1, and Stop 2 modes.
3. Only Stop 0 and Stop 1 modes.

RM0456 Rev 6

<!-- pagebreak -->

