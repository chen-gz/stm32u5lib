1904

Touch sensing controller (TSC)

RM0456

Bit 2 AM: Acquisition mode
This bit is set and cleared by software to select the acquisition mode.
0: Normal acquisition mode (acquisition starts as soon as START bit is set)
1: Synchronized acquisition mode (acquisition starts if START bit is set and when the
selected signal is detected on the SYNC input pin)
Note: This bit must not be modified when an acquisition is ongoing.
Bit 1 START: Start a new acquisition
This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the
acquisition is complete or by software to cancel the ongoing acquisition.
0: Acquisition not started
1: Start a new acquisition
Bit 0 TSCE: Touch sensing controller enable
This bit is set and cleared by software to enable/disable the touch sensing controller.
0: Touch sensing controller disabled
1: Touch sensing controller enabled
Note: When the touch sensing controller is disabled, TSC registers settings have no effect.

47.6.2

TSC interrupt enable register (TSC_IER)
Address offset: 0x04
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

MCEIE

EOAIE

rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 MCEIE: Max count error interrupt enable
This bit is set and cleared by software to enable/disable the max count error interrupt.
0: Max count error interrupt disabled
1: Max count error interrupt enabled
Bit 0 EOAIE: End of acquisition interrupt enable
This bit is set and cleared by software to enable/disable the end of acquisition interrupt.
0: End of acquisition interrupt disabled
1: End of acquisition interrupt enabled

<!-- pagebreak -->

