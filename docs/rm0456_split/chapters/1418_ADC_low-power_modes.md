1443

Analog-to-digital converter (ADC4)

RM0456

prevent any unwanted consumption on the battery, it is recommended to enable the bridge
divider only during ADC conversion.
Figure 307. VBAT channel block diagram

VBAT

ADC
VBAT/4

+

ADC VIN[14]

Address/data bus

VBATEN control bit

-

MSv62489V3

34.4.29

Concurrent operation with another ADC
When ADC4 is used simultaneously with another ADC (let us call it ADCx, x being different
from 4), ADCx operation might generate noise on VREF+ voltage. Since VREF+ is also ADC4
reference voltage, this might cause conversion errors. To prevent this issue from happening,
set VREFPROTEN bit in ADC_PWRR register. As soon as ADCx sampling phase starts,
ADC4 is put on hold during one ADC4 clock cycle, thus resulting in ADC4 conversion time to
be longer of one clock cycle.
In addition, ADCx might have two sampling phases during ADC4 conversion. This is due to
the injected conversion function of ADCx. By setting VREFSECSMP to 1 in ADC_PWRR,
ADC4 operation can be held twice during the conversion phase. When VREFSECSMP bit is
set, ADC4 conversion time is longer of two clock cycles.
VREFSECSMP and VREFPROTEN bits must be set simultaneously.
ADCx and ADC4 must use the same clock source to be able to use the concurrent operation
feature.

34.5

ADC low-power modes
Table 337. Effect of low-power modes on the ADC

<!-- pagebreak -->

