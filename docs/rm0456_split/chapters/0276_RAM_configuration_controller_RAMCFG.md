275

RAM configuration controller (RAMCFG)

RM0456

6

RAM configuration controller (RAMCFG)

6.1

RAMCFG introduction
The RAMCFG configures the features of the internal SRAMs (SRAM1/2/3/4/5/6
and BKPSRAM).

6.2

RAMCFG main features
The internal SRAM supports some of the features listed hereafter, configured in RAMCFG:
•

Error code correction (ECC):
–

Single error detection and correction with interrupt generation

–

Double error detection with interrupt or NMI generation

–

Status with failing address

•

Write protection (1-Kbyte granularity)

•

Programmable wait states for voltage scaling range 4

•

SRAM software erase

6.3

RAMCFG functional description

6.3.1

Internal SRAMs features
Up to seven SRAMs are embedded in the devices, each with specific features:
•

SRAM1/2/3/5/6 are the main SRAMs. The SRAM4 is in the SRAM used for peripheral
low-power background autonomous mode (LPBAM) in Stop 2 mode.
Table 45. SRAM structure

SRAM
SRAM1
SRAM2
SRAM3

STM32U535/545

STM32U575/585

192 Kbytes (3 blocks of 64 Kbytes)

STM32U5Fx/5Gx

768 Kbytes (12 blocks of 64 Kbytes)

64 Kbytes (8-Kbyte and 56-Kbyte blocks, can be retained in Standby mode)
N/A

512 Kbytes
(8 blocks of 64 Kbytes)

SRAM4

832 Kbytes (13 blocks of 64 Kbytes)

16 Kbytes

SRAM5

N/A

N/A

SRAM6

N/A

N/A

BKPSRAM

STM32U59x/5Ax

832 Kbytes (13 blocks of 64 Kbytes)
N/A

512 Kbytes
(8 blocks of 64 Kbytes)

2 Kbytes

The backup SRAM (BKPSRAM) can be retained in all low-power modes and
when VDD is off in VBAT mode (see Section 10: Power control (PWR) for more details).

<!-- pagebreak -->

