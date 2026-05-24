609

Reset and clock control (RCC)

11.4.1

RM0456

HSE clock
The high-speed external clock signal (HSE) can be generated from two possible clock
sources:
•

HSE external crystal/ceramic resonator

•

HSE user external clock

The resonator and the load capacitors must be placed as close as possible to the oscillator
pins in order to minimize output distortion and startup stabilization time. The loading
capacitance values must be adjusted according to the selected oscillator.
Figure 39. HSE/ LSE clock sources
Clock source

Hardware configuration

OSC_IN

OSC_OUT

External clock

GPIO
External
source

OSC_IN OSC_OUT
Crystal/ceramic
resonators
CL1

Load
capacitors

CL2

External crystal/ceramic resonator (HSE crystal)
The 4 to 50 MHz external oscillator has the advantage of producing a very accurate rate on
the main clock.
The associated hardware configuration is shown in Figure 39. Refer to the electrical
characteristics section of the datasheet for more details.
HSERDY in RCC_CR indicates if the HSE oscillator is stable or not. At startup, the clock is
not released until this bit is set by hardware. An interrupt can be generated if enabled
in RCC_CIER.
The HSE crystal can be switched on and off using HSEON in RCC_CR.

<!-- pagebreak -->

