3086

FD controller area network (FDCAN)

70.4.8

RM0456

FDCAN timestamp counter configuration register (FDCAN_TSCC)
Address offset: 0x0020
Reset value: 0x0000 0000

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

19

18

17

16

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

TCP[3:0]

TSS[1:0]
rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:16 TCP[3:0]: Timestamp counter prescaler
Configures the timestamp and timeout counters time unit in multiples of CAN bit times
[1…16].
The actual interpretation by the hardware of this value is such that one more than the value
programmed here is used.
In CAN FD mode, the internal timestamp counter TCP does not provide a constant time base
due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD
requires an external counter for timestamp generation (TSS[1:0] = 10).
This bitfield is write-protected (P): write access is possible only when the CCE and INIT bits
of the FDCAN_CCCR register are both set.
Bits 15:2 Reserved, must be kept at reset value.
Bits 1:0 TSS[1:0]: Timestamp select
00: Timestamp counter value always 0x0000
01: Timestamp counter value incremented according to TCP
10: External timestamp counter from TIM3 value (tim3_cnt[0:15])
11: Same as 00.
These bits are write-protected write (P): write access is possible only when the CCE and INIT
bits of the FDCAN_CCCR register are both set.

70.4.9

FDCAN timestamp counter value register (FDCAN_TSCV)
Address offset: 0x0024
Reset value: 0x0000 0000

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

rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

TSC[15:0]
rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

rc_w

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

