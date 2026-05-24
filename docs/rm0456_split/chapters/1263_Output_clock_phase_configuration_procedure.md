RM0456 Rev 6

RM0456

Delay block (DLYB)

32.4.5

Output clock phase configuration procedure
When the delay line length is configured to one input clock period, the output clock phase
can be selected between the unit delays spanning one Input clock period.
Follow the steps below to select the output clock phase:
1.

32.5

Disable the output clock and enable the access to the phase selection SEL[3:0] bits by
setting SEN bit to 1.

2.

Program SEL[3:0] with the desired output clock phase value.

3.

Enable the output clock on the selected phase by clearing SEN to 0.

DLYB registers
All registers can be accessed in word, half-word and byte access.

32.5.1

DLYB control register (DLYB_CR)
Address offset: 0x000
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

Res.

Res.

SEN

DEN

rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 SEN: Sampler length enable bit
0: Sampler length and register access to UNIT[6:0] and SEL[3:0] disabled, output clock
enabled.
1: Sampler length and register access to UNIT[6:0] and SEL[3:0] enabled, output clock
disabled.
Bit 0 DEN: Delay block enable bit
0: DLYB disabled.
1: DLYB enabled.

RM0456 Rev 6

<!-- pagebreak -->

