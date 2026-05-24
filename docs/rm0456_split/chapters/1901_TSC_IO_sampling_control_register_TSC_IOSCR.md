RM0456 Rev 6

rw

RM0456

Touch sensing controller (TSC)

47.6.7

TSC I/O sampling control register (TSC_IOSCR)
Address offset: 0x20
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

G8_IO4 G8_IO3 G8_IO2 G8_IO1 G7_IO4 G7_IO3 G7_IO2 G7_IO1 G6_IO4 G6_IO3 G6_IO2 G6_IO1 G5_IO4 G5_IO3 G5_IO2 G5_IO1
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

G4_IO4 G4_IO3 G4_IO2 G4_IO1 G3_IO4 G3_IO3 G3_IO2 G3_IO1 G2_IO4 G2_IO3 G2_IO2 G2_IO1 G1_IO4 G1_IO3 G1_IO2 G1_IO1
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

rw

rw

Bits 31:0 Gx_IOy: Gx_IOy sampling mode
These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor
I/O. Only one I/O per analog I/O group must be defined as sampling capacitor.
0: Gx_IOy unused
1: Gx_IOy used as sampling capacitor
Note: These bits must not be modified when an acquisition is ongoing.
During the acquisition phase and even if the TSC peripheral alternate function is not
enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch
is automatically controlled by the touch sensing controller.

47.6.8

TSC I/O channel control register (TSC_IOCCR)
Address offset: 0x28
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

G8_IO4 G8_IO3 G8_IO2 G8_IO1 G7_IO4 G7_IO3 G7_IO2 G7_IO1 G6_IO4 G6_IO3 G6_IO2 G6_IO1 G5_IO4 G5_IO3 G5_IO2 G5_IO1
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

G4_IO4 G4_IO3 G4_IO2 G4_IO1 G3_IO4 G3_IO3 G3_IO2 G3_IO1 G2_IO4 G2_IO3 G2_IO2 G2_IO1 G1_IO4 G1_IO3 G1_IO2 G1_IO1
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

rw

rw

Bits 31:0 Gx_IOy: Gx_IOy channel mode
These bits are set and cleared by software to configure the Gx_IOy as a channel I/O.
0: Gx_IOy unused
1: Gx_IOy used as channel
Note: These bits must not be modified when an acquisition is ongoing.
During the acquisition phase and even if the TSC peripheral alternate function is not
enabled, as soon as the TSC_IOCCR bit is set, the corresponding GPIO analog switch
is automatically controlled by the touch sensing controller.

RM0456 Rev 6

<!-- pagebreak -->

