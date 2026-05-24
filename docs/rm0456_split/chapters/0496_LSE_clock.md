609

Reset and clock control (RCC)

RM0456
Figure 42. PLL initialization flow

PLL enable sequence
integer mode

PLL enable sequence
fractional mode

Select clock source
(RCC_PLLxCFGR)
- (PLLSRC)

Select clock source (RCC_PLLxCFGR)
- (PLLSRC)

Init pre-divider (RCC_PLLxCFGR)
- PLLxM

Init pre-divider (RCC_PLLxCFGR)
- PLLxM

PLLx config (RCC_PLLCFGR)
- PLLxRGE
- PLLxFRACEN = 0
- PLLxPEN, PLLxQEN, PLLxREN
Init PLLx dividers (RCC_PLLxDIVR)
- PLLxN, PLLxP, PLLxQ, PLLxR

Init fractional value (RCC_PLLxFRACR)
- FRACN= FracInitValue
PLLx config (RCC_PLLCFGR)
- PLLxRGE
- PLLxFRACEN = 1
- PLLxPEN, PLLxQEN, PLLxREN
Init PLLx dividers (RCC_PLLxDIVR)
- PLLxN, PLLxP, PLLxQ, PLLxR

Can be repeated
for each PLL

Enable PLLx (RCC_CR)
- PLLxON = 1
Enable PLLx (RCC_CR)
- PLLxON = 1
No

PLLxRDY = 1 ?

No

PLLxRDY = 1 ?

Yes
Yes

Ready for use in integer
mode

Disable fractional mode (RCC_PLLCFGR)
- PLLxFRACEN = 0
Init fractional value (RCC_PLLxFRACR)
- FRACN= FracValue(n)
Enable fractional mode (RCC_PLLCFGR)
- PLLxFRACEN = 1

Ready for use in
fractional mode

Value update
on-the-fly
Ready for use in
fractional mode
MSv65659V1

Note:

When the PLLxRDY goes to 1, it means that the difference between the PLLx output
frequency and the target value is lower than ± 2 %.

11.4.7

LSE clock
The LSE crystal is a 32.768 kHz low-speed external crystal or ceramic resonator. It has the
advantage of providing a low-power but highly accurate clock source to the real-time clock
peripheral (RTC) for clock/calendar or other timing functions.
The LSE crystal is switched on and off using LSEON in RCC_BDCR. If the LSE is used by
other peripherals of functions than RTC, TAMP, and LSECSS, the LSESYSEN bit must be
also be set in RCC_BDCR (refer to LSE when used by peripherals other than RTC/TAMP,
and RCC functions).
The crystal oscillator driving strength is configured using the LSEDRV[1:0] bits, according to
crystal specification, to obtain the best compromise between robustness and short startup

<!-- pagebreak -->

