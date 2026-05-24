RM0456 Rev 6

rw

rw

4

3

2

1

0

CCI

BB

Res.

Res.

Res.

rw

rw

RM0456

Debug support (DBG)

Bits 10:5 COND[5:0]: conditional instruction tracing
0x0: conditional instruction tracing disabled
0x1: conditional load instructions traced
0x2: conditional store instructions traced
0x3: conditional load and store instructions traced
0x7: All conditional instructions traced
Bit 4 CCI: cycle counting in instruction trace
0: disabled
1: enabled
Bit 3 BB: branch broadcast mode
0: disabled
1: enabled
Bits 2:0 Reserved, must be kept at reset value.

ETM event control 0 register (ETM_EVENTCTL0R)
Address offset: 0x020
Reset value: 0xXXXX XXXX
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

11

10

9

8

3

2

1

0

15

14

13

12

TYPE1

Res.

Res.

Res.

rw

SEL1[3:0]
rw

rw

rw

7

6

5

4

TYPE0

Res.

Res.

Res.

rw

rw

SEL0[3:0]
rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 TYPE1: resource type for event1
0: single selected resource
1: boolean combined resource pair
Bits 14:12 Reserved, must be kept at reset value.
Bits 11:8 SEL1[3:0]: resource number based on TYPE1
Selects the resource number, based on the value of TYPE1.
When TYPE1 = 0, a single resource from 0-15 defined by SEL1[3:0] is selected.
When TYPE1 = 1, a boolean combined resource pair defined by SEL1[2:0] is selected.
Bit 7 TYPE0: resource type for event0
0: single selected resource
1: boolean combined resource pair
Bits 6:4 Reserved, must be kept at reset value.
Bits 3:0 SEL0[3:0]: resource number based on TYPE0
Selects the resource number, based on the value of TYPE0.
When TYPE0 = 0, a single resource from 0-15 defined by SEL0[3:0] is selected.
When TYPE0 = 1, a boolean combined resource pair defined by SEL0[2:0] is selected.

RM0456 Rev 6

<!-- pagebreak -->

