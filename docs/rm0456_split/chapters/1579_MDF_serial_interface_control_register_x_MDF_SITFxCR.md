RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

39.8.3

MDF serial interface control register x (MDF_SITFxCR)
Address offset: 0x080 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 1F00
This register is used to control the serial interfaces (SITFx). The number of registers is equal
to the amount of filters. Refer to Section 39.3 for details.

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

SITFAC
TIVE

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

r
15
Res.

Res.

Res.

STH[4:0]
rw

rw

rw

Res.
rw

rw

Res.

SITFMOD[1:0]
rw

rw

Res.

SCKSRC[1:0]
rw

rw

0
SITFE
N
rw

Bit 31 SITFACTIVE: Serial interface active flag
This bit is set and cleared by hardware. It is used by the application to check if the serial
interface is effectively enabled (active) or not. The protected fields of this function can only be
updated when the SITFACTIVE is set to 0 (refer to Section 39.4.15 for details).
The delay between a transition on SITFEN and a transition on SITFACTIVE is two periods of
AHB clock and two periods of mdf_proc_ck.
0: The serial interface is not active and can be configured if needed.
1: The serial interface is active and protected fields cannot be configured.
Bits 30:13 Reserved, must be kept at reset value.
Bits 12:8 STH[4:0]: Manchester symbol threshold/SPI threshold
This bitfield is set and cleared by software. It is used for Manchester mode to define the
expected symbol threshold levels (refer to Manchester mode for details on computation).
In addition this bitfield is used to define the timeout value for the clock absence detection in
Normal SPI mode. STH[4:0] values lower than four are invalid.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 SITFMOD[1:0]: Serial interface type
This bitfield is set and cleared by software. it is used to define the serial interface type.
00: LF_MASTER SPI mode
01: Normal SPI mode
10: Manchester mode: rising edge = logic 0, falling edge = logic 1
11: Manchester mode: rising edge = logic 1, falling edge = logic 0
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bit 3 Reserved, must be kept at reset value.
Bits 2:1 SCKSRC[1:0]: Serial clock source
This bitfield is set and cleared by software. it is used to select the clock source of the serial
interface.
00: Serial clock source is MDF_CCK0.
01: Serial clock source is MDF_CCK1.
1x: Serial clock source is MDF_CKIx (not allowed in LF_MASTER SPI mode).
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

RM0456 Rev 6

<!-- pagebreak -->

