1485

Digital-to-analog converter (DAC)

35.4.7

RM0456

DAC output voltage
Digital inputs are converted to output voltages on a linear conversion between 0 and VREF+.
The analog output voltages on each DAC channel pin are determined by the following
equation:
DOR
DAC output = V REF × ------------4096

where all voltages are expressed in Volt.

35.4.8

DAC trigger selection
If the TENx control bit is set, the conversion can then be triggered by an external event (timer
counter, external interrupt line). The TSELx[3:0] control bits determine which out of 16 possible events triggers the conversion as shown in TSELx[3:0] bits of the DAC_CR register.
These events can be either the software trigger or hardware triggers. Refer to the interconnection table in Section 35.4.2.
Each time a DAC interface detects a rising edge on the selected trigger source (refer to the
table below), the last data stored into the DAC_DHRx register are transferred into the
DAC_DORx register. The DAC_DORx register is updated three dac_hclk cycles after the
trigger occurs.
If the software trigger is selected, the conversion starts once the SWTRIG bit is set.
SWTRIG is reset by hardware once the DAC_DORx register has been loaded with the
DAC_DHRx register contents.

Note:

TSELx[3:0] bit cannot be changed when the ENx bit is set.
When software trigger is selected, the transfer from the DAC_DHRx register to the
DAC_DORx register takes only one dac_hclk clock cycle.

35.4.9

DMA requests
Each DAC channel has a DMA capability. Two DMA channels are used to service DAC
channel DMA requests.
When an external trigger (but not a software trigger) occurs while the DMAENx bit is set, the
value of the DAC_DHRx register is transferred into the DAC_DORx register when the
transfer is complete, and a DMA request is generated.
In dual mode, if both DMAENx bits are set, two DMA requests are generated. If only one
DMA request is needed, only the corresponding DMAENx bit must be set. In this way, the
application can manage both DAC channels in dual mode by using one DMA request and a
unique DMA channel.
As DAC_DHRx to DAC_DORx data transfer occurred before the DMA request, the very first
data has to be written to the DAC_DHRx before the first trigger event occurs.

DMA underrun
The DAC DMA request is not queued so that if a second external trigger arrives before the
acknowledgment for the first external trigger is received (first request), then no new request
is issued and the DMA channelx underrun flag DMAUDRx in the DAC_SR register is set,
reporting the error condition. The DAC channelx continues to convert old data.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
The software must clear the DMAUDRx flag by writing 1, clear the DMAEN bit of the used
DMA stream and re-initialize both DMA and DAC channelx to restart the transfer correctly.
The software must modify the DAC trigger conversion frequency or lighten the DMA
workload to avoid a new DMA underrun. Finally, the DAC conversion can be resumed
by enabling both DMA data transfer and conversion trigger.
For each DAC channelx, an interrupt is also generated if its corresponding DMAUDRIEx bit
in the DAC_CR register is enabled.

DMA double data mode
When the DMA controller is used in normal mode, only 12-bit (or 8-bit) data are transferred
by a DMA request. As the AHB width is 32 bits, two 12-bit data may be transferred
simultaneously. To use this mode, set the DMADOUBLEx bit of DAC_MCR register.
A DAC DMA request is generated every two external triggers (except for software triggers)
when the DMAENx bit is set:
1.

When the first trigger is detected, the value of the DAC_DHRx and DAC_DHRBx
registers are transferred into the DAC_DORx and DAC_DORBx registers. The actual
DAC data is loaded into the DAC_DORx register. A DMA request is then generated.
The DMA writes the new data to the DAC_DHRx and DAC_DHRBx data registers.

2.

When the next trigger is detected, the actual DAC data is loaded into the DAC_DHRBx
register. This second trigger does not generate any DMA request. The DORSTATx bit
indicates which DOR data is actually loaded into the analog DAC input.

DMA underrun function is also supported in DMA double data mode.
The following conditions must be met to change from double data to single data mode or
vice versa:

35.4.10

•

The DAC must be disabled.

•

DMAEN bit must be cleared (ENx = 0 and DMAEN = 0).

Noise generation
In order to generate a variable-amplitude pseudonoise, an LFSR (linear feedback shift
register) is available. DAC noise generation is selected by setting WAVEx[1:0] to 01. The
preloaded value in LFSR is 0xAAA. This register is updated three dac_hclk clock cycles
after each trigger event, following a specific calculation algorithm.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Figure 312. DAC LFSR register calculation algorithm

XOR
X6
X 12

11

10

9

8

7

6

X4
5

4

X0

X
3

2

1

0

12
NOR

ai14713c

The LFSR value, that may be masked partially or totally by means of the MAMPx[3:0] bits in
the DAC_CR register, is added up to the DAC_DHRx contents without overflow and this
value is then transferred into the DAC_DORx register.
If LFSR is 0x0000, a 1 is injected into it (antilock-up mechanism).
It is possible to reset LFSR wave generation by resetting the WAVEx[1:0] bits.
Figure 313. DAC conversion (SW trigger enabled) with LFSR wave generation

Bus clock

DHR

DOR

0x00

0xAAA

0xD55

SWTRIG
MSv45320V2

Note:

<!-- pagebreak -->

The DAC trigger must be enabled for noise generation by setting the TENx bit in the
DAC_CR register.

RM0456 Rev 6

RM0456

35.4.11

Digital-to-analog converter (DAC)

Triangle-wave generation
It is possible to add a small-amplitude triangular waveform on a DC or slowly varying signal.
DAC triangle-wave generation is selected by setting WAVEx[1:0] to 10. The amplitude is
configured through the MAMPx[3:0] bits in the DAC_CR register. An internal triangle counter
is incremented three dac_hclk clock cycles after each trigger event. The value of this
counter is then added to the DAC_DHRx register without overflow and the sum is
transferred into the DAC_DORx register. The triangle counter is incremented as long as it is
less than the maximum amplitude defined by the MAMPx[3:0] bits. Once the configured
amplitude is reached, the counter is decremented down to 0, then incremented again and so
on.
It is possible to reset triangle wave generation by resetting the WAVEx[1:0] bits.

n

tio

ta

en

m
re

In
cr
em

c
De

MAMPx[3:0] max amplitude
+ DAC_DHRx base value

en
ta
tio
n

Figure 314. DAC triangle wave generation

DAC_DHRx base value
0
ai14715c

Figure 315. DAC conversion (SW trigger enabled) with triangle wave generation

Bus clock

DHR

DOR

0xABE

0xABE

0xABF

0xAC0

SWTRIG
MS45321V2

Note:

The DAC trigger must be enabled for triangle wave generation by setting the TENx bit in the
DAC_CR register.
The MAMPx[3:0] bits must be configured before enabling the DAC, otherwise they cannot
be changed.

RM0456 Rev 6

<!-- pagebreak -->

