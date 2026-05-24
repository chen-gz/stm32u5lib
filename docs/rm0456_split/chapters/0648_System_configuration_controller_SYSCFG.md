647

System configuration controller (SYSCFG)

15

System configuration controller (SYSCFG)

15.1

SYSCFG main features

RM0456

The STM32U5 series devices feature a set of configuration registers. The main purposes of
the system configuration controller are the following:
•

Managing robustness feature

•

Configuring FPU interrupts

•

Enabling/disabling the FMP high-drive mode of some I/Os and voltage booster for I/Os
analog switches

•

Managing the I/O compensation cell

•

Configuring register security access

•

Configuring the OTG_HS PHY (only for STM32U59x/5Ax/5Fx/5Gx)

•

Adjust the HSPI supply capacitance (only for STM32U59x/5Ax/5Fx/5Gx)

•

Disable internal SRAMs cacheability by DCACHE2
(only for STM32U59x/5Ax/5Fx/5Gx)

15.2

SYSCFG functional description

15.2.1

I/O compensation cell management
The I/O compensation cell generates an 8-bit value for the I/O buffer (4 bits for N-MOS and
4 bits for P-MOS), which depends on PVT operating conditions (process, voltage,
temperature). These bits are used to control the current slew-rate and output impedance in
the I/O buffer. Three compensation cells are embedded, one for the I/Os supplied by VDD,
one for the I/Os supplied by VDDIO2, and one dedicated to GPIOs with HSPI alternate
functions (AF) capabilities.
By default, the compensation cells are disabled, and a fixed code is applied to all the I/Os.
The HSI is used by the compensation cells and must be enabled before enabling the
compensation cells in SYSCFG_CCCSR.
When enabled, the compensation cell tracks the PVT, and the 8-bit code PCVx and NCVx
(x = 1 for I/Os supplied by VDD except the HSPI AF capabilities, x = 2 for I/Os supplied by
VDDIO2 and x = 3 for I/Os with HSPI AF capabilities supplied by VDD) are available in
SYSCFG_CCVR once the RDYx is set. If the CSx bit is cleared, the I/Ox receives the code
from SYSCFG_CCVR, resulting from the compensation cell.
To optimize the trimming, the code can be adjusted through SYSCFG_CCCR. Three sets of
bits are available: PCC1/NCC1 and PCC3/NCC3 for the VDD power rail, and PCC2/NCC2
for the VDDIO2 power rail. They can be selected independently through CS1, CS2, and CS3
bits in SYSCFG_CCCSR (see Figure 50).
To reduce the power consumption, it is recommended to copy the code from
SYSCFG_CCVR to SYSCFG_CCCR. After the result is ready, set the CSx bit and disable
the compensation cell.

Note:

<!-- pagebreak -->

