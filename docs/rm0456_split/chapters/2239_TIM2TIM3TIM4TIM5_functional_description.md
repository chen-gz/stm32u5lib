Timer instance

TIM2

TIM3

TIM4

TIM5

Resolution

32-bit

32-bit

32-bit

32-bit

OCREF clear
selection
Sources

Yes

Yes

Yes

Yes

tim_etrf
tim_ocref_clr[7:0]

tim_etrf
tim_ocref_clr[7:0]

tim_etrf
tim_ocref_clr[7:0]

tim_etrf
tim_ocref_clr[7:0]

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.4

TIM2/TIM3/TIM4/TIM5 functional description

55.4.1

Block diagram
Figure 599. General-purpose timer block diagram

tim_ker_ck
tim_pclk

Trigger
controller

tim_etrf
TIM_ETR
tim_etr[15:1]

tim_etr0
tim_etr_in

Polarity selection & edge tim_etrp
Input filter
detector & prescaler

tim_trgo

tim_itr

tim_itr[15:0]

TRG
tim_trc

32-bit APB
bus

tim_trgi

tim_ti1f_ed

tim__it

IRQ interface

tim_cc1_dma
tim_cc2_dma
tim_cc3_dma
tim_cc4_dma
tim_upd_dma
tim_trgi_dma

DMA interface

U
Auto-reload register

PSC
tim_cnt_ck
prescaler
U
tim_ic1
CC1I

tim_psc_ck

tim_ti1_in0

tim_ti1_in[15:1]
TIM_CH2

tim_ti2_in0

tim_ti2_in[15:1]
TIM_CH3

tim_ti3_in0

tim_ti3_in[15:1]
TIM_CH4

tim_ti4_in0

tim_ti4_in[15:1]

Reset
enable
up/down
count

Encoder
interface

tim_ti1fp1
tim_ti2fp2

Stop, clear or up/down
XOR

TIM_CH1

Slave
controller
mode

tim_ti1
Input tim_ti1fp1
filter & tim_ti1fp2
edge
detector tim_trc

tim_ic2

Input tim_ti3fp3
tim_ti3 filter &
tim_ti3fp4
edge
detector tim_trc

tim_ic3

Input tim_ti4fp3
tim_ti4 filter &
tim_ti4fp4
edge
detector tim_trc

tim_ic4

CC1I

Capture/Compare 2 register

Prescaler

Capture/Compare 3 register

tim_oc2ref

tim_oc3ref

Capture/Compare 4 register

tim_oc4ref

TIM_CH1

tim_oc2

Output
control

TIM_CH2

Output
control

TIM_CH3

Output
control

TIM_CH4

tim_oc3

CC4I

CC4I U

Prescaler

tim_oc1

Output
control

CC3I

CC3I U

Prescaler

tim_oc1ref

CC2I

CC2I U

tim_oc4

tim_ocref_clr_int
tim_ocref_clr(1)

tim_ocref_clr [7:0](1)

CNT counter

Capture/Compare 1 register

Prescaler

Input tim_ti2fp1
tim_ti2 filter &
tim_ti2fp2
edge
detector tim_trc

+/-

UI
U

tim_etrf

Notes:
Reg

Preload registers transferred
to active registers on U event
according to control bit
Event
Interrupt & DMA output

MSv62373V6

1. This feature is not available on all timers, refer to Section 55.3: TIM2/TIM3/TIM4/TIM5 implementation.

RM0456 Rev 6

<!-- pagebreak -->

