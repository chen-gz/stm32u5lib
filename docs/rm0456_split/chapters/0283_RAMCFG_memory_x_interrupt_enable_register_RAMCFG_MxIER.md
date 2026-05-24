RM0456 Rev 6

RM0456

RAM configuration controller (RAMCFG)

Bit 0 ECCE: ECC enable.
This bit reset value is defined by the user option bit configuration. When set, it can be cleared
by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register.
0: ECC disabled
1: ECC enabled
Note: This bit is reserved and must be kept at reset value in SRAM1/4/5/6 control registers.

6.6.2

RAMCFG memory x interrupt enable register (RAMCFG_MxIER)
Address offset: 0x004 + 0x40 * (x - 1), (x = 2, 3, 5)
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

ECCN
MI

Res.

DEIE

SEIE

rw

rw

rs

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 ECCNMI: Double error NMI
This bit is set by software and cleared only by a global RAMCFG reset.
0: NMI not generated in case of ECC double error
1: NMI generated in case of ECC double error
Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit
value.
Bit 2 Reserved, must be kept at reset value.
Bit 1 DEIE: ECC double error interrupt enable
0: Double error interrupt disabled
1: Double error interrupt enabled
Bit 0 SEIE: ECC single error interrupt enable
0: Single error interrupt disabled
1: Single error interrupt enabled

6.6.3

RAMCFG memory interrupt status register (RAMCFG_MxISR)
Address offset: 0x008 + 0x40 * (x - 1), (x = 1 to 7)
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

SRAM
BUSY

Res.

Res.

Res.

Res.

Res.

Res.

DED

SEDC

r

r

Res.

Res.

Res.

Res.

Res.

Res.

r

RM0456 Rev 6

<!-- pagebreak -->

290

RAM configuration controller (RAMCFG)

RM0456

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 SRAMBUSY: SRAM busy with erase operation
0: No erase operation on going
1: Erase operation on going
Note: Depending on the SRAM, the erase operation can be performed due to software
request, system reset if the option bit is enabled, tamper detection or readout protection
regression (see Table 46).
Bits 7:2 Reserved, must be kept at reset value.
Bit 1 DED: ECC double error detected
0: No double error
1: Double error detected
Note: This bit is reserved and must be kept at reset value in SRAM1/4/5/6 interrupt status
registers.
Bit 0 SEDC: ECC single error detected and corrected
0: No single error
1: Single error detected and corrected
Note: This bit is reserved and must be kept at reset value in SRAM1/4/5/6 interrupt status
registers.

6.6.4

RAMCFG memory x ECC single error address register
(RAMCFG_MxSEAR)
Address offset: 0x00C + 0x40 * (x - 1), (x = 2, 3, 5)
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

ESEA[31:16]
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

r

r

r

r

r

r

r

r

r

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

ESEA[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 ESEA[31:0]: ECC single error address
When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address
corresponding to the ECC single error.

<!-- pagebreak -->

