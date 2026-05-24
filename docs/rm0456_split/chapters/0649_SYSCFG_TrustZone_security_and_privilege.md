The compensation cell can be used only when 1.6 V ≤ VDDIOx ≤ 3.6 V.

RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)
Figure 50. I/O compensation cell block diagram
VDD I/O

VDD I/O POWER RAIL
PCV1

0

PCC1

Compensation cell

CS1
NCV1

0

NCC1

VDD

P_CODE1

1

Output
control

N_CODE1

1

VSS

VDDIO2 I/O

VDDIO2 I/O POWER RAIL
PCV2

0

PCC2

Compensation cell

CS2
NCV2

0

NCC2

VDDIO2

P_CODE2

1

Output
control

N_CODE2

1

VSS

VDD I/O

HSPI I/O POWER RAIL
PCV3

0

PCC3

Compensation cell

CS3
NCV3

0

NCC3

VDD

P_CODE3

1

Output
control

N_CODE3

1

VSS
MSv68195V1

15.2.2

SYSCFG TrustZone security and privilege
SYSCFG TrustZone security
When the TrustZone security is activated, the SYSCFG is able to secure registers from
being modified by nonsecure accesses.
The TrustZone security is activated by the TZEN option bit in the FLASH_OPTR register.
A nonsecure read/write access to a secured register is RAZ/WI and generates an illegal
access event. An illegal access interrupt is generated if the SYSCFG illegal access event is
enabled in the GTZC.

Privileged/unprivileged mode
The SYSCFG registers can be read and written by privileged and unprivileged accesses
except the SYSCFG registers for CPU configuration: SYSCFG_CSLCKR,
SYSCFG_FPUIMR and SYSCFG_CNSLCKR registers, and the FPUSEC bit in the
SYSCFG_SECCFGR.
An unprivileged access to a privileged register is RAZ/WI.

RM0456 Rev 6

<!-- pagebreak -->

