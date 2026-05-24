RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
This register is only writable if the CCE and INIT bits of the FDCAN_CCCR register are both
set. The CAN bit time can be programed in the range of 4 to 81 × tq. The CAN time quantum
can be programmed in the range of 1 to 1024 FDCAN kernel clock periods:
tq = (BRP + 1) × FDCAN clock period fdcan_ker_ck.
NTSEG1[7:0] is the sum of PROP_SEG and PHASE_SEG1. NTSEG2[6:0] is
PHASE_SEG2. Therefore, the length of the bit time is
(programmed values) × [NTSEG1[7:0] + NTSEG2[6:0] + 3] × tq or
(functional values) × [SYNC_SEG + PROP_SEG + PHASE_SEG1 + PHASE_SEG2] × tq.
The information processing time (IPT) is 0, meaning the data for the next bit is available at
the first clock edge after the sample point.

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

NSJW[6:0]
rw

rw

rw

15

14

13

rw

rw

rw

rw

rw

12

11

10

9

8

NTSEG1[7:0]
rw

rw

rw

rw

20

19

18

17

16

rw

rw

rw

rw

3

2

1

0

rw

rw

rw

NBRP[8:0]

rw

rw

rw

rw

rw

7

6

5

4

Res.
rw

rw

rw

NTSEG2[6:0]
rw

rw

rw

rw

Bits 31:25 NSJW[6:0]: Nominal (re)synchronization jump width
Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that
the used value is the one programmed incremented by one.
This bitfield is write-protected (P): write access is possible only when the CCE and INIT bits of
the FDCAN_CCCR register are both set.
Bits 24:16 NBRP[8:0]: Bit rate prescaler
Value by which the oscillator frequency is divided for generating the bit time quanta. The bit
time is built up from a multiple of this quantum. Valid values are 0 to 511. The actual
interpretation by the hardware of this value is such that one more than the value programmed
here is used.
This bitfield is write-protected (P): write access is possible only when the CCE and INIT bits of
the FDCAN_CCCR register are both set.
Bits 15:8 NTSEG1[7:0]: Nominal time segment before sample point
Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that
one more than the programmed value is used.
This bitfield is write-protected write (P): write access is possible only when the CCE and INIT
bits of the FDCAN_CCCR register are both set.
Bit 7 Reserved, must be kept at reset value.
Bits 6:0 NTSEG2[6:0]: Nominal time segment after sample point
Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that
one more than the programmed value is used.

Note:

With a CAN kernel clock of 48 MHz, the reset value of 0x0600 0A03 configures the FDCAN
for a bit rate of 3 Mbit/s.

RM0456 Rev 6

<!-- pagebreak -->

