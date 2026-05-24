RM0456 Rev 6

rw

rw

rw

rw

rw

RM0456

Global TrustZone controller (GTZC)

Bit 3 LPTIM1PRIV: privileged access mode for LPTIM1
0: unprivileged
1: privileged
Bit 2 I2C3PRIV: privileged access mode for I2C3
0: unprivileged
1: privileged
Bit 1 LPUART1PRIV: privileged access mode for LPUART1
0: unprivileged
1: privileged
Bit 0 SPI3PRIV: privileged access mode for SPI3
0: unprivileged
1: privileged

5.9.4

GTZC2 TZSC register map

Reset value

VREFBUFSEC

ADC4SEC

COMPSEC

OPAMPSEC

LPTIM4SEC

LPTIM3SEC

LPTIM1SEC

I2C3SEC

LPUART1SEC

SPI3SEC

0

0

0

0

0

0

0

0

0

ADC4PRIV

COMPPRIV

OPAMPPRIV

LPTIM4PRIV

LPTIM3PRIV

LPTIM1PRIV

I2C3PRIV

LPUART1PRIV

SPI3PRIV

0

0

VREFBUFPRIV

0

Res.

DAC1SEC
0

DAC1PRIV

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

GTZC2_TZSC_
PRIVCFGR1

0

0

0

0

0

0

0

0

0

0

0

Reserved

Res.

0x020

Reserved

ADF1SEC

Reset value
0x0140x01C

ADF1PRIV

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

GTZC2_TZSC_
SECCFGR1

Reserved

Res.

0x010

0

Reserved

Res.

0x0040x00C

LCK

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

Res.

GTZC2_TZSC_CR

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

Register name

Res.

Offset

Res.

Table 42. GTZC2 TZSC register map and reset values

Refer to Table 30: GTZC2 subblocks address offset.

RM0456 Rev 6

<!-- pagebreak -->

