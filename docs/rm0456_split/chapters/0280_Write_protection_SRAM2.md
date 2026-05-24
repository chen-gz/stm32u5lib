290

RAM configuration controller (RAMCFG)

RM0456

The procedure to check ECC is the following:
1.

On an erased memory, write data with ECC on.

2.

Disable ECC.

3.

Write same data with 1- or 2-bit modification (for single or double error test
respectively).

4.

Enable ECC.

5.

Wait until ECCE bit of RAMCFG_M3CR is read at 1.

6.

Read data. Enabled interrupt is generated because of single or double error.

Steps 4 and 5 instructions must not be located in SRAM3 with ECC area. Any access to
SRAM3 with ECC area by the other masters is forbidden during the steps 4 and 5
executions, until they are completed.

Warning:

6.3.3

The ECC fault injection test triggers a system break event
in TIM1/8/15/16/17, if the SPL bit is set in SYSCFG_CFGR2.
This implies that the test must be performed while the PWM
outputs of the timers are in idle state.

Write protection (SRAM2)
The SRAM2 is made of 64 1-Kbyte pages. Each 1-Kbyte page can be write-protected
by setting its corresponding PxWP (x = 0 to 63) bit in RAMCFG_M2WPR1
and RAMCFG_M2WPR2.

6.3.4

Read access latency
To correctly read data from SRAMs, the number of wait states must be correctly
programmed in WSC[2:0] field of RAMCFG_MxCR, depending on AHB clock frequency
(HCLK) and voltage scaling range, as shown in the table below.
Table 47. Number of wait states versus HCLK frequency and voltage range scaling
HCLK (MHz)

Wait states (WS) (latency)
VCORE range 1

VCORE range 2

VCORE range 3

VCORE range 4 and
Stop 0/1/2 modes(1)

0 WS (1 AHB cycle)

≤ 160

≤ 110

≤ 55

≤ 16

1 WS (2 AHB cycle)

-

-

-

≤ 25

1. The system clock can be requested in Stop 0/1/2 modes to perform DMA transfers to SRAM.

6.3.5

Software erase
SRAM erase can be requested by executing this software sequence:

<!-- pagebreak -->

