2526

Low-power timer (LPTIM)

RM0456

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 REPOKIE: Repetition register update OK interrupt Enable
0: Repetition register update OK interrupt disabled
1: Repetition register update OK interrupt enabled
Bit 7 UEIE: Update event interrupt enable
0: Update event interrupt disabled
1: Update event interrupt enabled
Bit 6 DOWNIE: Direction change to down Interrupt Enable
0: DOWN interrupt disabled
1: DOWN interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 5 UPIE: Direction change to UP Interrupt Enable
0: UP interrupt disabled
1: UP interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 4 ARROKIE: Autoreload register update OK Interrupt Enable
0: ARROK interrupt disabled
1: ARROK interrupt enabled
Bit 3 CMP1OKIE: Compare register 1 update OK interrupt enable
0: CMPOK register 1 interrupt disabled
1: CMPOK register 1 interrupt enabled
Bit 2 EXTTRIGIE: External trigger valid edge Interrupt Enable
0: EXTTRIG interrupt disabled
1: EXTTRIG interrupt enabled
Bit 1 ARRMIE: Autoreload match Interrupt Enable
0: ARRM interrupt disabled
1: ARRM interrupt enabled
Bit 0 CC1IE: Capture/compare 1 interrupt enable
0: Capture/compare 1 interrupt disabled
1: Capture/compare 1 interrupt enabled

Caution:

<!-- pagebreak -->

The LPTIMx_DIER register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1). After a write to the LPTIMx_DIER register, a new write operation to the same
register can only be performed when the previous write operation is completed. Any
successive write before the DIEROK flag is set, leads to unpredictable results.

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

58.7.8

LPTIMx interrupt enable register [alternate] (LPTIMx_DIER)
(x = 1 to 3)
This description of the register can only be used for output compare mode. See next section
for input capture compare mode.
Address offset: 0x008
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

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

Res.

Res.

Res.

Res.

Res.

Res.

CC2IE
rw

23
UEDE

22

21

20

19

18

17

16

CMP2
OKIE

Res.

Res.

Res.

2

1

0

Res.

Res.

Res.

7

6

5

4

3

REPOK
IE

UEIE

DOWNI
E

UPIE

ARRO
KIE

CMP1
OKIE

rw

rw

rw

rw

rw

rw

rw

rw

EXT
ARRM
TRIGIE
IE
rw

rw

CC1IE
rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 UEDE: Update event DMA request enable
0: UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal.
1: UE DMA request enabled
Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section 58.3.
Bit 22 Reserved, must be kept at reset value.
Bit 21 Reserved, must be kept at reset value.
Bit 20 Reserved, must be kept at reset value.
Bit 19 CMP2OKIE: Compare register 2 update OK interrupt enable
0: CMPOK register 2 interrupt disabled
1: CMPOK register 2 interrupt enabled
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bits 18:12 Reserved, must be kept at reset value.
Bit 11 Reserved, must be kept at reset value.
Bit 10 Reserved, must be kept at reset value.
Bit 9 CC2IE: Capture/compare 2 interrupt enable
0: Capture/compare 2 interrupt disabled
1: Capture/compare 2 interrupt enabled
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bit 8 REPOKIE: Repetition register update OK interrupt Enable
0: Repetition register update OK interrupt disabled
1: Repetition register update OK interrupt enabled
Bit 7 UEIE: Update event interrupt enable
0: Update event interrupt disabled
1: Update event interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

