1904

Touch sensing controller (TSC)

47.6.9

RM0456

TSC I/O group control status register (TSC_IOGCSR)
Address offset: 0x30
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

G8S

G7S

G6S

G5S

G4S

G3S

G2S

G1S

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

G8E

G7E

G6E

G5E

G4E

G3E

G2E

G1E

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 GxS: Analog I/O group x status
These bits are set by hardware when the acquisition on the corresponding enabled analog
I/O group x is complete. They are cleared by hardware when a new acquisition is started.
0: Acquisition on analog I/O group x is ongoing or not started
1: Acquisition on analog I/O group x is complete
Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O
groups are not set.
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 GxE: Analog I/O group x enable
These bits are set and cleared by software to enable/disable the acquisition (counter is
counting) on the corresponding analog I/O group x.
0: Acquisition on analog I/O group x disabled
1: Acquisition on analog I/O group x enabled

47.6.10

TSC I/O group x counter register (TSC_IOGxCR)
x represents the analog I/O group number.
Address offset: 0x30 + 0x04 * x, (x = 1 to 8)
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

r

r

r

r

r

r

CNT[13:0]
r

r

r

r

r

r

r

r

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:0 CNT[13:0]: Counter value
These bits represent the number of charge transfer cycles generated on the analog I/O
group x to complete its acquisition (voltage across CS has reached the threshold).

<!-- pagebreak -->

