0

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

0x0A8

0

RELOAD[15:0]
0

Res.

Reset value
GFXTIM_
WDGPAR

0

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

0x0A4

0

VALUE[15:0]
0

Res.

Reset value
GFXTIM_
WDGRR

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

0x0A0

0

Reserved
Res.

Reserved
GFXTIM_
WDGCR

0

FRAME[11:0]

0

Res.

0x0900x09C

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

0x08C

Res.

Reset value
GFXTIM_
RFC2RR

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

GFXTIM_RFC2R

Res.

Reset value
0x088

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

0x084

0

FRAME[11:0]
0

Res.

Reset value
GFXTIM_
RFC1RR

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

GFXTIM_RFC1R

0

Reserved
Res.

0x080

0

LINE[11:0]
0

Reserved

Res.

0x0780x07C

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

0x074

0

LINE[11:0]
0

Res.

Reset value
GFXTIM_
ALCC2R

0

Reserved
Res.

Reserved
GFXTIM_
ALCC1R

0

Res.

0

Res.

Reset value
0x0640x06C

FRAME[19:0]

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

GFXTIM_
AFCC1R

0x060

Res.

Register
name

Res.

Offset

Res.

Table 621. GFXTIM register map and reset values (continued)

0

0

0

0

0

0

0

0

RM0456

60

Infrared interface (IRTIM)

Infrared interface (IRTIM)
An infrared interface (IRTIM) for remote control is available on the device. It can be used
with an infrared LED to perform remote control functions.
It uses internal connections with TIM16, and TIM17 as shown in Figure 765.
To generate the infrared remote control signals, the IR interface must be enabled and TIM16
channel 1 (TIM16_OC1) and TIM17 channel 1 (TIM17_OC1) must be properly configured to
generate correct waveforms.
The infrared receiver can be implemented easily through a basic input capture mode.
Figure 765. IRTIM internal hardware connections with TIM16 and TIM17
TIM17_CH1

IRTIM

IR_OUT

TIM16_CH1

MS30474V2

All standard IR pulse modulation modes can be obtained by programming the two timer
output compare channels.
TIM17 is used to generate the high frequency carrier signal, while TIM16 generates the
modulation envelope.
The infrared function is output on the IR_OUT pin. The activation of this function is done
through the GPIOx_AFRx register by enabling the related alternate function bit.
The high sink LED driver capability (only available on the PB9 pin) can be activated through
the PB9_FMP bit in the SYSCFG_CFGR1 register and used to sink the high current needed
to directly control an infrared LED.

RM0456 Rev 6

<!-- pagebreak -->

2563

Independent watchdog (IWDG)

RM0456

61

Independent watchdog (IWDG)

61.1

Introduction
The independent watchdog (IWDG) peripheral offers a high safety level, thanks to its
capability to detect malfunctions due to software or hardware failures.
The IWDG is clocked by an independent clock, and stays active even if the main clock fails.
In addition, the watchdog function is performed in the VDD voltage domain, allowing the
IWDG to remain functional even in low power modes. Refer to Section 61.3 to check the
capability of the IWDG in this product.
The IWDG is best suited for applications that require the watchdog to run as a totally
independent process outside the main application, making it very reliable to detect any
unexpected behavior.

61.2

61.3

IWDG main features
•

12-bit down-counter

•

Dual voltage domain, thus enabling operation in low power modes

•

Independent clock

•

Early wake-up interrupt generation

•

Reset generation
–

In case of timeout

–

In case of refresh outside the expected window

IWDG implementation
Table 622. STM32U5 series IWDG features(1)
IWDG modes/features
LSI used as IWDG kernel clock (iwdg_ker_ck)

X

Window function

X

Early wake-up interrupt generation

X

System reset generation(2)

X

Capability to work in system Stop

X

Capability to work in system Standby

X

Capability to generate a wake-up interrupt in system Stop(3)

X

Capability to generate a wake-up interrupt in system Standby

-

Capability to be frozen when the microcontroller enters in debug mode(4)

X

Option bytes to control the activity in Stop mode(5)

X

Option bytes to control the activity in Standby mode(6)

X

Option bytes to control the hardware mode(7)

X

1. ‘X’ = supported, ‘-’ = not supported.
2. Refer to the RCC section for additional information.

<!-- pagebreak -->

