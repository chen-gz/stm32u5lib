The ENx bit enables the analog DAC channelx only. The DAC channelx digital interface is
enabled even if the ENx bit is reset.

RM0456 Rev 6

RM0456

35.4.5

Digital-to-analog converter (DAC)

DAC data format
Depending on the selected configuration mode, the data have to be written into the specified
register as described below:
•

Single DAC channel
There are three possibilities:
–

8-bit right alignment: the software has to load data into the DAC_DHR8Rx[7:0] bits
(stored into the DHRx[11:4] bits)

–

12-bit left alignment: the software has to load data into the DAC_DHR12Lx [15:4]
bits (stored into the DHRx[11:0] bits)

–

12-bit right alignment: the software has to load data into the DAC_DHR12Rx [11:0]
bits (stored into the DHRx[11:0] bits)

Depending on the loaded DAC_DHRyyyx register, the data written by the user is shifted and
stored into the corresponding DAC_DHRx (data holding registerx, which are internal nonmemory-mapped registers). The DAC_DHRx register is then loaded into the DAC_DORx
register either automatically, by software trigger or by an external event trigger.
Figure 309. Data registers in single DAC channel mode

31

24

15

7

0
8-bit right aligned
12-bit left aligned
12-bit right aligned
ai14710b

•

Dual DAC channels (when available)
There are three possibilities:
–

8-bit right alignment: data for DAC channel1 to be loaded into the DAC_DHR8RD
[7:0] bits (stored into the DHR1[11:4] bits) and data for DAC channel2 to be loaded
into the DAC_DHR8RD [15:8] bits (stored into the DHR2[11:4] bits)

–

12-bit left alignment: data for DAC channel1 to be loaded into the DAC_DHR12LD
[15:4] bits (stored into the DHR1[11:0] bits) and data for DAC channel2 to be
loaded into the DAC_DHR12LD [31:20] bits (stored into the DHR2[11:0] bits)

–

12-bit right alignment: data for DAC channel1 to be loaded into the
DAC_DHR12RD [11:0] bits (stored into the DHR1[11:0] bits) and data for DAC
channel2 to be loaded into the DAC_DHR12RD [27:16] bits (stored into the
DHR2[11:0] bits)

Depending on the loaded DAC_DHRyyyD register, the data written by the user is shifted
and stored into DHR1 and DHR2 (data holding registers, which are internal non-memorymapped registers). The DHR1 and DHR2 registers are then loaded into the DAC_DOR1
and DOR2 registers, respectively, either automatically, by software trigger or by an external
event trigger.

RM0456 Rev 6

<!-- pagebreak -->

