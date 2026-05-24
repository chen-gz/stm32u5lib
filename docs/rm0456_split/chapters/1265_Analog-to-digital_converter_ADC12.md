RM0456 Rev 6

0

0

0

0

0

Res.

Res.

UNIT[6:0]

Res.

LNG[11:0]

Res.

Res.

0

Res.

Reset value

Res.

DLYB_CFGR

Res.

0x004

LNGF

SEN

Reset value

DEN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DLYB_CR

Res.

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

0x000

Res.

Offset

Res.

Table 301. DLYB register map and reset values
Register
name

0

0

SEL[3:0]
0

0

0

0

RM0456

33

Analog-to-digital converter (ADC12)

Analog-to-digital converter (ADC12)
STM32U535/545/575/585 devices embed one analog-to-digital converter (ADC1) while
STM32U59x/5Ax/5Fx/5Gx devices embed two analog-to-digital converters (ADC1 and
ADC2), controlled by a single interface ADC12.

33.1

Introduction
This section describes the implementation of the 14-bit ADC successive approximation
analog-to-digital converter.
ADC1 and ADC2 are tightly coupled and can operate in dual mode (ADC1 is a master).
The ADC features up to 20 multiplexed channels. Channel A/D conversion can be
performed in single, continuous, scan, or discontinuous mode. The result of the ADC can be
stored in a left-aligned or right-aligned 32-bit data register.
The ADC is mapped on the AHB bus to enable fast data handling.
In addition, the analog watchdog features enable the application to detect if the input voltage
goes outside user-defined high or low thresholds.
The ADC features a built-in hardware oversampler that improves analog performances
while off-loading the related computational burden from the CPU.
An efficient low-power mode is also implemented to achieve very low consumption at low
frequency.

33.2

ADC main features
•

High-performance features
–

Dual mode operation (refer to Section 33.3: ADC implementation)

–

14-, 12-, 10-, or 8-bit configurable resolution

–

ADC conversion time independent from the AHB bus clock frequency

–

Faster conversion time by lowering resolution

–

Management of single-ended or differential inputs (programmable per channels)

–

AHB slave bus interface for fast data handling

–

Self-calibration (both offset and linearity)

–

Channel-wise programmable sampling time

–

Flexible sampling time control

–

Up to four injected channels (fully configurable analog input assignment to regular
or injected channels)

–

Hardware assistant to prepare the injected channel context and enable fast
context switching

–

Data alignment with in-built data coherency

–

Data management by general-purpose DMA for regular channel conversions with
FIFO

–

Data routing to MDF for post processing

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)
–
•

•

•

RM0456

Four dedicated data registers for injected channels

Oversampler
–

32-bit data register

–

Oversampling ratio adjustable from 2 to 1024

–

Programmable data right and left shift

Data preconditioning
–

Gain compensation

–

Offset compensation

Low-power features
–

Speed adaptive low-power mode to reduce ADC consumption when operating at
low frequency

–

Support of slow bus frequency applications while keeping optimum ADC
performance

–

Automatic control to avoid ADC overrun in AHB bus clock low-frequency
application (auto-delayed mode)

•

Up to 17 external analog input channels connected to dedicated GPIO pads

•

3 internal dedicated channels
–

•

•

One channel for internal reference voltage (VREFINT)

–

One channel for internal temperature sensor (VSENSE)

–

One channel for VBAT monitoring channel (VBAT/4)

Start-of-conversion can be initiated:
–

by software for both regular and injected conversions or

–

by hardware triggers with configurable polarity (internal timers events or GPIO
input events) for both regular and injected conversions

Conversion modes
–

Single mode: the ADC converts a single channel. The conversion is triggered by a
special event.

–

Scan mode: the ADC scans and converts a sequence of channels.

–

Continuous mode: the ADC converts continuously selected inputs.

–

Discontinuous mode: the ADC converts a subset of the conversion sequence.

•

Interrupt generation when the ADC is ready, at end of sampling, end of conversion
(regular or injected), end of sequence conversion (regular or injected), analog
watchdog 1, 2 or 3 or when an overrun event occurs

•

Three analog watchdogs
The watchdogs can perform filtering to ignore out-of-range data.

•

ADC input range: VREF– ≤ VIN ≤ VREF+

Figure 221 shows the block diagram of one ADC.

<!-- pagebreak -->

