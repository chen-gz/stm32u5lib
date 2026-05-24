TIMx_ARR

Reset value
Res.

Res.

Res.

Res.

TIMx_PSC

Res.

0

Res.

Reset value

Res.

TIMx_CNT

Res.

0x28

0
0
0

RM0456 Rev 6
Res.

Res.

Res.
Res.

Res.

Res.

0x180x20

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

Reset value

0
0

0

1
0

0

1
0

0

1

Refer to Section 2.3 for the register boundary addresses.
0

0

1
0

0

1
0

0

1
0

0

1
0

0

1
0
0

Reset value

Reset value
UIF

Res.

Res.

0

UG

Res.

Res.
Res.
Res.

Res.
Res.
UIE

URS
UDIS
CEN

OPM

Res.

Res.

Res.

ARPE

Res.

Res.

Res.

UIFREMA

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

0

Res.

Res.

Res.

0

Res.

0

Res.

Reserved
0

Res.

Res.
MMS
[2:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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
Res.

Res.

DITHEN

0

Res.

Res.

Res.

Res.

UDE

Res.

Reset value

Res.

Res.

Res.

Res.

Reset value

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

TIMx_CR1

0

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

Register
name

0

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

Reset value

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

0x08

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

TIMx_EGR

Res.

0x24
TIMx_SR

Res.

0x14
TIMx_DIER

Res.

0x10
TIMx_CR2

Res.

0x0C

UIFCPY or Res.

0x04

Res.

0x00

Res.

Offset

Res.

57.4.9

Res.

Basic timers (TIM6/TIM7)
RM0456

TIMx register map
TIMx registers are mapped as 16-bit addressable registers as described in the table below:
Table 598. TIMx register map and reset values

0
0
0

0

0

0

Reserved

CNT[15:0]

0
0
0
0
0
0

PSC[15:0]

ARR[19:0]
0
0
0
0
0
0
0
0

1
1
1
1
1
1
1
1

RM0456

Low-power timer (LPTIM)

58

Low-power timer (LPTIM)

58.1

Introduction
The LPTIM is a 16-bit timer that benefits from the ultimate developments in power
consumption reduction. Thanks to its diversity of clock sources, the LPTIM is able to keep
running in all power modes except for Standby mode. Given its capability to run even with
no internal clock source, the LPTIM can be used as a “Pulse Counter” which can be useful
in some applications. The LPTIM capability to wake up the system from low-power modes,
makes it suitable to realize “Timeout functions” with extremely low power consumption.
The LPTIM introduces a flexible clock scheme that provides the needed functionalities and
performance, while minimizing the power consumption.

58.2

LPTIM main features
•

16-bit upcounter

•

3-bit prescaler with 8 possible dividing factors (1,2,4,8,16,32,64,128)

•

Selectable clock
–

Internal clock sources: configurable internal clock source (see RCC section)

–

External clock source over LPTIM input (working with no LP oscillator running,
used by pulse counter application)

•

16-bit ARR autoreload register

•

16-bit capture/compare register

•

Continuous/One-shot mode

•

Selectable software/hardware input trigger

•

Programmable digital glitch filter

•

Configurable output: Pulse, PWM

•

Configurable I/O polarity

•

Encoder mode

•

Repetition counter

•

Up to 2 independent channels for:
–

Input capture

–

PWM generation (edge-aligned mode)

–

One-pulse mode output

•

Interrupt generation on 10 events

•

DMA request generation on the following events:
–

Update event

–

Input capture

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.3

RM0456

LPTIM implementation
The table below describes LPTIM implementation on STM32U5 series devices. The full set
of features is implemented in LPTIM1, LPTIM2 and LPTIM3. LPTIM4 supports a smaller set
of features.
Table 599. STM32U5 series LPTIM features
LPTIM modes/features(1)

LPTIM1

LPTIM2

LPTIM3

LPTIM4

Encoder mode

X

X

-

-

PWM mode

X

X

X

X

Input Capture

X

X

X

-

Number of channels

2

2

2

-

Number of DMA requests

3

3

3

-

Wake-up from Stop mode

X

(2)

X(3)

X(2)

X(2)

Autonomous mode

X

X

X

-

1. X = supported.
2. Wake-up supported from Stop 0, Stop 1, and Stop 2 modes.
3. Wake-up supported from Stop 0 and Stop 1 modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

58.4

LPTIM functional description

58.4.1

LPTIM block diagram
Figure 736. LPTIM1/2/3 block diagram(1)
LPTIM
Kernel clock domain

Edge
detector

lptim_in2_mux1
lptim_in2_mux2
lptim_in2_mux3

Glitch
filter

Up/down

LPTIM_IN2
lptim_in1_mux1
lptim_in1_mux2
lptim_in1_mux3

Encoder

LPTIM_IN1
lptim_ext_trigx

Glitch
filter
LPTIM
register
interface

lptim_pclk

lptim_it

Edge
detector

Synchronzation

32-bit APB bus

APB clock
domain

Glitch
filter

CNTSTRT/
SNGSTRT

LPTIM_RCR
16-bit ARR
Repetition
counter

Mux trigger

IRQ
interface

LPTIM_ETR

1

1

0

16-bit counter

Count mode
1

lptim_ker_ck

clkmux

Prescaler clkpresc

0

lptim_wakeup
lpti1

Glitch
filter & ic1 Preedge
scaler
detector

lpti2

Glitch
filter & ic2 Preedge
scaler
detector

LPTIM_CH1
lptim_ic1_mux1
lptim_ic1_mux2
lptim_ic1_mux3
LPTIM_CH2
lptim_ic2_mux1
lptim_ic2_mux2
lptim_ic2_mux3

ic1ps

oc1
oc1ref
Capture/compare1
Output
register
control

ic2ps

oc2
oc2ref
Capture/compare2
Output
register
control

lptim_ch1

LPTIM_CH1
lptim_ch2

LPTIM_CH2

lptim_ic1_dma
lptim_ic2_dma
lptim_ue_dma
MSv50909V5

1. Some I/Os may not be available, refer to Section 58.4.2: LPTIM pins and internal signals.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456
Figure 737. LPTIM4 block diagram(1)

LPTIM
Kernel clock domain
Edge
detector
Up/down

Edge
detector
LPTIM
register
interface

APB clock

LPTIM
interrupt

Encoder
Glitch
filter

Glitch
filter

Synchronzation

32-bit APB bus

APB clock
domain

Glitch
filter

CNTSTRT/
SNGSTRT

16-bit ARR

LPTIM_IN2
lptim_in1_mux1
lptim_in1_mux2
lptim_in1_mux3
LPTIM_IN1
lptim_ext_trigx
LPTIM_ETR

LPTIM_RCR

Mux trigger

IRQ
interface

lptim_in2_mux1
lptim_in2_mux2
lptim_in2_mux3

Repetition
counter
1

1

0

16-bit counter

LPTIM_OUT

Count mode
1

lptim_ker_ck

clkmux

lptim_out

Prescaler clkpresc

0

16-bit compare
Wake-up
MSv63063V2

1. Some I/Os may not be available, refer to Section 58.4.2: LPTIM pins and internal signals.

58.4.2

LPTIM pins and internal signals
The following tables provide the list of LPTIM pins and internal signals, respectively.
Table 600. LPTIM1/2/3 input/output pins
Pin name

Pin type

Description

LPTIM_IN1

Digital input

LPTIM input 1 from LPTIMx_IN1 pin on mux input 0

LPTIM_IN2(1)

Digital input

LPTIM input 2 from LPTIMx_IN2 pin on mux input 0

LPTIM_ETR

Digital input

LPTIM external trigger LPTIMx_ETR pin

LPTIM_CH1

Digital
input/output

LPTIM channel 1 input/output LPTIMx_IN1 pin

LPTIM_CH2

Digital
input/output

LPTIM channel 2 input/output LPTIMx_IN2 pin

1. LPTIM3 has only the input 1 (no input 2).

Table 601. LPTIM4 input/output pins
Pin name

<!-- pagebreak -->

Pin type

Description

LPTIM_IN1

Digital input

LPTIM Input 1 from LPTIMx_IN1 pin on mux input 0

LPTIM_ETR

Digital input

LPTIM external trigger LPTIMx_ETR pin

LPTIM_OUT

Digital output

LPTIM output LPTIMx_OUT pin

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)
Table 602. LPTIM1/2/3 internal signals
Signal name

Signal type

Description

lptim_pclk

Digital input

LPTIM APB clock domain

lptim_ker_ck

Digital input

LPTIM kernel clock

lptim_in1_mux1

Digital input

Internal LPTIM input 1 connected to mux input 1

lptim_in1_mux2

Digital input

Internal LPTIM input 1 connected to mux input 2

lptim_in1_mux3

Digital input

Internal LPTIM input 1 connected to mux input 3

(1)

Digital input

Internal LPTIM input 2 connected to mux input 1

lptim_in2_mux2(1)

Digital input

Internal LPTIM input 2 connected to mux input 2

lptim_in2_mux3(1)

Digital input

Internal LPTIM input 2 connected to mux input 3

lptim_ic1_mux1

Digital input

Internal LPTIM input capture 1 connected to mux input 1

lptim_ic1_mux2

Digital input

Internal LPTIM input capture 1 connected to mux input 2

lptim_ic1_mux3

Digital input

Internal LPTIM input capture 1 connected to mux input 3

lptim_ic2_mux1

Digital input

Internal LPTIM input capture 2 connected to mux input 1

lptim_ic2_mux2

Digital input

Internal LPTIM input capture 2 connected to mux input 2

lptim_ic2_mux3

Digital input

Internal LPTIM input capture 2 connected to mux input 3

lptim_ext_trigx

Digital input

LPTIM external trigger input x

lptim_it

Digital output

LPTIM global interrupt

lptim_wakeup

Digital output

LPTIM wake-up event

lptim_ic1_dma

Digital output

LPTIM input capture 1 DMA request

lptim_ic2_dma

Digital output

LPTIM input capture 2 DMA request

lptim_ue_dma

Digital output

LPTIM update event DMA request

lptim_in2_mux1

1. LPTIM3 has only the input 1(no input 2).

Table 603. LPTIM4 internal signals
Signal name

Signal type

Description

lptim_pclk

Digital input

LPTIM APB clock domain

lptim_ker_ck

Digital input

LPTIM kernel clock

lptim_in1_mux1

Digital input

Internal LPTIM input 1 connected to mux input 1

lptim_in1_mux2

Digital input

Internal LPTIM input 1 connected to mux input 2

lptim_in1_mux3

Digital input

Internal LPTIM input 1 connected to mux input 3

lptim_ext_trigx

Digital input

LPTIM external trigger input x

lptim_out

Digital output

LPTIM counter output

lptim_it

Digital output

LPTIM global interrupt

lptim_wakeup

Digital output

LPTIM wake-up event

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.4.3

RM0456

LPTIM input and trigger mapping
The LPTIM external trigger and input connections are detailed hereafter.
Table 604. LPTIM1/2/3/4 external trigger connections
External trigger
TRIGSEL
LPTIM1

LPTIM2

LPTIM3

lptim_ext_trig0

GPIO

lptim_ext_trig1

rtc_alra_trg

lptim_ext_trig2

rtc_alrb_trg

lptim_ext_trig3

tamp_trg1

LPTIM4

lpdma_ch0_tc

lptim_ext_trig4

tamp_trg2

gpdma_ch0_tc

lpdma_ch1_tc

tamp_trg2

lptim_ext_trig5

lpdma_ch2_tc

gpdma_ch4_tc

tamp_trg3

tamp_trg3

lptim_ext_trig6

comp1_out

lptim_ext_trig7

comp2_out(1)

1. This connection is not present in STM32U535/545 as COMP2 is not available.

Table 605. LPTIM1/2/3/4 input 1 connections
lptim_in1_mux

LPTIM1/2/3/4 input 1 connected to

lptim_in1_mux0

GPIO

lptim_in1_mux1

comp1_out

lptim_in1_mux2

Reserved

lptim_in1_mux3

Reserved

Table 606. LPTIM1/2 input 2 connections
lptim_in2_mux

LPTIM1/2 input 2 connected to

lptim_in2_mux0

GPIO

lptim_in2_mux1

comp2_out(1)

lptim_in2_mux2

Reserved

lptim_in2_mux3

Reserved

1. This connection is not present in STM32U535/545 as COMP2 is not available.

Table 607. LPTIM1/2/3 input capture 1 connections
lptim_ic1_mux

<!-- pagebreak -->

LPTIM1/2/3 input capture 1 connected to

lptim_ic1_mux0

GPIO

lptim_ic1_mux1

comp1_out

lptim_ic1_mux2

comp2_out(1)

lptim_ic1_mux3

Reserved

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

1. This connection is not present in STM32U535/545 as COMP2 is not available.

Table 608. LPTIM1 input capture 2 connections
lptim_ic2_mux

LPTIM1 input capture 2 connected to

lptim_ic2_mux0

I/O

lptim_ic2_mux1

LSI

lptim_ic2_mux2

LSE

lptim_ic2_mux3

Reserved

Table 609. LPTIM2 input capture 2 connections
lptim_ic2_mux

LPTIM2 input capture 2 connected to

lptim_ic2_mux0

I/O

lptim_ic2_mux1

HSI/256

lptim_ic2_mux2

MSIS/1024

lptim_ic2_mux3

MSIS/4

Table 610. LPTIM3 input capture 2 connections
lptim_ic2_mux

58.4.4

LPTIM3 input capture 2 connected to

lptim_ic2_mux0

I/O

lptim_ic2_mux1

Reserved

lptim_ic2_mux2

Reserved

lptim_ic2_mux3

Reserved

LPTIM reset and clocks
The LPTIM can be clocked using several clock sources. It can be clocked using an internal
clock signal which can be any configurable internal clock source selectable through the
RCC (see RCC section for more details). Also, the LPTIM can be clocked using an external
clock signal injected on its external Input1. When clocked with an external clock source, the
LPTIM can run in one of the following configurations:
•

The first configuration is when the LPTIM is clocked by an external signal but in the
same time an internal clock signal is provided to the LPTIM from configurable internal
clock source (see RCC section).

•

The second configuration is when the LPTIM is solely clocked by an external clock
source through its external Input1. This configuration is the one used to realize Timeout
function or pulse counter function when all the embedded oscillators are turned off after
entering a low-power mode.

Programming the CKSEL and COUNTMODE bits allows controlling whether the LPTIM
uses an external clock source or an internal one.
When configured to use an external clock source, the CKPOL bits are used to select the
external clock signal active edge. If both edges are configured to be active ones, an internal

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

clock signal must also be provided (first configuration). In this case, the internal clock signal
frequency must be at least four times higher than the external clock signal frequency.

58.4.5

Glitch filter
The LPTIM inputs, either external (mapped to GPIOs) or internal (mapped on the chip-level
to other embedded peripherals), are protected with digital filters that prevent any glitches
and noise perturbations to propagate inside the LPTIM. This is in order to prevent spurious
counts or triggers.
Before activating the digital filters, an internal clock source must first be provided to the
LPTIM. This is necessary to guarantee the proper operation of the filters.
The digital filters are divided into three groups:

Note:

•

The first group of digital filters protects the LPTIM internal or external inputs. The digital
filters sensitivity is controlled by the CKFLT bits

•

The second group of digital filters protects the LPTIM internal or external trigger inputs.
The digital filters sensitivity is controlled by the TRGFLT bits.

•

The third group of digital filters protects the LPTIM internal or external input captures.
The digital filters sensitivity is controlled by the ICxF bits.

The digital filters sensitivity is controlled by groups. It is not possible to configure each digital
filter sensitivity separately inside the same group.
The filter sensitivity acts on the number of consecutive equal samples that is detected on
one of the LPTIM inputs to consider a signal level change as a valid transition. Figure 738
shows an example of glitch filter behavior in case of a two consecutive samples
programmed.
Figure 738. Glitch filter timing diagram

CLKMUX
Input
Filter out

2 consecutive samples

2 consecutive samples

Filtered
MS32490V1

Note:

<!-- pagebreak -->

In case no internal clock signal is provided, the digital filter must be deactivated by setting
the CKFLT, ICxF and TRGFLT bits to 0. In this case, an external analog filter can be used to
protect the LPTIM external inputs against glitches.

RM0456 Rev 6

RM0456

58.4.6

Low-power timer (LPTIM)

Prescaler
The LPTIM 16-bit counter is preceded by a configurable power-of-2 prescaler. The prescaler
division ratio is controlled by the PRESC[2:0] field. The table below lists all the possible
division ratios:
Table 611. Prescaler division ratios

58.4.7

Programming

Dividing factor

000

/1

001

/2

010

/4

011

/8

100

/16

101

/32

110

/64

111

/128

Trigger multiplexer
The LPTIM counter can be started either by software or after the detection of an active edge
on one of the eight trigger inputs.
TRIGEN[1:0] is used to determine the LPTIM trigger source:
•

When TRIGEN[1:0] equals 00, the LPTIM counter is started as soon as one of the
CNTSTRT or the SNGSTRT bits is set by software. The three remaining possible
values for the TRIGEN[1:0] are used to configure the active edge used by the trigger
inputs. The LPTIM counter starts as soon as an active edge is detected.

•

When TRIGEN[1:0] is different than 00, TRIGSEL[2:0] is used to select which of the
eight trigger inputs is used to start the counter.

The external triggers are considered asynchronous signals for the LPTIM. After a trigger
detection, a two-counter-clock period latency is needed before the timer starts running due
to the synchronization.
If a new trigger event occurs when the timer is already started it is ignored (unless timeout
function is enabled).
Note:

The timer must be enabled before setting the SNGSTRT/CNTSTRT bits. Any write on these
bits when the timer is disabled is discarded by hardware.

Note:

When starting the counter by software (TRIGEN[1:0] = 00), there is a delay of 3 kernel clock
cycles between the LPTIM_CR register update (set one of SNGSTRT or CNTSTRT bits)
and the effective start of the counter.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.4.8

RM0456

Operating mode
The LPTIM features two operating modes:
•

Continuous mode: the timer is free running, the timer is started from a trigger event and
never stops until the timer is disabled

•

One-shot mode: the timer is started from a trigger event and stops when an LPTIM
update event is generated.

One-shot mode
To enable the one-shot counting, the SNGSTRT bit must be set.
A new trigger event re-starts the timer. Any trigger event occurring after the counter starts
and before the next LPTIM update event, is discarded.
In case an external trigger is selected, each external trigger event arriving after the
SNGSTRT bit is set, and after the repetition counter has stopped (after the update event),
and if the repetition register content is different from zero, the repetition counter gets
reloaded with the value already contained by the repetition register and a new one-shot
counting cycle is started as shown in Figure 739.
Figure 739. LPTIM output waveform, single-counting mode configuration
when repetition register content is different than zero (with PRELOAD = 1)
LPTIM_RCR

Repetition counter

2

2

1

0

2

LPTIM_ARR
Compare

0
PWM
External trigger event
Ignored external trigger event

MSv47414V1

- Set-once mode activated:
Note that when the WAVE bitfield in the LPTIM_CFGR register is set, the Set-once mode is
activated. In this case, the counter is only started once following the first trigger, and any
subsequent trigger event is discarded as shown in Figure 740.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)
Figure 740. LPTIM output waveform, single-counting mode configuration
and Set-once mode activated (WAVE bit is set)
LPTIM_ARR
Compare
Discarded trigger

0
PWM

External trigger event
MSv39231V2

In case of software start (TRIGEN[1:0] = 00), the SNGSTRT setting starts the counter for
one-shot counting.

Continuous mode
To enable the continuous counting, the CNTSTRT bit must be set.
In case an external trigger is selected, an external trigger event arriving after CNTSTRT is
set, starts the counter for continuous counting. Any subsequent external trigger event is
discarded as shown in Figure 741.
In case of software start (TRIGEN[1:0] = 00), setting CNTSTRT starts the counter for
continuous counting.
Figure 741. LPTIM output waveform, Continuous counting mode configuration
Discarded triggers

LPTIM_ARR
Compare

0
PWM

External trigger event
MSv39229V2

SNGSTRT and CNTSTRT bits can only be set when the timer is enabled (ENABLE bit set to
1). It is possible to change “on the fly” from One-shot mode to Continuous mode.
If the Continuous mode was previously selected, setting SNGSTRT switches the LPTIM to
the One-shot mode. The counter (if active) stops as soon as an LPTIM update event is
generated.
If the One-shot mode was previously selected, setting CNTSTRT switches the LPTIM to the
Continuous mode. The counter (if active) restarts as soon as it reaches ARR.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.4.9

RM0456

Timeout function
The detection of an active edge on one selected trigger input can be used to reset the
LPTIM counter. This feature is controlled through the TIMOUT bit.
The first trigger event starts the timer, any successive trigger event resets the LPTIM
counter and the repetition counter and the timer restarts.
A low-power timeout function can be realized. The timeout value corresponds to the
compare value; if no trigger occurs within the expected time frame, the MCU is waked-up by
the compare match event.

58.4.10

Waveform generation
Two 16-bit registers, the LPTIM_ARR (autoreload register) and LPTIM_CCRx
(capture/compare register), are used to generate several different waveforms on LPTIM
output
The timer can generate the following waveforms:
•

The PWM mode: the LPTIM output is set as soon as the counter value in LPTIM_CNT
exceeds the compare value in LPTIM_CCRx. The LPTIM output is reset as soon as a
match occurs between the LPTIM_ARR and the LPTIM_CNT register. For more details
see Section 58.4.19: PWM mode.

•

The One-pulse mode: the output waveform is similar to the one of the PWM mode for
the first pulse, then the output is permanently reset

•

The Set-once mode: the output waveform is similar to the One-pulse mode except that
the output is kept to the last signal level (depends on the output configured polarity).

The above described modes require the LPTIM_ARR register value to be strictly greater
than the LPTIM_CCRx register value.
The LPTIM output waveform can be configured through the WAVE bit as follow:
•

Resetting the WAVE bit to 0 forces the LPTIM to generate either a PWM waveform or a
One pulse waveform depending on which bit is set: CNTSTRT or SNGSTRT.

•

Setting the WAVE bit to 1 forces the LPTIM to generate a Set-once mode waveform.

The WAVPOL/CCxP bit controls the LPTIM output polarity. The change takes effect
immediately, so the output default value changes immediately after the polarity is reconfigured, even before the timer is enabled.
Signals with frequencies up to the LPTIM clock frequency divided by two can be generated.
Figure 742 below shows the three possible waveforms that can be generated on the LPTIM
output. Also, it shows the effect of the polarity change using the WAVPOL/CCxP bit.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)
Figure 742. Waveform generation

LPTIM_ARR
Compare
0
PWM
One shot

Pol = 0

Set once

PWM
Pol = 1

One shot
Set once

MS32467V2

58.4.11

Register update
The LPTIM_ARR register, the LPTIM_RCR register and the LPTIM_CCRx register are
updated immediately after the APB bus write operation or in synchronization with the next
LPTIM update event if the timer is already started.
The PRELOAD bit controls how the LPTIM_ARR, the LPTIM_RCR and the LPTIM_CCRx
registers are updated:
•

When the PRELOAD bit is reset to 0, the LPTIM_ARR, the LPTIM_RCR and the
LPTIM_CCRx registers are immediately updated after any write access.

•

When the PRELOAD bit is set to 1, the LPTIM_ARR, the LPTIM_RCR and the
LPTIM_CCRx registers are updated at next LPTIM update event, if the timer has been
already started.

The LPTIM APB interface and the LPTIM kernel logic use different clocks, so there is some
latency between the APB write and the moment when these values are available to the
counter comparator. Within this latency period, any additional write into these registers must
be avoided.
The ARROK flag, the REPOK flag and the CMPxOK flag in the LPTIM_ISR register indicate
when the write operation is completed to respectively the LPTIM_ARR register, the
LPTIM_RCR register and the LPTIM_CCRx register.
After a write to the LPTIM_ARR, the LPTIM_RCR or the LPTIM_CCRx register, a new write
operation to the same register can only be performed when the previous write operation is
completed. Any successive write before respectively the ARROK flag, the REPOK flag or
the CMPxOK flag be set, leads to unpredictable results.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.4.12

RM0456

Counter mode
The LPTIM counter can be used to count external events on the LPTIM input1 or it can be
used to count internal clock cycles. The CKSEL and COUNTMODE bits control which
source is used for updating the counter.
In case the LPTIM is configured to count external events on Input1, the counter can be
updated following a rising edge, falling edge or both edges depending on the value written
to the CKPOL[1:0] bits.
The count modes below can be selected, depending on CKSEL and COUNTMODE values:
•

CKSEL = 0: the LPTIM is clocked by an internal clock source
–

COUNTMODE = 0
The LPTIM is configured to be clocked by an internal clock source and the LPTIM
counter is configured to be updated following each internal clock pulse.

–

COUNTMODE = 1
The LPTIM external Input1 is sampled with the internal clock provided to the
LPTIM.
Consequently, in order not to miss any event, the frequency of the changes on the
external Input1 signal must never exceed the frequency of the internal clock
provided to the LPTIM. Also, the internal clock provided to the LPTIM must not be
prescaled (PRESC[2:0] = 000).

•

CKSEL = 1: the LPTIM is clocked by an external clock source
COUNTMODE value is don’t care.
In this configuration, the LPTIM has no need for an internal clock source (except if the
glitch filters are enabled). The signal injected on the LPTIM external input1 is used as
system clock for the LPTIM. This configuration is suitable for operation modes where
no embedded oscillator is enabled.
For this configuration, the LPTIM counter can be updated either on rising edges or
falling edges of the input1 clock signal but not on both rising and falling edges.
Since the signal injected on the LPTIM external input1 is also used to clock the LPTIM
kernel logic, there is some initial latency (after the LPTIM is enabled) before the counter
is incremented. More precisely, the first five active edges on the LPTIM external Input1
(after LPTIM is enable) are lost.

58.4.13

Timer enable
The ENABLE bit located in the LPTIM_CR register is used to enable/disable the LPTIM
kernel logic. After setting the ENABLE bit, a delay of two counter clock cycles is needed
before the LPTIM is actually enabled.
The LPTIM_CFGR register must be modified only when the LPTIM is disabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

58.4.14

Low-power timer (LPTIM)

Timer counter reset
In order to reset the content of LPTIM_CNT register, two reset mechanisms are
implemented:
•

Note:

The synchronous reset mechanism: the synchronous reset is controlled by the
COUNTRST bit in the LPTIM_CR register. After setting the COUNTRST bitfield to '1',
the reset signal is propagated in the LPTIM kernel clock domain. So it is important to
note that a few clock pulses of the LPTIM kernel logic elapse before the reset is taken
into account. This makes the LPTIM counter count few extra pluses between the time
when the reset is triggered and it become effective. Since the COUNTRST bit is
located in the APB clock domain and the LPTIM counter is located in the LPTIM kernel
clock domain, a delay of 3 clock cycles of the kernel clock is needed to synchronize the
reset signal issued by the APB clock domain when writing '1' to the COUNTRST bit.

The software should ensure that COUNRST bit is '0' before generating every synchronous
reset.
•

The asynchronous reset mechanism: the asynchronous reset is controlled by the
RSTARE bit located in the LPTIM_CR register. When this bit is set to '1', any read
access to the LPTIM_CNT register resets its content to zero. Asynchronous reset must
be triggered within a timeframe in which no LPTIM core clock is provided. For example
when LPTIM Input1 is used as external clock source, the asynchronous reset must be
applied only when there is enough insurance that no toggle occurs on the LPTIM
Input1.
To read reliably the content of the LPTIM_CNT register two successive read accesses
must be performed and compared. A read access can be considered reliable when the
value of the two read accesses is equal. Unfortunately when asynchronous reset is
enabled there is no possibility to read twice the LPTIM_CNT register.

Warning:

58.4.15

There is no mechanism inside the LPTIM that prevents the
two reset mechanisms from being used simultaneously. The
developer must make sure that these two mechanisms are
used exclusively.

Encoder mode
This mode allows handling signals from quadrature encoders used to detect angular
position of rotary elements. Encoder interface mode acts simply as an external clock with
direction selection. This means that the counter just counts continuously between 0 and the
auto-reload value programmed into the LPTIM_ARR register (0 up to ARR or ARR down to
0 depending on the direction). Therefore LPTIM_ARR must be configured before starting
the counter. From the two external input signals, Input1 and Input2, a clock signal is
generated to clock the LPTIM counter. The phase between those two signals determines
the counting direction.
The Encoder mode is only available when the LPTIM is clocked by an internal clock source.
The signals frequency on both Input1 and Input2 inputs must not exceed the LPTIM internal
clock frequency divided by 4. This is mandatory in order to guarantee a proper operation of
the LPTIM.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Direction change is signalized by the two down and up flags in the LPTIM_ISR register. An
interrupt can be generated for both direction change events if enabled through the DOWNIE
bit.
To activate the Encoder mode the ENC bit has to be set to 1. The LPTIM must first be
configured in Continuous mode.
When Encoder mode is active, the LPTIM counter is modified automatically following the
speed and the direction of the incremental encoder. Therefore, its content always
represents the encoder’s position. The count direction, signaled by the Up and Down flags,
correspond to the rotation direction of the encoder rotor.
According to the edge sensitivity configured using the CKPOL[1:0] bits, different counting
scenarios are possible. The following table summarizes the possible combinations,
assuming that Input1 and Input2 do not switch at the same time.
Table 612. Encoder counting scenarios
Active edge

Rising Edge

Falling Edge

Both Edges

Level on opposite
signal (Input1 for
Input2, Input2 for
Input1)

Input1 signal

Input2 signal

Rising

Falling

Rising

Falling

High

Down

No count

Up

No count

Low

Up

No count

Down

No count

High

No count

Up

No count

Down

Low

No count

Down

No count

Up

High

Down

Up

Up

Down

Low

Up

Down

Down

Up

The following figure shows a counting sequence for Encoder mode where both-edge
sensitivity is configured.
Caution:

<!-- pagebreak -->

In this mode the LPTIM must be clocked by an internal clock source, so the CKSEL bit must
be maintained to its reset value which is equal to 0. Also, the prescaler division ratio must be
equal to its reset value which is 1 (PRESC[2:0] bits must be 000).

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)
Figure 743. Encoder mode counting sequence

T1
T2

Counter

up

58.4.16

down

up

MS32491V1

Repetition counter
The LPTIM features a repetition counter that decrements by 1 each time an LPTIM counter
overflow event occurs. A repetition counter underflow event is generated when the repetition
counter contains zero and the LPTIM counter overflows. Next to each repetition counter
underflow event, the repetition counter gets loaded with the content of the REP[7:0] bitfield
which belongs to the repetition register LPTIM_RCR.
A repetition underflow event is generated on each and every LPTIM counter overflow when
the REP[7:0] register is set to 0.
When PRELOAD = 1, writing to the REP[7:0] bitfield has no effect on the content of the
repetition counter until the next repetition underflow event occurs. The repetition counter
continues to decrement each LPTIM counter overflow event and only when a repetition
underflow event is generated, the new value written into REP[7:0] is loaded into the
repetition counter. This behavior is depicted in Figure 744.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Figure 744. Continuous counting mode when repetition register LPTIM_RCR
different from zero (with PRELOAD = 1)
Repetition counter underflow event
LPTIM_RCR

4

Repetition counter

1

9

0

LPTIM_ARR
Compare

9

8

7

Preloaded registers updated

0
PWM
MSv47415V1

A repetition counter underflow event is systematically associated with LPTIM preloaded
registers update (refer to Section 58.4.11: Register update for more information).
Repetition counter underflow event is signaled to the software through the update event
(UE) flag mapped into the LPTIM_ISR register. When set, the UE flag can trigger an LPTIM
interrupt if its respective update event interrupt enable (UEIE) control bit, mapped to the
LPTIM_DIER register, is set.
The repetition register LPTIM_RCR is located in the APB bus interface clock domain where
the repetition counter itself is located in the LPTIM kernel clock domain. Each time a new
value is written to the LPTIM_RCR register, this new content is propagated from the APB
bus interface clock domain to the LPTIM kernel clock domain. The new written value is then
loaded to the repetition counter immediately after a repetition counter underflow event. The
synchronization delay for the new written content is four APB clock cycles plus three LPTIM
kernel clock cycles and it is signaled by the REPOK flag located in the LPTIM_ISR register
when it is elapsed. When the LPTIM kernel clock cycle is relatively slow, for instance when
the LPTIM kernel is being clocked by the LSI clock source, it can be lengthy to keep polling
on the REPOK flag by software to detect that the synchronization of the LPTIM_RCR
register content is finished. For that reason, the REPOK flag, when set, can generate an
interrupt if its associated REPOKIE control bit in the LPTIM_DIER register is set.
Note:

After a write to the LPTIM_RCR register, a new write operation to the same register can only
be performed when the previous write operation is completed. Any successive writes before
the REPOK flag is set, lead to unpredictable results.

Caution:

When using repetition counter with PRELOAD = 0, LPTIM_RCR register must be changed
at least five counter cycles before the autoreload match event, otherwise an unpredictable
behavior may occur.

58.4.17

Capture/compare channels
Each capture/compare channel is built around a capture/compare register, an input stage
for capture (with digital filter, multiplexing and prescaler) and an output stage (with
comparator and output control) for PWM.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Input stage
The input stage samples the corresponding LPTIx input to generate a filtered signal LPTIxF.
Then, an edge detector with polarity selection generates ICx signal used as the capture
command. It is prescaled to generate the capture command signal (ICxPS).
Figure 745. Capture/compare input stage (channel 1)
LPTI1

Glitch filter

IC1F[3:0]
LPTIM_CCMR1

LPTI1F

IC1

Edge detector

CC1P[1:0]
LPTIM_CCMR1

Divider
/1, /2, /4, /8

IC1PSC[1:0]
CC1E
LPTIM_CCMR1

IC1PS

MSv50905V1

Output stage
The output stage generates an intermediate waveform which is then used for reference:
OCxREF (active high). The polarity acts at the end of the chain.
Figure 746. Capture/compare output stage (channel 1)
CNT > CCRx

58.4.18

Output mode
controller

OC1REF

0

1

Output
enable
circuit

CC1P
LPTIM_CCMR1

CC1E
LPTIM_CCMR1

OC1

MSv50906V2

Input capture mode
In Input capture mode, the capture/compare registers (LPTIM_CCRx) are used to latch the
value of the counter after a transition detected by the corresponding ICx signal. Assuming
input capture is enabled on a channel x (CCxE set) and when a capture occurs, the
corresponding CCxIF flag (LPTIM_ISR register) is set and an interrupt or a DMA request
can be sent if they are enabled. If a capture occurs while the CCxIF flag was already high,
then the over-capture flag CCxOF (LPTIM_ISR register) is set. CCxIF can be cleared by
software by writing the CCxICF to 1 or by reading the captured data stored in the
LPTIM_CCRx register. CCxOF is cleared by writing CCxOCF to 1.

Note:

In DMA mode, the input capture channel have to be enabled (set CCxE bit) the last, after
enabling the DMA request and after starting the counter. This is in order to prevent
generating an input capture DMA request when the counter is not started yet.

Input capture Glitch filter latency
When a trigger event arrives on channel x input (LPTIx) and depending on the configured
glitch filter (ICxF[1:0] field in CCMRx register) and on the kernel clock prescaler value
(PRESC[2:0] field in CFGR register), there is a variable latency that leads to a systematic
offset (see Table 613) between the captured value stored in the CCRx register and the real
value corresponding to the capture trigger.
This offset has no impact on pulse width measurement as it is systematic and compensated
between two captures.
RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

The real capture value corresponding to the input capture trigger can be calculated using
the below formula:
Real capture value = captured(LPTIM_CCRx) - offset
The relevant offset must be used depending on the glitch filter and on the kernel clock
prescaler value (PRESC field in CFGR register)
Example: determining the real capture value when PRESC[2:0] = 0x2 and ICxF = 0x3.
For this configuration (PRESC[2:0] = 0x2 and ICxF = 0x3) and according to the
Table 613, the offset is 5.
Assuming that the captured value in CCRx is 9 (LPTIM_CNT = 9), this means that the
capture trigger occurred when the LPTIM_CNT was equal to 9 - 5 = 4.
Table 613. Input capture Glitch filter latency (in counter step unit)
Prescaler PRESC[2:0]

0

1

2

3

4

5

<!-- pagebreak -->

RM0456 Rev 6

ICxF[1:0]

Offset

0

2

1

7

2

9

3

13

0

3

1

5

2

6

3

8

0

2

1

3

2

4

3

5

0

2

1

2

2

3

3

3

0

2

1

2

2

2

3

2

0

2

1

2

2

2

3

2

RM0456

Low-power timer (LPTIM)
Table 613. Input capture Glitch filter latency (in counter step unit) (continued)
Prescaler PRESC[2:0]

6

7

58.4.19

ICxF[1:0]

Offset

0

2

1

2

2

2

3

2

0

2

1

2

2

2

3

2

PWM mode
The PWM mode enables to generate a signal with a frequency determined by the value of
the LPTIM_ARR register and a duty cycle determined by the value of the LPTIM_CCRx
register. The LPTIM is able to generate PWM in edge-aligned mode.
OCx polarity is software programmable using the CCxP bit in the LPTIM_CCMRx register. It
can be programmed as active high or active low. OCx output is enabled by the CCxE bit in
the LPTIM_CCMRx register. Refer to the LPTIM_CCMRx register description for more
details.
Figure 747 gives an example where the LPTIM channel 1 is configured in PWM mode with
LPTIM_CCR1 = 6 then 1 and LPTIM_ARR=10.
Figure 747. Edge-aligned PWM mode (PRELOAD = 1)
LPTIM_ARR

10

6

LPTIM_CCR1

LPTIM_CNT

5

6

1

7

8

9

10

0

1

2

OC1REF = OC1
Compare match

Auto reload
match

Compare match
MSv50907V1

In the following example the reference PWM signal OCxREF is low as long as
LPTIM_CNT ≤ LPTIM_CCRx else it becomes high.
Figure 748 shows some edge-aligned PWM waveforms in an example where
LPTIM_ARR = 8.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Figure 748. Edge-aligned PWM waveforms (ARR=8 and CCxP = 0)
Counter register

0

1

2

3

4

5

6

7

8

0

1

OCxREF
CCRx=3
CCxIF

OCxREF
CCRx=6
CCxIF

OCxREF ‘0’
CCRx=0
CCxIF
MSv50908V2

PWM mode with immediate update PRELOAD = 0
The PWM mode with PRELOAD = 0 enables the early change of the output level within the
current PWM cycle. Based on the immediate update (PRELOAD = 0) of the LPTIM_CCRx
register and on the continuous comparison of LPTIM_CNT and LPTIM_CCRx registers, it
permits to have a new duty cycle value applied as soon as possible within the current PWM
cycle, without having to wait for the completion of the current PWM period.
When the (PRELOAD = 0), the OCxREF signal level can be changed on-the-fly by software
(or DMA) by updating the compare value in the LPTIM_CCRx register.
Depending on the written compare value and on the current counter and compare values,
the OCxREF level is re-assigned as illustrated below:
•

If the new compare value does not exceed the current counter value and the current
compare value exceeds the counter, OCxREF level is re-assigned high as soon as the
new compare value is written.

•

If the new compare value exceeds the counter value and the current compare value
does not exceed the counter, OCxREF level is re-assigned low as soon as the new
compare value is written.

The output reference signal OCxREF level is left unchanged when none of the new
compare value and the current compare value exceed the counter. Figure 749 illustrates the
behavior of the OCxREF signal level when PRELOAD = 0 and PRELOAD = 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)
Figure 749. PWM mode with immediate update versus preloaded update
Write to CCRx

Write to CCRx

Write to CCRx

LPTIM_CNT

LPTIM_CCRx

PWM mode with
immediate update
PRELOAD = 0

OCx

PWM mode with
preloaded update
PRELOAD = 1

OCx
MSv50938V2

Note:

For both PWM modes, the compare match, auto-reload match, and the update event flags
are set one LPTIM counter cycle later after the corresponding event, the OCxREF level is
also changed one LPTIM counter cycle later after the corresponding event. For instance
when the LPTIM_CCRx is set to 3 the CCxIF is set when the LPTIM_CNT = 4. Figure 747
illustrates this behavior.

58.4.20

Autonomous mode
When clocked by oscillators available in this mode (refer to RCC), the LPTIM can operate in
autonomous mode, remaining then fully functional in Stop mode where the APB clock is
stopped. The APB clock is requested by the peripheral each time a data must be transferred
from or to the SRAM. Once the APB clock is received by the peripheral, either an interrupt
or a DMA request is generated, depending on the LPTIM configuration.
In order to offload the CPU (in Run mode) or to avoid to wake it up when in Stop mode, it is
possible to use LPTIM DMA requests to transfer the captured values when in input capture
mode or to update LPTIM registers when in PWM mode.
When in Stop mode, the LPTIM counter can be automatically started after the detection of
an active edge on one of its external input triggers.

Input capture mode
To operate autonomously in Stop mode, the input capture DMA request must be enabled by
setting the CCxDE bit in the LPTIM_DIER register.
Each time a counter value is captured and available in the LPTIM_CCRx register, the APB
clock is requested by the peripheral and a DMA request is generated. The captured value is
then transferred to the SRAM. The CCxIF flag is automatically cleared by hardware once
the captured value is read by APB (can be any bus master like CPU or DMA).

PWM mode
The LPTIM can be configured to autonomously change, at each update event, the output
waveform pulse width and/or the duty cycle without any CPU intervention. To enable this
autonomous mode, the corresponding UEDE bit must be set in the LPTIM_DIER register.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

At each update event, the APB clock is requested by the peripheral and a DMA request is
generated. DMA direction must be configured as memory-to-peripheral which enables
updating LPTIM registers, at each DMA request, with values stored in SRAM.
The UE flag is automatically cleared by hardware once the LPTIM_ARR register is written
by any bus master like CPU or DMA. Thus, to enable automatic hardware clearing of UE
flag, the application must configure the LPTIM_ARR register to be the last one to be written
(at the end of list). For instance, if LPTIM_CCR1 and LPTIM_RCR registers need to be
updated in Stop mode by DMA, the update sequence must be: LPTIM_CCR1, LPTIM_RCR
then LPTIM_ARR.
The UE flag can also be cleared over its corresponding clear bit UECF in the LPTIM_ICR
register, this can be done by configuring the DMA to write the LPTIM_ICR register at the end
of register update.

58.4.21

DMA requests
The LPTIM can generate two categories of DMA requests:
•

DMA requests used to retrieve the input-capture counter values

•

DMA update requests used to re-program part of the LPTIM, multiple times, at regular
intervals, without software overhead

Input capture DMA request
Each LPTIM channel has its dedicated input capture DMA request. A DMA request is
generated (if CCxDE bit is set in LPTIM_DIER) and CCxIF is set each time a capture is
ready in the LPTIM_CCRx register. The captured values in LPTIM_CCRx can then be
transferred regularly by DMA to the desired memory destination. The CCxIF is automatically
cleared by hardware when the captured value in LPTIM_CCRx register is read.
Note:

The ICx DMA request signal lptim_icx_dma is reset in the following conditions:
- if the corresponding DMA request is disabled (clear CCxDE bit in the LPTIM_DIER
register)
- or if the channel x is disabled (clear CCxE bit)
- or if the LPTIM is disabled (clear the ENABLE bit in the LPTIM_CR register)

Update event DMA request
A DMA request is generated (if UEDE is set in LPTIM_DIER) and the UE flag is set at each
update event. DMA request can be used to regularly update the LPTIM_ARR, the
LPTIM_RCR or the LPTIM_CCRx registers permitting to generate custom PWM waveforms.
The UE is automatically cleared by hardware upon any bus master (like CPU or DMA) write
access to the LPTIM_ARR register.
Note:

The UE DMA request signal lptim_ue_dma is reset in the following conditions:
- if the corresponding DMA request is disabled (clear UEDE bit in the LPTIM_DIER register)
- or if the LPTIM is disabled (clear the ENABLE bit in the LPTIM_CR register)
- or if the channel x is disabled (clear CCxE bit) and all the other channels are already
disabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

58.4.22

Low-power timer (LPTIM)

Debug mode
When the microcontroller enters debug mode (core halted), the LPTIM counter either
continues to work normally or stops, depending on the timer dedicated bit configuration in
the debug support (DBG) peripheral.
For further details, refer to section debug support (DBG).

58.5

LPTIM low-power modes
Table 614. Effect of low-power modes on the LPTIM
Mode

Description

Sleep

No effect. LPTIM interrupts cause the device to exit Sleep mode.

Stop(1)

If the LPTIM is clocked by an oscillator available in Stop mode, LPTIM is
functional and the interrupts cause the device to exit the Stop mode. The
DMA requests are functional if the instance supports the autonomous
mode (refer to Section 58.3: LPTIM implementation).

Standby

The LPTIM peripheral is powered down and must be reinitialized after
exiting Standby mode.

1. Only LPTIM1 and LPTIM3 support autonomous mode with wake-up capability in Stop 2 mode.
LPTIM4 does not support autonomous mode but has wake-up capability in stop 2 mode.
LPTIM2 must be disabled before entering Stop 2 mode.

58.6

LPTIM interrupts
The following events generate an interrupt/wake-up event, if they are enabled through the
LPTIM_DIER register:

Note:

•

Compare match

•

Auto-reload match (whatever the direction if encoder mode)

•

External trigger event

•

Autoreload register write completed

•

Compare register write completed

•

Direction change (encoder mode), programmable (up / down / both).

•

Update Event

•

Repetition register update OK

•

Input capture occurred

•

Over-capture occurred

•

Interrupt enable register update OK

If any bit in the LPTIM_DIER register is set after that its corresponding flag in the
LPTIM_ISR register (status register) is set, the interrupt is not asserted.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456
Table 615. Interrupt events

Interrupt
vector

Event flag

Enable
control bit

Interrupt clear
method

Compare match

CCxIF

CCxIE

Write 1 to CCxCF

Yes

Yes

(2)

CCxIF

CCxIE

Write 1 to CCxCF

Yes

Yes

Over-capture(2)

CCxOF

CCxOIE

Write 1 to
CCxOCF

Yes

Yes

Auto-reload match

ARRM

ARRMIE

Write 1 to
ARRMCF

Yes

Yes

External trigger event

EXTTRIG

EXTTRIGIE

Write 1 to
EXTTRIGCF

Yes

Yes

Auto-reload register
update OK

ARROK

ARROKIE

Write 1 to
ARROKCF

Yes

Yes

Capture/compare
register update OK

CMPxOK

CMPxOKIE

Write 1 to
CMPxOKCF

Yes

Yes

Direction change to
up(3)

UP

UPIE

Write 1 to UPCF

Yes

Yes

Direction change to
down(3)

DOWN

DOWNIE

Write 1 to
DOWNCF

Yes

Yes

UE

UEIE

Write 1 to UECF

Yes

Yes

REPOK

REPOKIE

Write 1 to
REPOKCF

Yes

Yes

Interrupt event

Input capture

LPTIMx

Update event
Repetition register
update OK

Exit from Exit from Stop
Sleep mode
mode(1)

1. Each LPTIM event can wake up the device from Stop mode only if the LPTIM instance supports the wake-up
from Stop mode feature. Refer to Section 58.3: LPTIM implementation.
2. If LPTIM does not implement any channel this event does not exist. Refer to Section 58.3: LPTIM
implementation.
3. If LPTIM does not support encoder mode feature, this event does not exist. Refer to Section 58.3: LPTIM
implementation.

58.7

LPTIM registers
Refer to Section 1.2: List of abbreviations for registers for a list of abbreviations used in
register descriptions.
The peripheral registers can only be accessed by words (32-bit).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

58.7.1

LPTIM4 interrupt and status register (LPTIM4_ISR)
Address offset: 0x000
Reset value: 0x0000 0000

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

DIER
OK

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REP
OK

UE

DOWN

UP

ARR
OK

CMP1
OK

EXT
TRIG

ARRM

CC1IF

r

r

r

r

r

r

r

r

r

r

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROK: Interrupt enable register update OK
DIEROK is set by hardware to inform application that the APB bus write operation to the
LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to
the DIEROKCF bit in the LPTIM_ICR register.
Bits 23:9 Reserved, must be kept at reset value.
Bit 8 REPOK: Repetition register update OK
REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR
register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF
bit in the LPTIM_ICR register.
Bit 7 UE: LPTIM update event occurred
UE is set by hardware to inform application that an update event was generated. UE flag can be
cleared by writing 1 to the UECF bit in the LPTIM_ICR register.
Bit 6 DOWN: Counter direction change up to down
In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has
changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the
LPTIM_ICR register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3:
LPTIM implementation.
Bit 5 UP: Counter direction change down to up
In Encoder mode, UP bit is set by hardware to inform application that the counter direction has
changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR
register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3:
LPTIM implementation.
Bit 4 ARROK: Autoreload register update OK
ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR
register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF
bit in the LPTIM_ICR register.
Bit 3 CMP1OK: Compare register 1 update OK
CMP1OK is set by hardware to inform application that the APB bus write operation to the
LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1
to the CMP1OKCF bit in the LPTIM_ICR register.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bit 2 EXTTRIG: External trigger edge event
EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger
input has occurred. If the trigger is ignored because the timer has already started, then this flag is
not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
Bit 1 ARRM: Autoreload match
ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the
LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the
LPTIM_ICR register.
Bit 0 CC1IF: Compare 1 interrupt flag
The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the
compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the
LPTIM_ICR register.
0: No match
1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR1 register's
value

58.7.2

LPTIMx interrupt and status register [alternate] (LPTIMx_ISR)
(x = 1 to 3)
This description of the register can only be used for output compare mode. See next section
for input capture mode.
Address offset: 0x000
Reset value: 0x0000 0000

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25

24

Res.

DIER
OK

23
Res.

22
Res.

21
Res.

20

19

18

17

16

Res.

CMP2
OK

Res.

Res.

Res.

r
15
Res.

14
Res.

13
Res.

12
Res.

11
Res.

10
Res.

r

9

8

CC2IF

REP
OK

7
UE

r

r

r

6

5

4

3

2

1

0

DOWN

UP

ARR
OK

CMP1
OK

EXT
TRIG

ARRM

CC1IF

r

r

r

r

r

r

r

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROK: Interrupt enable register update OK
DIEROK is set by hardware to inform application that the APB bus write operation to the
LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to
the DIEROKCF bit in the LPTIM_ICR register.
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 Reserved, must be kept at reset value.
Bit 20 Reserved, must be kept at reset value.
Bit 19 CMP2OK: Compare register 2 update OK
CMP2OK is set by hardware to inform application that the APB bus write operation to the
LPTIM_CCR2 register has been successfully completed. CMP2OK flag can be cleared by writing 1
to the CMP2OKCF bit in the LPTIM_ICR register.
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bits 18:12 Reserved, must be kept at reset value.

<!-- pagebreak -->

