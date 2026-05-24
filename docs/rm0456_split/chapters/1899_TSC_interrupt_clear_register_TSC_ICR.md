RM0456 Rev 6

RM0456

Touch sensing controller (TSC)

47.6.3

TSC interrupt clear register (TSC_ICR)
Address offset: 0x08
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

MCEIC EOAIC
rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 MCEIC: Max count error interrupt clear
This bit is set by software to clear the max count error flag and it is cleared by hardware
when the flag is reset. Writing a 0 has no effect.
0: No effect
1: Clears the corresponding MCEF of the TSC_ISR register
Bit 0 EOAIC: End of acquisition interrupt clear
This bit is set by software to clear the end of acquisition flag and it is cleared by hardware
when the flag is reset. Writing a 0 has no effect.
0: No effect
1: Clears the corresponding EOAF of the TSC_ISR register

47.6.4

TSC interrupt status register (TSC_ISR)
Address offset: 0x0C
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

MCEF

EOAF

r

r

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 MCEF: Max count error flag
This bit is set by hardware as soon as an analog I/O group counter reaches the max count
value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register.
0: No max count error (MCE) detected
1: Max count error (MCE) detected
Bit 0 EOAF: End of acquisition flag
This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits
of all enabled analog I/O groups are set or when a max count error is detected). It is cleared
by software writing 1 to the bit EOAIC of the TSC_ICR register.
0: Acquisition is ongoing or not started
1: Acquisition is complete

RM0456 Rev 6

<!-- pagebreak -->

