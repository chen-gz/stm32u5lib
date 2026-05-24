1377

Analog-to-digital converter (ADC12)

33.4.2

RM0456

ADC pins and internal signals
Table 305. ADC input/output pins
Name

Signal type

VREF+

Input, analog reference
positive

Higher/positive reference voltage for the ADC

VDDA

Input, analog supply

Analog power supply equal VDDA

VREF–

Input, analog reference
negative

Lower/negative reference voltage for the ADC,
VREF– = VSSA

VSSA

Description

Input, analog supply ground Ground for analog power supply equal to VSS

ADCx_INy

External analog input signals Up to 17 external analog input channels

Table 306. ADC internal input/output signals
Internal signal name

Signal type

Description

VINP[i]

Analog inputs

Positive input analog channels for each ADC

VINN[i]

Analog inputs

Negative input analog channels for each ADC

adc_ext_trgy

Inputs

External trigger inputs for the regular
conversions (can be connected to on-chip
timers).

adc_jext_trgy

Inputs

External trigger inputs for the injected
conversions (can be connected to on-chip
timers).

adc_awd1
adc_awd2
adc_awd3

Outputs

Internal analog watchdog output signal
connected to on-chip timers.

adc_it

Output

ADC interrupt

adc_hclk

Input

AHB clock

adc_ker_ck

Input

ADC kernel clock

adc_dma

Output

ADC DMA requests

adcx_dat[15:0]

Output

ADC data outputs (regular data register)

Table 307. ADC1/ADC12 interconnection

<!-- pagebreak -->

Signal name

Source/destination

ADCx VINP[0] (x = 1, 2)

VREFINT buffered voltage

ADCx VINP[18] (x = 1, 2)

VBAT/4

ADCx VINP[19] (x = 1, 2)

VSENSE

adcx_dat[15:0] (x = 1, 2)

mdf1_adcx_dat[15:0]

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Table 308. ADC1/ADC12 external triggers for regular channels
Name

Source

adc_ext_trg0

tim1_oc1

adc_ext_trg1

tim1_oc2

adc_ext_trg2

tim1_oc3

adc_ext_trg3

tim2_oc2

adc_ext_trg4

tim3_trgo

adc_ext_trg5

tim4_oc4

adc_ext_trg6

exti11

adc_ext_trg7

tim8_trgo

adc_ext_trg8

tim8_trgo2

adc_ext_trg9

tim1_trgo

adc_ext_trg10

tim1_trgo2

adc_ext_trg11

tim2_trgo

adc_ext_trg12

tim4_trgo

adc_ext_trg13

tim6_trgo

adc_ext_trg14

tim15_trgo

adc_ext_trg15

tim3_oc4

adc_ext_trg16

exti15

adc_ext_trg18

lptim1_ch1

adc_ext_trg19

lptim2_ch1

adc_ext_trg20

lptim3_ch1

adc_ext_trg21

lptim4_out

Table 309. ADC1/ADC12 external triggers for injected channels
Name

Source

adc_jext_trg0

tim1_trgo

adc_jext_trg1

tim1_oc4

adc_jext_trg2

tim2_trgo

adc_jext_trg3

tim2_oc1

adc_jext_trg4

tim3_oc4

adc_jext_trg5

tim4_trgo

adc_jext_trg6

exti15

adc_jext_trg7

tim8_oc4

adc_jext_trg8

tim1_trgo2

adc_jext_trg9

tim8_trgo

adc_jext_trg10

tim8_trgo2

RM0456 Rev 6

<!-- pagebreak -->

