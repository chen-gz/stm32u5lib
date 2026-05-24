RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Table 135. GPDMA1 channel implementation
Hardware parameters

Channel x

x = 0 to 11

x = 12 to 15

17.3.2

Features

dma_fifo_
dma_
size[x]
addressing[x]

2

4

0

Channel x (x = 0 to 11) is implemented with:
– a FIFO of 8 bytes, 2 words
– fixed/contiguously incremented addressing
These channels must be typically allocated for GPDMA transfers
between an APB or AHB peripheral and SRAM.

1

Channel x (x = 12 to 15) is implemented with:
– a FIFO of 32 bytes, 8 words
– 2D addressing
These channels may be also used for GPDMA transfers, between a
demanding AHB peripheral and SRAM, or for transfers from/to external
memories.

GPDMA autonomous mode in low-power modes
The GPDMA autonomous mode and wake-up features are implemented in the device
low-power modes as per the table below.
Table 136. GPDMA1 autonomous mode and wake-up in low-power modes

17.3.3

Feature

Low-power modes

Autonomous mode and wake-up

GPDMA1 in Sleep, Stop 0, and Stop 1 modes

GPDMA requests
A GPDMA request from a peripheral can be assigned to a GPDMA channel x, via
REQSEL[6:0] in GPDMA_CxTR2, provided that SWREQ = 0.
The GPDMA requests mapping is specified in the table below.
Table 137. Programmed GPDMA1 request
GPDMA_CxTR2.REQSEL[6:0]

Selected GPDMA request

0

adc1_dma

1

adc4_dma

2

dac1_ch1_dma

3

dac1_ch2_dma

4

tim6_upd_dma

5

tim7_upd_dma

6

spi1_rx_dma

7

spi1_tx_dma

8

spi2_rx_dma

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Table 137. Programmed GPDMA1 request (continued)

<!-- pagebreak -->

GPDMA_CxTR2.REQSEL[6:0]

Selected GPDMA request

9

spi2_tx_dma

10

spi3_rx_dma

11

spi3_tx_dma

12

i2c1_rx_dma

13

i2c1_tx_dma

14

i2c1_evc_dma

15

i2c2_rx_dma

16

i2c2_tx_dma

17

i2c2_evc_dma

18

i2c3_rx_dma

19

i2c3_tx_dma

20

i2c3_evc_dma

21

i2c4_rx_dma

22

i2c4_tx_dma

23

i2c4_evc_dma

24

usart1_rx_dma

25

usart1_tx_dma

26

usart2_rx_dma

27

usart2_tx_dma

28

usart3_rx_dma

29

usart3_tx_dma

30

uart4_rx_dma

31

uart4_tx_dma

32

uart5_rx_dma

33

uart5_tx_dma

34

lpuart1_rx_dma

35

lpuart1_tx_dma

36

sai1_a_dma

37

sai1_b_dma

38

sai2_a_dma

39

sai2_b_dma

40

octospi1_dma

41

octospi2_dma

42

tim1_cc1_dma

43

tim1_cc2_dma

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Table 137. Programmed GPDMA1 request (continued)
GPDMA_CxTR2.REQSEL[6:0]

Selected GPDMA request

44

tim1_cc3_dma

45

tim1_cc4_dma

46

tim1_upd_dma

47

tim1_trg_dma

48

tim1_com_dma

49

tim8_cc1_dma

50

tim8_cc2_dma

51

tim8_cc3_dma

52

tim8_cc4_dma

53

tim8_upd_dma

54

tim8_trg_dma

55

tim8_com_dma

56

tim2_cc1_dma

57

tim2_cc2_dma

58

tim2_cc3_dma

59

tim2_cc4_dma

60

tim2_upd_dma

61

tim3_cc1_dma

62

tim3_cc2_dma

63

tim3_cc3_dma

64

tim3_cc4_dma

65

tim3_upd_dma

66

tim3_trg_dma

67

tim4_cc1_dma

68

tim4_cc2_dma

69

tim4_cc3_dma

70

tim4_cc4_dma

71

tim4_upd_dma

72

tim5_cc1_dma

73

tim5_cc2_dma

74

tim5_cc3_dma

75

tim5_cc4_dma

76

tim5_upd_dma

77

tim5_trg_dma

78

tim15_cc1_dma

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Table 137. Programmed GPDMA1 request (continued)

<!-- pagebreak -->

