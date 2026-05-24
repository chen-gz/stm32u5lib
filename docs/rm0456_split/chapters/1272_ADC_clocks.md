1377

Analog-to-digital converter (ADC12)

RM0456

Table 309. ADC1/ADC12 external triggers for injected channels (continued)

33.4.3

Name

Source

adc_jext_trg11

tim3_oc3

adc_jext_trg12

tim3_trgo

adc_jext_trg13

tim3_oc1

adc_jext_trg14

tim6_trgo

adc_jext_trg15

tim15_trgo

adc_jext_trg18

lptim1_ch2

adc_jext_trg19

lptim2_ch2

adc_jext_trg20

lptim3_ch1

adc_jext_trg21

lptim4_out1

ADC clocks
Dual clock domain architecture
Dual clock-domain architecture means that the ADC kernel clock is independent from the
AHB bus clock that is used to access ADC registers.
The adc_ker_ck input clock can be selected between different clock sources (see
Figure 222: ADC clock scheme). This selection is done in the RCC (refer to th RCC section
for more information):
1.

The ADC clock can be provided by an internal or external clock source, which is
independent and asynchronous with the AHB clock.

2.

The ADC clock can be derived from the AHB clock.

Option 1 has the advantage of achieving the maximum ADC clock frequency whatever the
AHB clock scheme selected. The ADC clock can eventually be divided by a ratio of 1, 2, 4,
6, 8, 10, 12, 16, 32, 64, 128 or 256, using the prescaler configured through the PRESC[3:0]
bits in the ADC12_CCR register.
Option 2 enables to bypass the clock domain resynchronizations. This can be useful when
the ADC is triggered by a timer and the application requires that the ADC is accurately
triggered without any uncertainty (otherwise, an uncertainty of the trigger instant is added by
the resynchronizations between the two clock domains).
The clock is configured through the RCC. It must be compliant with the operating frequency
specified in the device datasheet.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 222. ADC clock scheme
RCC
(Reset and clock
controller)

ADC1 and ADC2
adc_hclk

adc_ker_ck

AHB interface

/1, 2, 4, 6, 8, 10,
12, 16, 32, 64,
128, 256

Fadc_ker_ck

Analog ADC1, 2

Bits PREC[3:0]
of ADCCx_CCR

MSv65324V2

1. Refer to the RCC section for information on adc_hclk and adc_ker_ck generation.

Clock ratio constraint between ADC clock and AHB clock
There are generally no constraints to be respected for the ratio between the ADC clock and
the AHB clock. However, the ratio must be carefully chosen to avoid any overrun especially
if the clock AHB is much slower than the ADC clock.

RM0456 Rev 6

<!-- pagebreak -->

