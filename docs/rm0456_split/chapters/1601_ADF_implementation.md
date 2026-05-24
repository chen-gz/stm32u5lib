ADF main features
•

AHB Interface

•

1 serial digital input:
–

configurable SPI interface to connect various digital sensors

–

configurable Manchester coded interface support

–

compatible with PDM interface to support digital microphones

•

2 common clocks input/output for Σ∆ modulators

•

1 flexible digital filter path including:
–

A MCIC filter configurable in Sinc4 or Sinc5 filter with an adjustable decimation
ratio

–

A reshape filter to improve the out-off band rejection and in-band ripple

RM0456 Rev 6

RM0456

40.3

Audio digital filter (ADF)
–

A high-pass filter to cancel the DC offset

–

Gain control

–

Saturation blocks

•

Clock absence detector

•

Sound activity detector

•

24-bit signed output data resolution

•

Continuous or single conversion

•

Possibility to delay the selected bitstream

•

One trigger input

•

Autonomous functionality in Stop mode(s)

•

DMA can be used to read the conversion data

•

Interrupts services

ADF implementation
The devices embed one MDF instance and one ADF instance, both being digital filters with
common features.
Table 389. ADF/MDF features(1)
Mode or feature

Number of filters (DFLTx) and serial
interfaces (SITFx)

ADF1
MDF1
all devices STM32U535/545

MDF1
STM32U59x/5Ax
5Fx/5Gx

MDF1
STM32U575/585

1

2

6

MDF_CKIy/ADF_CKI0 connected to pins

-

-

Sound activity detection (SAD)

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

X
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

