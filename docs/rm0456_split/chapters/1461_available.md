1.

Set the DAC channel trigger enable bit, TENx.

2.

Configure the trigger sources by setting different values in the TSELx[3:0] bits.

3.

Configure the DAC channel WAVEx[1:0] bits as 01 and the same LFSR mask value in
the MAMPx[3:0] bits.

4.

Load the DAC channel data into the desired DHR register (DAC_DHR12R1,
DAC_DHR12L1 or DAC_DHR8R1).

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
When a DAC channel trigger arrives, the LFSR1 counter, with the same mask, is added to
the DHR1 register and the sum is transferred into DAC_DOR1 (three dac_hclk clock cycles
later). Then the LFSR1 counter is updated.

Independent trigger with single triangle generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the DAC channel trigger enable bits, TENx.

2.

Configure the trigger sources by setting different values in the TSELx[3:0] bits.

3.

Configure the DAC channel WAVEx[1:0] bits as 1x and the same maximum amplitude
value in the MAMPx[3:0] bits.

4.

Load the DAC channel data into the desired DHR register (DAC_DHR12R1,
DAC_DHR12L1 or DAC_DHR8R1).

When a DAC channel trigger arrives, the DAC channel triangle counter, with the same
triangle amplitude, is added to the DHRx register and the sum is transferred into
DAC_DOR1 (three dac_hclk clock cycles later). The DAC channel triangle counter is then
updated.

Independent trigger with single sawtooth generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Configure the trigger sources by setting different values in STRSTTRIGSELx[3:0] and
STINCTRIGSELx[3:0] bits.

2.

Configure the DAC channel WAVEx[1:0] bits to 11 and set the same
STRSTDATAx[11:0], STINCDATAx[15:0] and STDIRx values for each register.

When a DAC channel trigger arrives, the DAC channel sawtooth counter updates the DHRx
register and transfers it into DAC_DOR1 (three AHB clock cycles later).

35.4.15

Dual DAC channel conversion modes (if dual channels are
available)
To efficiently use the bus bandwidth in applications that require the two DAC channels at the
same time, three dual registers are implemented: DHR8RD, DHR12RD and DHR12LD. A
unique register access is then required to drive both DAC channels at the same time. For
the wave generation, no accesses to DHRxxxD registers are required. As a result, two
output channels can be used either independently or simultaneously.
15 conversion modes are possible using the two DAC channels and these dual registers. All
the conversion modes can nevertheless be obtained using separate DAC_DHRx registers if
needed.
All modes are described in the paragraphs below.

Independent trigger without wave generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure different trigger sources by setting different values in the TSEL1 and TSEL2
bitfields.

3.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

When a DAC channel1 trigger arrives, the DHR1 register is transferred into DAC_DOR1
(three dac_hclk clock cycles later).
When a DAC channel2 trigger arrives, the DHR2 register is transferred into DAC_DOR2
(three dac_hclk clock cycles later).

Independent trigger with single LFSR generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure different trigger sources by setting different values in the TSEL1 and TSEL2
bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 01 and the same LFSR mask value
in the MAMPx[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a DAC channel1 trigger arrives, the LFSR1 counter, with the same mask, is added to
the DHR1 register and the sum is transferred into DAC_DOR1 (three dac_hclk clock cycles
later). Then the LFSR1 counter is updated.
When a DAC channel2 trigger arrives, the LFSR2 counter, with the same mask, is added to
the DHR2 register and the sum is transferred into DAC_DOR2 (three dac_hclk clock cycles
later). Then the LFSR2 counter is updated.

Independent trigger with different LFSR generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure different trigger sources by setting different values in the TSEL1 and TSEL2
bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 01 and set different LFSR masks
values in the MAMP1[3:0] and MAMP2[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a DAC channel1 trigger arrives, the LFSR1 counter, with the mask configured by
MAMP1[3:0], is added to the DHR1 register and the sum is transferred into DAC_DOR1
(three dac_hclk clock cycles later). Then the LFSR1 counter is updated.
When a DAC channel2 trigger arrives, the LFSR2 counter, with the mask configured by
MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2
(three dac_hclk clock cycles later). Then the LFSR2 counter is updated.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

Independent trigger with single triangle generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure different trigger sources by setting different values in the TSEL1 and TSEL2
bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 1x and the same maximum
amplitude value in the MAMPx[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a DAC channel1 trigger arrives, the DAC channel1 triangle counter, with the same
triangle amplitude, is added to the DHR1 register and the sum is transferred into
DAC_DOR1 (three dac_hclk clock cycles later). The DAC channel1 triangle counter is then
updated.
When a DAC channel2 trigger arrives, the DAC channel2 triangle counter, with the same
triangle amplitude, is added to the DHR2 register and the sum is transferred into
DAC_DOR2 (three dac_hclk clock cycles later). The DAC channel2 triangle counter is then
updated.

Independent trigger with different triangle generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure different trigger sources by setting different values in the TSEL1 and TSEL2
bits.

3.

Configure the two DAC channel WAVEx[1:0] bits as 1x and set different maximum
amplitude values in the MAMP1[3:0] and MAMP2[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a DAC channel1 trigger arrives, the DAC channel1 triangle counter, with a triangle
amplitude configured by MAMP1[3:0], is added to the DHR1 register and the sum is
transferred into DAC_DOR1 (three dac_hclk clock cycles later). The DAC channel1 triangle
counter is then updated.
When a DAC channel2 trigger arrives, the DAC channel2 triangle counter, with a triangle
amplitude configured by MAMP2[3:0], is added to the DHR2 register and the sum is
transferred into DAC_DOR2 (three dac_hclk clock cycles later). The DAC channel2 triangle
counter is then updated.

Simultaneous software start
To configure the DAC in this conversion mode, the following sequence is required:
•

Load the dual DAC channel data to the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

In this configuration, one dac_hclk clock cycle later, the DHR1 and DHR2 registers are
transferred into DAC_DOR1 and DAC_DOR2, respectively.

Simultaneous trigger without wave generation
To configure the DAC in this conversion mode, the following sequence is required:

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

1.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

2.

Configure the same trigger source for both DAC channels by setting the same value in
the TSEL1 and TSEL2 bitfields.

3.

Load the dual DAC channel data to the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a trigger arrives, the DHR1 and DHR2 registers are transferred into DAC_DOR1 and
DAC_DOR2, respectively (after three dac_hclk clock cycles).

Simultaneous trigger with single LFSR generation
1.

To configure the DAC in this conversion mode, the following sequence is required:

2.

Set the two DAC channel trigger enable bits TEN1 and TEN2.

3.

Configure the same trigger source for both DAC channels by setting the same value in
the TSEL1 and TSEL2 bitfields.

4.

Configure the two DAC channel WAVEx[1:0] bits as 01 and the same LFSR mask value
in the MAMPx[3:0] bits.

5.

Load the dual DAC channel data to the desired DHR register (DHR12RD, DHR12LD or
DHR8RD).

When a trigger arrives, the LFSR1 counter, with the same mask, is added to the DHR1
register and the sum is transferred into DAC_DOR1 (three dac_hclk clock cycles later). The
LFSR1 counter is then updated. At the same time, the LFSR2 counter, with the same mask,
is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three dac_hclk
clock cycles later). The LFSR2 counter is then updated.

Simultaneous trigger with different LFSR generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2

2.

Configure the same trigger source for both DAC channels by setting the same value in
the TSEL1 and TSEL2 bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 01 and set different LFSR mask
values using the MAMP1[3:0] and MAMP2[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a trigger arrives, the LFSR1 counter, with the mask configured by MAMP1[3:0], is
added to the DHR1 register and the sum is transferred into DAC_DOR1 (three dac_hclk
clock cycles later). The LFSR1 counter is then updated.
At the same time, the LFSR2 counter, with the mask configured by MAMP2[3:0], is added to
the DHR2 register and the sum is transferred into DAC_DOR2 (three dac_hclk clock cycles
later). The LFSR2 counter is then updated.

Simultaneous trigger with single triangle generation
To configure the DAC in this conversion mode, the following sequence is required:

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2

2.

Configure the same trigger source for both DAC channels by setting the same value in
the TSEL1 and TSEL2 bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 1x and the same maximum
amplitude value using the MAMPx[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a trigger arrives, the DAC channel1 triangle counter, with the same triangle
amplitude, is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three
dac_hclk clock cycles later). The DAC channel1 triangle counter is then updated.
At the same time, the DAC channel2 triangle counter, with the same triangle amplitude, is
added to the DHR2 register and the sum is transferred into DAC_DOR2 (three dac_hclk
clock cycles later). The DAC channel2 triangle counter is then updated.

Simultaneous trigger with different triangle generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the two DAC channel trigger enable bits TEN1 and TEN2

2.

Configure the same trigger source for both DAC channels by setting the same value in
the TSEL1 and TSEL2 bitfields.

3.

Configure the two DAC channel WAVEx[1:0] bits as 1x and set different maximum
amplitude values in the MAMP1[3:0] and MAMP2[3:0] bits.

4.

Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,
DAC_DHR12LD or DAC_DHR8RD).

When a trigger arrives, the DAC channel1 triangle counter, with a triangle amplitude
configured by MAMP1[3:0], is added to the DHR1 register and the sum is transferred into
DAC_DOR1 (three AHB clock cycles later). Then the DAC channel1 triangle counter is
updated.
At the same time, the DAC channel2 triangle counter, with a triangle amplitude configured
by MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2
(three dac_hclk clock cycles later). Then the DAC channel2 triangle counter is updated.

35.4.16

DAC autonomous mode
The autonomous mode can be used to update the DAC output voltage in Stop mode. This
allows DMA transfers to be performed when the device operates in Run, Sleep or Stop
mode. The autonomous mode is supported AUTOMODE only when the DAC is in sample
and hold mode. It is enabled by setting the bit in the DAC_AUTOCR register. DMA requests
must also be enabled (DMAEN = 1).
When the AUTOMODE bit is set, each hardware trigger signal generates an AHB clock
request to the RCC. Once the peripheral receives the AHB clock, the content of the
DAC_DHRx register is loaded into DAC_DOR register and a DMA request is generated.
When the DMA transaction is complete, the DAC deasserts the AHB clock request and
waits for a new trigger event.
When the sample and hold mode is selected, the dac_ker_ck low-power clock must be
enabled by the RCC.

RM0456 Rev 6

<!-- pagebreak -->

