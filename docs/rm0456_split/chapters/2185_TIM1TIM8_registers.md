RM0456 Rev 6

BIE

IERRIE

RM0456

Advanced-control timers (TIM1/TIM8)

54.6

TIM1/TIM8 registers
Refer to Section 1.2 for a list of abbreviations used in register descriptions.
The peripheral registers can be accessed by half-words (16-bit) or words (32-bit).

54.6.1

TIMx control register 1 (TIMx_CR1)(x = 1, 8)
Address offset: 0x000
Reset value: 0x0000

15

14

13

12

11

10

9

Res.

Res.

Res.

DITH
EN

UIFRE
MAP

Res.

CKD[1:0]

rw

rw

rw

8

7
ARPE

rw

rw

6

5

4

3

2

1

0

CMS[1:0]

DIR

OPM

URS

UDIS

CEN

rw

rw

rw

rw

rw

rw

rw

Bits 15:13 Reserved, must be kept at reset value.
Bit 12 DITHEN: Dithering enable
0: Dithering disabled
1: Dithering enabled
Note: The DITHEN bit can only be modified when CEN bit is reset.
Bit 11 UIFREMAP: UIF status bit remapping
0: No remapping. UIF status bit is not copied to TIMx_CNT register bit 31.
1: Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31.
Bit 10 Reserved, must be kept at reset value.
Bits 9:8 CKD[1:0]: Clock division
This bitfield indicates the division ratio between the timer clock (tim_ker_ck) frequency and
the dead-time and sampling clock (tDTS)used by the dead-time generators and the digital
filters (tim_etr_in, tim_tix),
00: tDTS = ttim_ker_ck
01: tDTS = 2*ttim_ker_ck
10: tDTS = 4*ttim_ker_ck
11: Reserved, do not program this value
Bit 7 ARPE: Autoreload preload enable
0: TIMx_ARR register is not buffered
1: TIMx_ARR register is buffered
Bits 6:5 CMS[1:0]: Center-aligned mode selection
00: Edge-aligned mode. The counter counts up or down depending on the direction bit
(DIR).
01: Center-aligned mode 1. The counter counts up and down alternatively. Output compare
interrupt flags of channels configured in output (CCxS = 00 in TIMx_CCMRx register) are
set only when the counter is counting down.
10: Center-aligned mode 2. The counter counts up and down alternatively. Output compare
interrupt flags of channels configured in output (CCxS = 00 in TIMx_CCMRx register) are
set only when the counter is counting up.
11: Center-aligned mode 3. The counter counts up and down alternatively. Output compare
interrupt flags of channels configured in output (CCxS = 00 in TIMx_CCMRx register) are
set both when the counter is counting up or down.
Note: It is not allowed to switch from edge-aligned mode to center-aligned mode as long as
the counter is enabled (CEN = 1)

RM0456 Rev 6

<!-- pagebreak -->

