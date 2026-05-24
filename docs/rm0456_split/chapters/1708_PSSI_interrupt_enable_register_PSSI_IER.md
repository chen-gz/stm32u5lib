1710

Parallel synchronous slave interface (PSSI)

42.5.4

RM0456

PSSI interrupt enable register (PSSI_IER)
Address offset: 0x0C
Reset value: 0x0000 0000
The PSSI_IER register is used to enable interrupts. When one of the PSSI_IER bits is set,
the corresponding interrupt is enabled. This register is accessible both in read and write
modes.

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

OVR_I
E

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

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 OVR_IE: Data buffer overrun/underrun interrupt enable
0: No interrupt generation
1: An interrupt is generated if either an overrun or an underrun error occurred.
Bit 0 Reserved, must be kept at reset value.

42.5.5

PSSI masked interrupt status register (PSSI_MIS)
This PSSI_MIS register is read-only. When read, it returns the current masked status value
of the corresponding interrupt (depending on the value in PSSI_IER). A bit in this register is
set if the corresponding enable bit in PSSI_IER is set and the corresponding bit in
PSSI_RIS is set.
Address offset: 0x10
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

Res.

OVR_
MIS

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

r

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 OVR_MIS: Data buffer overrun/underrun masked interrupt status
This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1.
0: No interrupt is generated when an overrun/underrun error occurs
1: An interrupt is generated if there is either an overrun or an underrun error and the
OVR_IE bit is set in PSSI_IER.
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

