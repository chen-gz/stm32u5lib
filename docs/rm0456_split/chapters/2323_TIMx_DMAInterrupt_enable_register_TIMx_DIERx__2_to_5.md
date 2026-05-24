RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.4

TIMx DMA/Interrupt enable register (TIMx_DIER)(x = 2 to 5)
Address offset: 0x00C
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

TERR
IE

IERR
IE

DIRIE

IDXIE

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

TDE

Res.

UDE

Res.

TIE

Res.

CC4IE

CC3IE

CC2IE

CC1IE

UIE

rw

rw

rw

rw

rw

rw

CC4DE CC3DE CC2DE CC1DE
rw

rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 TERRIE: Transition error interrupt enable
0: Transition error interrupt disabled
1: Transition error interrupt enabled
Bit 22 IERRIE: Index error interrupt enable
0: Index error interrupt disabled
1: Index error interrupt enabled
Bit 21 DIRIE: Direction change interrupt enable
0: Direction change interrupt disabled
1: Direction change interrupt enabled
Bit 20 IDXIE: Index interrupt enable
0: Index interrupt disabled
1: Index interrupt enabled
Bits 19:15 Reserved, must be kept at reset value.
Bit 14 TDE: Trigger DMA request enable
0: Trigger DMA request disabled.
1: Trigger DMA request enabled.
Bit 13 Reserved, must be kept at reset value.
Bit 12 CC4DE: Capture/Compare 4 DMA request enable
0: CC4 DMA request disabled.
1: CC4 DMA request enabled.
Bit 11 CC3DE: Capture/Compare 3 DMA request enable
0: CC3 DMA request disabled.
1: CC3 DMA request enabled.
Bit 10 CC2DE: Capture/Compare 2 DMA request enable
0: CC2 DMA request disabled.
1: CC2 DMA request enabled.
Bit 9 CC1DE: Capture/Compare 1 DMA request enable
0: CC1 DMA request disabled.
1: CC1 DMA request enabled.
Bit 8 UDE: Update DMA request enable
0: Update DMA request disabled.
1: Update DMA request enabled.

RM0456 Rev 6

<!-- pagebreak -->

