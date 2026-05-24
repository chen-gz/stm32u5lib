RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

15.3.9

SYSCFG compensation cell value register (SYSCFG_CCVR)
Address offset: 0x20
Reset value: 0x0000 0000
When the system is secure (TZEN = 1), this register can be protected against nonsecure
access by setting the SYSCFGSEC bit in the SYSCFG_SECCFGR register. When the
SYSCFGSEC bit is set, only secure access is allowed: nonsecure read/write access is
RAZ/WI and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.

31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

PCV2[3:0]
r

r

r

23

r

21

20

19

PCV3[3:0]

NCV2[3:0]
r

22

r

17

16

NCV3[3:0]

PCV1[3:0]
r

18

NCV1[3:0]
r

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:20 PCV3[3:0]: PMOS compensation value of the HSPI I/Os supplied by VDD
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for PMOS transistors. This code is applied to the I/O compensation
cell when CS3 = 0 in SYSCFG_CCCSR.
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 19:16 NCV3[3:0]: NMOS compensation value of the HSPI I/Os supplied by VDD
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for NMOS transistors. This code is applied to the I/O compensation
cell when CS3 = 0 in SYSCFG_CCCSR.
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 15:12 PCV2[3:0]: PMOS compensation value of the I/Os supplied by VDDIO2
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for PMOS transistors. This code is applied to the I/O compensation
cell when CS2 = 0 in SYSCFG_CCCSR.
Bits 11:8 NCV2[3:0]: NMOS compensation value of the I/Os supplied by VDDIO2
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for NMOS transistors. This code is applied to the I/O compensation
cell when CS2 = 0 in SYSCFG_CCCSR.

RM0456 Rev 6

<!-- pagebreak -->

667

System configuration controller (SYSCFG)

RM0456

Bits 7:4 PCV1[3:0]: PMOS compensation value of the I/Os supplied by VDD
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for PMOS transistors. This code is applied to the I/O compensation
cell when CS1 = 0 in SYSCFG_CCCSR.
Bits 3:0 NCV1[3:0]: NMOS compensation value of the I/Os supplied by VDD
This value is provided by the cell and can be used by the CPU to compute an I/O
compensation cell code for NMOS transistors. This code is applied to the I/O compensation
cell when CS1 = 0 in SYSCFG_CCCSR

15.3.10

SYSCFG compensation cell code register (SYSCFG_CCCR)
Address offset: 0x24
Reset value: 0x0000 7878 (for STM32U535/545/575/585 devices)
Reset value: 0x0078 7878 (for STM32U59x/5Ax/5Fx/5Gx devices)
When the system is secure (TZEN = 1), this register can be protected against nonsecure
access by setting the SYSCFGSEC bit in the SYSCFG_SECCFGR register. When
SYSCFGSEC bit is set, only secure access is allowed: nonsecure read/write access is
RAZ/WI and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.

31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

PCC2[3:0]
rw

23

21

20

19

PCC3[3:0]

NCC2[3:0]
rw

22

17

16

NCC3[3:0]

PCC1[3:0]
rw

18

NCC1[3:0]
rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:20 PCC3[3:0]: PMOS compensation code of the HSPI I/Os supplied by VDD
These bits are written by software to define an I/O compensation cell code for PMOS
transistors. This code is applied to the I/O compensation cell when CS3 = 1
in SYSCFG_CCCSR.
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 19:16 NCC3[3:0]: NMOS compensation code of the HSPI I/Os supplied by VDD
These bits are written by software to define an I/O compensation cell code for NMOS
transistors. This code is applied to the I/O compensation cell when CS3 = 1
in SYSCFG_CCCSR.
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

Bits 15:12 PCC2[3:0]: PMOS compensation code of the I/Os supplied by VDDIO2
These bits are written by software to define an I/O compensation cell code for PMOS
transistors. This code is applied to the I/O compensation cell when CS2 = 1
in SYSCFG_CCCSR.
Bits 11:8 NCC2[3:0]: NMOS compensation code of the I/Os supplied by VDDIO2
These bits are written by software to define an I/O compensation cell code for NMOS
transistors. This code is applied to the I/O compensation cell when CS2 = 1
in SYSCFG_CCCSR.
Bits 7:4 PCC1[3:0]: PMOS compensation code of the I/Os supplied by VDD
These bits are written by software to define an I/O compensation cell code for PMOS
transistors. This code is applied to the I/O compensation cell when CS1 = 1
in SYSCFG_CCCSR.
Bits 3:0 NCC1[3:0]: NMOS compensation code of the I/Os supplied by VDD
These bits are written by software to define an I/O compensation cell code for NMOS
transistors. This code is applied to the I/O compensation cell when CS1 = 1
in SYSCFG_CCCSR.

15.3.11

SYSCFG RSS command register (SYSCFG_RSSCMDR)
Address offset: 0x2C
Power-on reset value: 0x0000 0000
System reset: not affected
When the system is secure (TZEN = 1), this register can be read and written only when the
APB access is secure. Otherwise it is RAZ/WI.
When the system is not secure (TZEN = 0), this register is RAZ/WI. This register can be
read and written by privileged and unprivileged access.

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

RSSCMD[15:0]
rw

rw

ì

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 RSSCMD[15:0]: RSS commands
This field defines a command to be executed by the RSS.

RM0456 Rev 6

<!-- pagebreak -->

667

System configuration controller (SYSCFG)

15.3.12

RM0456

SYSCFG OTG_HS PHY register (SYSCFG_OTGHSPHYCR)
Address offset: 0x74
Reset value: 0x0000 0000
This register can be protected against nonsecure access by setting OTGSEC
in GTZC1_TZSC_SECCFGR3. When OTGSEC is set, only secure access is allowed:
non-secure read/write access is RAZ/WI, and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.
This register is only available on some devices of the STM32U5 Series. Refer to the device
datasheet for the availability of the associated peripheral.

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

CLKSEL[3:0]
rw

rw

rw

rw

1

0

PDCTR
L

EN

rw

rw

Bits 31:6 Reserved, must be kept at reset value.
Bits 5:2 CLKSEL[3:0]: OTG_HS PHY reference clock frequency selection
0011: 16 MHz
1000: 19.2 MHz
1001: 20 MHz
1010: 24 MHz
1110: 26 MHz
1011: 32 MHz
Others: reserved
Bit 1 PDCTRL: OTG_HS PHY common block power-down control
0: In SUSPEND, PHY state machine, bias, and OTG PHY PLL remain powered.
1: In SUSPEND, PHY state machine, bias, and OTG PHY PLL are powered down.
Bit 0 EN: OTG_HS PHY enable
0: PHY under reset
1: PHY enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

15.3.13

SYSCFG OTG_HS PHY tune register 2
(SYSCFG_OTGHSPHYTUNER2)
Address offset: 0x7C
Reset value: 0x81CD 06B1 (for STM32U59x/5Ax)
Reset value: 0x81CD 06B2 (for STM32U5Fx/5Gx)
This register can be protected against nonsecure access by setting OTGSEC
in GTZC1_TZSC_SECCFGR3. When OTGSEC is set, only secure access is allowed:
non-secure read/write access is RAZ/WI, and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.
This register in only available on STM32U59x/5Ax/5Fx/5Gx devices.

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

15
Res.

TXPREEMPAMP
TUNE[1:0]
rw

rw

SQRXTUNE[2:0]
rw

rw

rw

Res.

COMPDISTUNE[2:0]
rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:13 TXPREEMPAMPTUNE[1:0]: High-speed (HS) transmitter preemphasis current control
11: HS transmitter preemphasis circuit sources 3x preemphasis current
10: HS transmitter preemphasis circuit sources 2x preemphasis current
01: HS transmitter preemphasis circuit sources 1x preemphasis current
00: HS transmitter preemphasis circuit disabled
Bits 12:7 Reserved, must be kept at reset value.
Bits 6:4 SQRXTUNE[2:0]: Squelch threshold adjustment
This bitfield adjusts the voltage level for the threshold used to detect valid high-speed data.
000: +15% (recommended value)
011: 0% (default value)
others: reserved
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 COMPDISTUNE[2:0]: Disconnect threshold adjustment
This bitfield adjusts the voltage level for the threshold used to detect a disconnect event
at the host.
010: +5.9% (recommended value)
001: 0% (default value)
others: reserved

RM0456 Rev 6

<!-- pagebreak -->

