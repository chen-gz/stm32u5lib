275

Global TrustZone controller (GTZC)

5.9

RM0456

GTZC2 TZSC registers
All registers are accessed only by words (32-bit).

5.9.1

GTZC2 TZSC control register (GTZC2_TZSC_CR)
Address offset: 0x000
Reset value: 0x0000 0000
Secure privilege access only.

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

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LCK
rs

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 LCK: lock the configuration of GTZC2_TZSC_SECCFGRx and GTZC2_TZSC_PRIVCFGRx
registers until next reset
This bit is cleared by default and once set, it can not be reset until system reset.
0: configuration of all GTZC2_TZSC_SECCCFGRx and all GTZC2_TZSC_PRIVCFGRx
registers not locked
1: configuration of all GTZC2_TZSC_SECCCFGRx and all GTZC2_TZSC_PRIVCFGRx
registers locked

5.9.2

GTZC2 TZSC secure configuration register 1
(GTZC2_TZSC_SECCFGR1)
Address offset: 0x010
Reset value: 0x0000 0000
Write-secure access only.
This register can be written only by secure privileged transaction when corresponding
GTZC2_TZSC_PRIVCFGR register signal is set to 1. If a given PRIV bit is not set, the
equivalent SEC bit can be written by secure unprivileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

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

Res.

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

ADF1S DAC1S
EC
EC
rw

rw

Res.

VREFB ADC4S COMP OPAMP LPTIM4 LPTIM3 LPTIM1 I2C3SE LPUAR SPI3SE
UFSEC
EC
SEC
SEC
SEC
SEC
SEC
C
T1SEC
C
rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

rw

rw

rw

rw

rw

RM0456

Global TrustZone controller (GTZC)

Bit 12 ADF1SEC: secure access mode for ADF1
0: nonsecure
1: secure
Bit 11 DAC1SEC: secure access mode for DAC1
0: nonsecure
1: secure
Bit 10 Reserved, must be kept at reset value.
Bit 9 VREFBUFSEC: secure access mode for VREFBUF
0: nonsecure
1: secure
Bit 8 ADC4SEC: secure access mode for ADC4
0: nonsecure
1: secure
Bit 7 COMPSEC: secure access mode for COMP
0: nonsecure
1: secure
Bit 6 OPAMPSEC: secure access mode for OPAMP
0: nonsecure
1: secure
Bit 5 LPTIM4SEC: secure access mode for LPTIM4
0: nonsecure
1: secure
Bit 4 LPTIM3SEC: secure access mode for LPTIM3
0: nonsecure
1: secure
Bit 3 LPTIM1SEC: secure access mode for LPTIM1
0: nonsecure
1: secure
Bit 2 I2C3SEC: secure access mode for I2C3
0: nonsecure
1: secure
Bit 1 LPUART1SEC: secure access mode for LPUART1
0: nonsecure
1: secure
Bit 0 SPI3SEC: secure access mode for SPI3
0: nonsecure
1: secure

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

5.9.3

RM0456

GTZC2 TZSC privilege configuration register 1
(GTZC2_TZSC_PRIVCFGR1)
Address offset: 0x020
Reset value: 0x0000 0000
Write-privileged access only.
This register can be read or written only by secure privilege transaction when corresponding
GTZC2_TZSC_SECCFGR register signal is set to1. If a given SEC bit is not set, the
equivalent PRIV bit can be read/written by nonsecure privileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privilege or not.

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

Res.

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

ADF1P DAC1P
RIV
RIV
rw

rw

Res.

VREFB
ADC4P COMP OPAMP LPTIM4 LPTIM3 LPTIM1 I2C3PR LPUAR SPI3P
UFPRI
RIV
PRIV
PRIV
PRIV
PRIV
PRIV
IV
T1PRIV
RIV
V
rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 ADF1PRIV: privileged access mode for ADF1
0: unprivileged
1: privileged
Bit 11 DAC1PRIV: privileged access mode for DAC1
0: unprivileged
1: privileged
Bit 10 Reserved, must be kept at reset value.
Bit 9 VREFBUFPRIV: privileged access mode for VREFBUF
0: unprivileged
1: privileged
Bit 8 ADC4PRIV: privileged access mode for ADC4
0: unprivileged
1: privileged
Bit 7 COMPPRIV: privileged access mode for COMP
0: unprivileged
1: privileged
Bit 6 OPAMPPRIV: privileged access mode for OPAMP
0: unprivileged
1: privileged
Bit 5 LPTIM4PRIV: privileged access mode for LPTIM4
0: unprivileged
1: privileged
Bit 4 LPTIM3PRIV: privileged access mode for LPTIM3
0: unprivileged
1: privileged

<!-- pagebreak -->

