RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)

26.4.4

FMAC parameter register (FMAC_PARAM)
Address offset: 0x0C
Reset value: 0x0000 0000
Access: word access

31

30

29

28

START

27

26

25

24

23

22

21

20

FUNC[6:0]

19

18

17

16

R[7:0]

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

rw

rw

rw

rw

Q[7:0]
rw

rw

rw

rw

rw

P[7:0]
rw

rw

rw

rw

rw

rw

rw

Bit 31 START: Enable execution
0: Stop execution
1: Start execution
Setting this bit triggers the execution of the function selected in the FUNC bitfield. Resetting
it by software stops any ongoing function. For initialization functions, this bit is reset by
hardware.
Bits 30:24 FUNC[6:0]: Function
0: Reserved
1: Load X1 buffer
2: Load X2 buffer
3: Load Y buffer
4 to 7: Reserved
8: Convolution (FIR filter)
9: IIR filter (direct form 1)
10 to 127: Reserved
This bitfield can not be modified when a function is ongoing (START = 1)
Bits 23:16 R[7:0]: Input parameter R.
The value of this parameter is dependent on the function.
This bitfield can not be modified when a function is ongoing (START = 1)
Bits 15:8 Q[7:0]: Input parameter Q.
The value of this parameter is dependent on the function.
This bitfield can not be modified when a function is ongoing (START = 1)
Bits 7:0 P[7:0]: Input parameter P.
The value of this parameter is dependent on the function
This bitfield can not be modified when a function is ongoing (START = 1)

RM0456 Rev 6

<!-- pagebreak -->

