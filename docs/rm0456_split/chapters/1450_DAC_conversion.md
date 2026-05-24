1485

Digital-to-analog converter (DAC)

RM0456

Figure 310. Data registers in dual DAC channel mode
31

24

15

7

0
8-bit right aligned
12-bit left aligned
12-bit right aligned
ai14709b

Signed/unsigned data
DAC input data are unsigned: 0x000 corresponds to the minimum value and 0xFFF to the
maximum value for 12-bit mode.
The DAC can also handle signed input data in 2’s complement format. This is done by
setting SINFORMATx bit in the DAC_MCR register.
When SINFORMATx bit is set, the MSB of the data written to DAC_DHRx registers is
inverted when it is copied to the DAC_DORx register, and the DAC interface can accept
signed data (Q1.15, Q1.11 or Q1.7 format). DAC_DHR12Lx register can be used to store
16-bit signed data in the data holding registers. The 12 MSBs of 16-bit data are used for the
DAC output data and the MSB is inverted. The four LSBs are simply ignored.
Table 344. Data format (case of 12-bit data)

35.4.6

SINFORMATx bit

DATA written to DAC_DHRx
register

DATA transfered to DAC_DORx
register

0

0x000

0x000

0

0xFFF

0xFFF

1

0x7FF

0xFFF

1

0x000

0x800

1

0xFFF

0x7FF

1

0x800

0x000

DAC conversion
The DAC_DORx cannot be written directly and any data transfer to the DAC channelx must
be performed by loading the DAC_DHRx register (write operation to DAC_DHR8Rx,
DAC_DHR12Lx, DAC_DHR12Rx, DAC_DHR8RD, DAC_DHR12RD or DAC_DHR12LD).
Data stored in the DAC_DHRx register are automatically transferred to the DAC_DORx
register after one dac_hclk clock cycle, if no hardware trigger is selected (TENx bit in
DAC_CR register is reset). However, when a hardware trigger is selected (TENx bit in
DAC_CR register is set) and a trigger occurs, the transfer is performed three dac_hclk clock
cycles after the trigger signal.
When DAC_DORx is loaded with the DAC_DHRx contents, the analog output voltage
becomes available after a time tSETTLING that depends on the power supply voltage and the
analog output load.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
To synchronize DAC and ADC, the same clock source can be used for both peripherals.
This is done by selecting the dac_ker_ck clock instead of the dac_hclk clock (AHB clock) in
the RCC.
HFSEL bits of DAC_MCR must be set when dac_ker_ck clock speed is faster than 80 MHz.
Refer to Table HFSEL description below for the limitation of the DAC_DORx update rate
depending on HFSEL bits and dac_hclk or dac_ker_ck clock frequency.
If the data is updated or a software/hardware trigger event occurs during the non-allowed
period, the peripheral behavior is unpredictable.
The above timing is only related to the limitation of the DAC interface. Refer also to the
tSETTLING parameter value in the product datasheet.
Table 345. HFSEL description
HFSEL
Clock frequency
[1:0]

Latency using
AHB clock
(dac_hclk)

Latency using
dac_ker_ck
clock

Function

00

< 80 MHz

3

4

DAC_DOR update rate up to 3
AHB clock cycles or 4
dac_ker_ck cycles.

01

≥ 80 MHz(1)

5

5

DAC_DOR update rate up to 5
AHB clock or dac_ker_ck
cycles.

10

≥ 160 MHz

7

6

DAC_DOR update rate up to 7
AHB clock cycles or 6
dac_ker_ck cycles.

11

Reserved

-

-

-

1. Refer to the device datasheet for the value of the maximum dac_hclk or dac_ker_ck frequency.

Figure 311. Timing diagram for conversion with trigger disabled TEN = 0

Bus clock

DHR

DOR

0x1AC

0x1AC

Output voltage available on
DAC_OUT pin

tSETTLING
MSv45319V2

RM0456 Rev 6

<!-- pagebreak -->

