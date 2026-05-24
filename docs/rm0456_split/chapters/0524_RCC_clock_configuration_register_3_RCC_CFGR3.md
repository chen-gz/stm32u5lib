609

Reset and clock control (RCC)

RM0456

Bits 6:4 PPRE1[2:0]: APB1 prescaler
This bitfield is set and cleared by software to control the division factor of APB1 clock
(PCLK1).
0xx: PCLK1 not divided
100: PCLK1 divided by 2
101: PCLK1 divided by 4
110: PCLK1 divided by 8
111: PCLK1 divided by 16
Bits 3:0 HPRE[3:0]: AHB prescaler
This bitfield is set and cleared by software to control the division factor of the AHB clock
(HCLK).
Caution: Depending on the device voltage range, the software must set these bits correctly to
ensure that the system frequency does not exceed the maximum allowed frequency
(for more details, refer to Table 114). After a write operation to these bits and before
decreasing the voltage range, this register must be read to be sure that the new
value is taken into account.
0xxx: SYSCLK not divided
1000: SYSCLK divided by 2
1001: SYSCLK divided by 4
1010: SYSCLK divided by 8
1011: SYSCLK divided by 16
1100: SYSCLK divided by 64
1101: SYSCLK divided by 128
1110: SYSCLK divided by 256
1111: SYSCLK divided by 512

11.8.8

RCC clock configuration register 3 (RCC_CFGR3)
Address offset: 0x024
Reset value: 0x0000 0000
Access: word, half-word, and byte access
From 0 to 15 wait states are inserted if the access occurs when the APB or AHB prescalers
values update is on going.

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

APB3D AHB3D
IS
IS

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

PPRE3[2:0]
rw

Bits 31:18 Reserved, must be kept at reset value.

<!-- pagebreak -->

