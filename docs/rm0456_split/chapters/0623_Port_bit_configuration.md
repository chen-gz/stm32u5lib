•

Input floating

•

Input pull-up

•

Input-pull-down

•

Analog

•

Output open-drain with pull-up or pull-down capability

•

Output push-pull with pull-up or pull-down capability

•

Alternate function push-pull with pull-up or pull-down capability

•

Alternate function open-drain with pull-up or pull-down capability

RM0456 Rev 6

RM0456

General-purpose I/Os (GPIO)
Each I/O port bit is freely programmable, however the I/O port registers must be accessed
as 32-bit words, half-words, or bytes. The GPIOx_BSRR and GPIOx_BRR registers allow
atomic read/modify accesses to any of the GPIOx_ODR registers. In this way, there is no
risk of an IRQ occurring between the read and the modify access.
The figure below shows the basic structure of a three-volt or five-volt tolerant GPIO
(TT or FT). The Table 127 gives the possible port bit configurations.
Figure 45. Structure of three-volt or five-volt tolerant GPIO (TT or FT)

Analog

VDDA
Analog
option

Parasitic diode
and resistor

Analog IP
Analog switch

VDD

on/off

Alternate function input

RPU
Input data
register

I/O pin
Input buffer

Output data
register

RPD

on/off

VDD

ESD
protection

PMOS
Output
control
NMOS

Alternate function output

Ouput buffer

Digital

VSS

VSS

VSS

VSS
MSv46873V1

Note:

On a TT GPIO, the analog switch is not present and replaced by a direct connection. The
analog bloc parasitic circuitry does not allow five-volt tolerance.
Table 127. Port bit configuration(1)

MODE(i)[1:0]

01

OTYPE(i)

OSPEED(i)[1:0]

PUPD(i)[1:0]

I/O configuration

0

0

0

GP output

PP

0

0

1

GP output

PP + PU

0

1

0

GP output

PP + PD

1

1

0

0

GP output

OD

1

0

1

GP output

OD + PU

1

1

0

GP output

OD + PD

1

1

1

0
1

SPEED[1:0]

RM0456 Rev 6

Reserved

Reserved (GP output OD)

<!-- pagebreak -->

