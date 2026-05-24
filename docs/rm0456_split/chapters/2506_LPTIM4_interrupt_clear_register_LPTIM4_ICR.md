16

rw

Bits 31:24 Reserved, must be kept at reset value.

Note:

17

YU[3:0]

The calendar is frozen when reaching the maximum value, and can’t roll over.

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6.3

RTC subsecond register (RTC_SSR)
Address offset: 0x08
Backup domain reset value: 0x0000 0000
System reset value: 0x0000 0000 (when BYPSHAD = 0, not affected when BYPSHAD = 1)

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

SS[31:16]
r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

r

r

SS[15:0]
r

Bits 31:0 SS[31:0]: Synchronous binary counter
SS[31:16]: Synchronous binary counter MSB values
When Binary or Mixed mode is selected (BIN = 01 or 10 or 11):
SS[31:16] are the 16 MSB of the SS[31:0] free-running down-counter.
When BCD mode is selected (BIN=00):
SS[31:16] are forced by hardware to 0x0000.
SS[15:0]: Subsecond value/synchronous binary counter LSB values
When Binary mode is selected (BIN = 01 or 10 or 11):
SS[15:0] are the 16 LSB of the SS[31:0] free-running down-counter.
When BCD mode is selected (BIN=00):
SS[15:0] is the value in the synchronous prescaler counter. The fraction of a second is given
by the formula below:
Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1)
SS can be larger than PREDIV_S only after a shift operation. In that case, the correct
time/date is one second less than as indicated by RTC_TR/RTC_DR.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

63.6.4

RM0456

RTC initialization control and status register (RTC_ICSR)
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be globally protected, or each bit of this register can be individually
protected against nonsecure access. Refer to Section 63.3.4: RTC secure protection
modes.
This register can be globally protected, or each bit of this register can be individually
protected against unprivileged access. Refer to Section 63.3.5: RTC privilege protection
modes.
Address offset: 0x0C
Backup domain reset value: 0x0000 0007
System reset: not affected except INIT, INITF, and RSF bits which are cleared to 0

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

RECAL
PF

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

BIN[1:0]

INIT

INITF

RSF

INITS

SHPF

WUTW
F

Res.

Res.

rw

r

rc_w0

r

r

r

r

BCDU[2:0]
rw

rw

rw

rw

rw

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 RECALPF: Recalibration pending Flag
The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR
register, indicating that the RTC_CALR register is blocked. When the new calibration settings
are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly.
Bits 15:13 Reserved, must be kept at reset value.
Bits 12:10 BCDU[2:0]: BCD update (BIN = 10 or 11)
In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or
11), the calendar second is incremented using the SSR Least Significant Bits.
0x0: 1s calendar increment is generated each time SS[7:0] = 0
0x1: 1s calendar increment is generated each time SS[8:0] = 0
0x2: 1s calendar increment is generated each time SS[9:0] = 0
0x3: 1s calendar increment is generated each time SS[10:0] = 0
0x4: 1s calendar increment is generated each time SS[11:0] = 0
0x5: 1s calendar increment is generated each time SS[12:0] = 0
0x6: 1s calendar increment is generated each time SS[13:0] = 0
0x7: 1s calendar increment is generated each time SS[14:0] = 0
Bits 9:8 BIN[1:0]: Binary mode
00: Free running BCD calendar mode (Binary mode disabled).
01: Free running Binary mode (BCD mode disabled)
10: Free running BCD calendar and Binary modes
11: Free running BCD calendar and Binary modes

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

Bit 7 INIT: Initialization mode
0: Free running mode
1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and
prescaler register (RTC_PRER), plus BIN and BCDU fields. Counters are stopped and start
counting from the new value when INIT is reset.
Bit 6 INITF: Initialization flag
When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler
registers can be updated.
0: Calendar registers update is not allowed
1: Calendar registers update is allowed
Bit 5 RSF: Registers synchronization flag
This bit is set by hardware each time the calendar registers are copied into the shadow
registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization
mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register
mode (BYPSHAD = 1). This bit can also be cleared by software.
It is cleared either by software or by hardware in initialization mode.
0: Calendar shadow registers not yet synchronized
1: Calendar shadow registers synchronized
Bit 4 INITS: Initialization status flag
This bit is set by hardware when the calendar year field is different from 0 (Backup domain
reset state).
0: Calendar has not been initialized
1: Calendar has been initialized
Bit 3 SHPF: Shift operation pending
This flag is set by hardware as soon as a shift operation is initiated by a write to the
RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has
been executed. Writing to the SHPF bit has no effect.
0: No shift operation is pending
1: A shift operation is pending
Bit 2 WUTWF: Wake-up timer write flag
This bit is set by hardware when WUT value can be changed, after the WUTE bit has been
set to 0 in RTC_CR.
It is cleared by hardware in initialization mode.
0: Wake-up timer configuration update not allowed except in initialization mode
1: Wake-up timer configuration update allowed
Bits 1:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

