1741

LCD-TFT display controller (LTDC)

RM0456

Bits 15:8 BCGREEN[7:0]: Background color green value
These bits configure the background green value.
Bits 7:0 BCBLUE[7:0]: Background color blue value
These bits configure the background blue value.

43.7.8

LTDC interrupt enable register (LTDC_IER)
Address offset: 0x034
Reset value: 0x0000 0000
This register determines which status flags generate an interrupt request by setting the
corresponding bit to 1.

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

RRIE

TERRI
E

FUIE

LIE

rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RRIE: Register reload interrupt enable
This bit is set and cleared by software.
0: Register reload interrupt disable
1: Register reload interrupt enable
Bit 2 TERRIE: Transfer error interrupt enable
This bit is set and cleared by software.
0: Transfer error interrupt disable
1: Transfer error interrupt enable
Bit 1 FUIE: FIFO underrun interrupt enable
This bit is set and cleared by software.
0: FIFO underrun interrupt disable
1: FIFO underrun Interrupt enable
Bit 0 LIE: Line interrupt enable
This bit is set and cleared by software.
0: Line interrupt disable
1: Line interrupt enable

<!-- pagebreak -->

