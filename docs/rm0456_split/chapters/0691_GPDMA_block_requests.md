GPDMA_CxTR2.REQSEL[6:0]

Selected GPDMA request

79

tim15_upd_dma

80

tim15_trg_dma

81

tim15_com_dma

82

tim16_cc1_dma

83

tim16_upd_dma

84

tim17_cc1_dma

85

tim17_upd_dma

86

dcmi_dma or pssi_dma(1)

87

aes_in_dma

88

aes_out_dma

89

hash_in_dma

90

ucpd1_tx_dma

91

ucpd1_rx_dma

92

mdf1_flt0_dma

93

mdf1_flt1_dma

94

mdf1_flt2_dma

95

mdf1_flt3_dma

96

mdf1_flt4_dma

97

mdf1_flt5_dma

98

adf1_flt0_dma

99

fmac_read_dma

100

fmac_write_dma

101

cordic_read_dma

102

cordic_write_dma

103

saes_in_dma

104

saes_out_dma

105

lptim1_ic1_dma

106

lptim1_ic2_dma

107

lptim1_ue_dma

108

lptim2_ic1_dma

109

lptim2_ic2_dma

110

lptim2_ue_dma

111

lptim3_ic1_dma

112

lptim3_ic2_dma

113

lptim3_ue_dma

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Table 137. Programmed GPDMA1 request (continued)
GPDMA_CxTR2.REQSEL[6:0]

Selected GPDMA request

114

hspi1_dma

115

i2c5_rx_dma

116

i2c5_tx_dma

117

i2c5_evc_dma

118

i2c6_rx_dma

119

i2c6_tx_dma

120

i2c6_evc_dma

121

usart6_rx_dma

122

usart6_tx_dma

123

adc2_dma

124

jpeg_rx_dma

125

jpeg_tx_dma

1. Depends on which exclusive function is used.

17.3.4

GPDMA block requests
Some GPDMA requests must be programmed as a block request, and not as a burst
request. Then BREQ in GPDMA_CxTR2 must be set for a correct GPDMA execution of the
requested peripheral transfer at the hardware level.
Table 138. Programmed GPDMA1 request as a block request
GPDMA block requests
lptim1_ue_dma
lptim2_ue_dma

17.3.5

GPDMA triggers
A GPDMA trigger can be assigned to a GPDMA channel x, via TRIGSEL[6:0]
in GPDMA_CxTR2, provided that TRIGPOL[1:0] defines a rising or a falling edge of the
selected trigger (TRIGPOL[1:0] = 01 or TRIGPOL[1:0] = 10).
Table 139. Programmed GPDMA1 trigger
GPDMA_CxTR2.TRIGSEL[6:0]

Selected GPDMA trigger

0

exti0

1

exti1

2

exti2

3

exti3

4

exti4

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Table 139. Programmed GPDMA1 trigger (continued)

<!-- pagebreak -->

GPDMA_CxTR2.TRIGSEL[6:0]

Selected GPDMA trigger

5

exti5

6

exti6

7

exti7

8

tamp_trg1

9

tamp_trg2

10

tamp_trg3

11

lptim1_ch1

12

lptim1_ch2

13

lptim2_ch1

14

lptim2_ch2

15

lptim4_out

16

comp1_out

17

comp2_out

18

rtc_alra_trg

19

rtc_alrb_trg

20

rtc_wut_trg

21

Reserved

22

gpdma1_ch0_tc

23

gpdma1_ch1_tc

24

gpdma1_ch2_tc

25

gpdma1_ch3_tc

26

gpdma1_ch4_tc

27

gpdma1_ch5_tc

28

gpdma1_ch6_tc

29

gpdma1_ch7_tc

30

gpdma1_ch8_tc

31

gpdma1_ch9_tc

32

gpdma1_ch10_tc

33

gpdma1_ch11_tc

34

gpdma1_ch12_tc

35

gpdma1_ch13_tc

36

gpdma1_ch14_tc

37

gpdma1_ch15_tc

38

lpdma1_ch0_tc

39

lpdma1_ch1_tc

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Table 139. Programmed GPDMA1 trigger (continued)
GPDMA_CxTR2.TRIGSEL[6:0]

Selected GPDMA trigger

40

lpdma1_ch2_tc

41

lpdma1_ch3_tc

42

tim2_trgo

43

tim15_trgo

44

tim3_trgo(1)

45

tim4_trgo(1)

46

tim5_trgo(1)

47

ltdc_li

48

dsi_te

49

dsi_er

50

dma2d_tc

51

dma2d_ctc

52

dma2d_tw

53

gpu2d_flag[0]

54

gpu2d_flag[1]

55

gpu2d_flag[2]

56

gpu2d_flag[3]

57

adc4_awd1

58

adc1_awd1

59

gfxtim_ev4

60

gfxtim_ev3

61

gfxtim_ev2

62

gfxtim_ev1

63

jpeg_eoc_trg

64

jpeg_ifnf_trg

65

jpeg_ift_trg

66

jpeg_ofne_trg

67

jpeg_oft_trg

1. Connections only present in STM32U59x/5Ax and STM32U5Fx/5Gx.

RM0456 Rev 6

<!-- pagebreak -->

