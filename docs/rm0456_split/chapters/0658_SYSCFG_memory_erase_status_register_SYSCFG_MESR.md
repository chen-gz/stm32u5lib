667

System configuration controller (SYSCFG)

15.3.7

RM0456

SYSCFG memory erase status register (SYSCFG_MESR)
Address offset: 0x18
Power-on reset value: 0x0000 0000
System reset value: 0x0000 000X (bit 0 not affected by system reset)
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

IPMEE

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

MCLR

rc_w1

rc_w1

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 IPMEE: ICACHE and PKA SRAM erase status
This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential
tamper detection (refer to Section 64: Tamper and backup registers (TAMP) for more details).
This bit is cleared by software by writing 1 to it.
0: ICACHE and PKA SRAM erase on going if not yet cleared by software
1: ICACHE and PKA SRAM erase done
Bits 15:1 Reserved, must be kept at reset value.
Bit 0 MCLR: Erase status of device memories
This bit is set by hardware when SRAM2, BKPSRAM, ICACHE, DCACHE1/2, PKA SRAM
erase is completed after power-on reset or tamper detection (refer to Section 64: Tamper and
backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared
by software by writing 1 to it.
0: memory erase on going if not yet cleared by software
1: Memory erase done

<!-- pagebreak -->

RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

15.3.8

SYSCFG compensation cell control/status register
(SYSCFG_CCCSR)
Address offset: 0x1C
Reset value: 0x0000 000A (for STM32U535/545/575/585 devices)
Reset value: 0x0000 002A (for STM32U59x/5Ax/5Fx/5Gx devices)
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

RDY3

RDY2

RDY1

Res.

Res.

CS3

EN3

CS2

EN2

CS1

EN1

r

r

r

rw

rw

rw

rw

rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 RDY3: HSPI I/O compensation cell ready flag
This bit provides the compensation cell status of the HSPI I/Os supplied by VDD.
0: HSPI I/O compensation cell not ready
1: HSPI I/O compensation cell ready
Note: The HSI clock is required for the compensation cell to work properly. The compensation
cell ready bit (RDY3) is not set if the HSI clock is not enabled. This compensation cell
acts on the GPIOs with HSPI alternate functions but independently of this mode or AF
configuration. Compensation cell 1 does not have effect on these GPIOs.
This bit is only available on some devices in the STM32U5 Series; refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 9 RDY2: VDDIO2 I/O compensation cell ready flag
This bit provides the compensation cell status of the I/Os supplied by VDDIO2.
0: VDDIO2 I/O compensation cell not ready
1: VDDIO2 I/O compensation cell ready
Note: The HSI clock is required for the compensation cell to work properly. The compensation
cell ready bit (RDY2) is not set if the HSI clock is not enabled.
Bit 8 RDY1: VDD I/Os compensation cell ready flag
This bit provides the compensation cell status of the I/Os supplied by VDD.
0: VDD I/O compensation cell not ready
1: VDD I/O compensation cell ready
Note: The HSI clock is required for the compensation cell to work properly. The compensation
cell ready bit (RDY1) is not set if the HSI clock is not enabled.
Bits 7:6 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

667

System configuration controller (SYSCFG)

RM0456

Bit 5 CS3: HSPI I/Os code selection
This bit selects the code to be applied for the compensation cell of the HSPI I/Os supplied by
VDD.
0: HSPI I/O code from the cell (available in the SYSCFG_CCVR)
1: HSPI I/O code from the SYSCFG compensation cell code register (SYSCFG_CCCR)
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bit is only available on some devices in the STM32U5 Series; refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 4 EN3: HSPI I/Os compensation cell enable
This bit enables the compensation cell of the HSPI I/Os supplied by VDD.
0: HSPI I/O compensation cell disabled
1: HSPI I/O compensation cell enabled
Note: The compensation cell acts on the GPIOs with HSPI alternate functions but
independently of this mode or AF configuration. Compensation cell 1 does not have
effect on these GPIOs.
This bit is only available on some devices in the STM32U5 Series; refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 3 CS2: VDDIO2 I/O code selection
This bit selects the code to be applied for the compensation cell of the I/Os supplied by
VDDIO2.
0: VDDIO2 I/O code from the cell (available in SYSCFG_CCVR)
1: VDDIO2 I/O code from SYSCFG_CCCR
Bit 2 EN2: VDDIO2 I/O compensation cell enable
This bit enables the compensation cell of the I/Os supplied by VDDIO2.
0: VDDIO2 I/O compensation cell disabled
1: VDDIO2 I/O compensation cell enabled
Bit 1 CS1: VDD I/O code selection
This bit selects the code to be applied for the compensation cell of the I/Os supplied by VDD.
0: VDD I/O code from the cell (available in the SYSCFG_CCVR)
1: VDD I/O code from the SYSCFG compensation cell code register (SYSCFG_CCCR)
Bit 0 EN1: VDD I/O compensation cell enable
This bit enables the compensation cell of the I/Os supplied by VDD.
0: VDD I/O compensation cell disabled
1: VDD I/O compensation cell enabled

<!-- pagebreak -->

