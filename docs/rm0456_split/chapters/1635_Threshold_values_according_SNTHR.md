RM0456 Rev 6

RM0456

Audio digital filter (ADF)
•

If SADMOD[1:0] = 1x, THRH is obtained by multiplying the current ANMIN[12:0] by 4.
THR H = ANMIN × 4

This threshold value is then compared to:
ANLVL × 10

GdB SNTHR
---------------------------20

The hysteresis mode can be enabled to reduce the spurious transitions between MONITOR
and DETECT states. In hysteresis mode (HYSTEN = 1), the following threshold values are
used:
•

THRH when the SAD is in MONITOR state.

•

THRL when the SAD is in DETECT state.

Table 401 shows the thresholds values according to SNTHR.
Table 401. Threshold values according SNTHR(1)
SNTHR[3:0]

THRH

THRL

Comments

0

LVL + 3.5 dB

LVL x 1.5

LVL + 3.5 dB

LVL x 1.5

No hysteresis

1

LVL + 6.0 dB

LVL x 2

LVL + 3.5 dB

LVL x 1.5

Hysteresis of 2.5 dB

2

LVL + 9.5 dB

LVL x 3

LVL + 6.0 dB

LVL x 2

Hysteresis of 3.5 dB

3

LVL + 12.0 dB

LVL x 4

LVL + 9.5 dB

LVL x 3

Hysteresis of 2.5 dB

4

LVL + 15.6 dB

LVL x 6

LVL + 12.0 dB

LVL x 4

Hysteresis of 3.5 dB

5

LVL + 18.1 dB

LVL x 8

LVL + 15.6 dB

LVL x 6

Hysteresis of 2.5 dB

6

LVL + 21.6 dB

LVL x 12

LVL + 18.1 dB

LVL x 8

Hysteresis of 3.5 dB

7

LVL + 24.1 dB

LVL x 16

LVL + 21.6 dB

LVL x 12

Hysteresis of 2.5 dB

8

LVL + 27.6 dB

LVL x 24

LVL + 24.1dB

LVL x 16

Hysteresis of 3.5 dB

9

LVL + 30.1 dB

LVL x 32

LVL + 27.6dB

LVL x 24

Hysteresis of 2.5 dB

1. LVL must be replaced by ANLVL when SADMOD[1:0] = 0 and by ANMIN for other SADMOD[1:0] values.

When the hysteresis function is disabled, the SAD always use THRH.
Note:

The hysteresis mode must not be used when SADMOD[1:0] = 1x.

Trigger logic
The signal compared to this threshold depends also on SADMOD[1:0].
The trigger condition is reached when the selected signal (SELSIG) is bigger than the
threshold level.
If the trigger condition is met, the following happens:
•

The SAD switches to DETECT state.

•

The SAD refreshes the hangover counter with HGOVR.

•

The sddet_evt event is asserted if the SAD transits from MONITOR to DETECT.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)
•

RM0456

The adf_sad_det signal is set to high.

The SAD remains in DETECT state as long as the trigger condition is met or the hangover
down-counter is different from 0.
The sddet_evt event indicates when the SAD enters and/or exits the DETECT state. This
event is used to generate an interrupt when a sound is detected or when a sound is no
longer detected:
•

When DETCFG = 0, the application receives an event only when the SAD enters the
DETECT state.

•

When DETCFG = 1, the application receives an event when the SAD enters or exits the
DETECT state.

The adf_sad_det signal remains high as long as the SAD is in DETECT state.
The SAD also provides a flag indicating that a new sound level value is available (SDLVLF).
The last computed sound level (SDLVL[14:0]) is available in the ADF SAD sound level
register (ADF_SADSDLVR), and the last computed ambient noise level (ANLVL[14:0]), in
the ADF SAD ambient noise level register (ADF_SADANLVR).
Note:

The SAD can work even when the AHB clock is not present. In that case, the SAD does not
update SDLVL[14:0] and ANLVL[14:0].
To get the latest valid SDLVL[14:0] and ANLVL[14:0] values, the application must read the
ADF_SADSDLVR, and ADF_SADANLVR registers, when the SDLVLF flag goes high. This
can be done in the following ways:
•

•

by polling the SDLVLF flag:
a)

Clear the SDLVLF flag by writing SDLVLF to 1.

b)

Wait for SDLVLF = 1, by reading ADF_DFLTxISR.

c)

Read ADF_SADSDLVR and ADF_SADANLVR.

d)

Clear SDLVLF by writing it to 1.

e)

Go to step 2 if other values must to be read.

by generating an interrupt:
a)

Read ADF_DFLTxISR.

b)

If SDLVLF = 1, read ADF_SADSDLVR and ADF_SADANLVR, and clear SDLVLF
by writing it to 1.

c)

Handle other status flags and exit from ISR.

Sample transfer to memory
The SAD offers the following options to control the samples transfer from DFLT0 to the
system memory:

<!-- pagebreak -->

•

If DATCAP[1:0] = 1x, the samples are transferred into the system memory as soon as
DFLT0 and SAD are enabled. The transfer does not depend on the SAD state.

•

If DATCAP[1:0] = 01, the samples are transferred into the system memory when the
SAD detects a sound (when the SAD is in DETECT state), assuming that DFLT0 and
SAD are enabled.

•

If DATCAP[1:0] = 0, the samples are not transferred into the memory. This mode can
be used if the application only wants to observe but does not need samples for other
processing.

RM0456 Rev 6

RM0456
Note:

Audio digital filter (ADF)
DATCAP[1:0] is taken into account only when the SADEN = 1. For example, if the SAD
configuration is DATCAP[1:0] = 0, SADEN = DFLTEN = 1, and if the application sets now
SADEN to 0, the samples provided by the DFLT0 are transferred to the RXFIFO.

Programming recommendations
To make the SAD function working properly, the ADF must be programmed as follows:
1.

Provide the proper kernel clock (adf_ker_ck) to the ADF.

2.

Configure the CKGEN and enable it.

3.

Configure the SITF and enable it (note that microphones have a settling time of several
milliseconds).

4.

Configure the DFLT0. A typical setting is the following:
–

CIC5 with a decimation ratio of 12, 16 or 24

–

RSFLT with a decimation ratio of 4

–

HPF with HPFC = 2 or 3

For a very-low power implementation, the RSFLT can be bypassed.
5.

Set SADEN to 0.

6.

Wait for SADACTIVE = 0. If SADEN was previously enabled, this phase can take two
periods of adf_hclk, and two periods of adf_proc_ck.

7.

Configure the SAD as follows:
–

Set DATCAP[1:0] to 0, if the application does not want to store the samples into
the system memory.

–

Set DATCAP[1:0] to 01, if the application wants to store the samples into the
system memory only when the SAD detects a sound.

–

Set DATCAP[1:0] to 11, if the application wants to store the samples into the
system memory continuously.

8.

Configure the DMA (optional).

9.

Enable the SAD.

10. Enable the DFLT0.
Figure 380 shows a simplified timing diagram when the SAD works with DATCAP[1:0] = 01.
Thanks to the kernel clock (adf_ker_ck), the SAD continuously monitors the audio signal
provided by the DFLT0. The threshold is also continuously updated according to the
ambient noise level estimation.
•

When the SAD detects a sound higher than the programmed threshold (1), the ADF
requests the bus clock (adf_bus_ckreq asserted).

•

When the bus clock is available (see 2 in Figure 380) then:
–

The data transfer to the memory is triggered.

–

The event interrupt (adf_evt_it) can be generated.

•

In this example, the event interrupt (adf_evt_it) is used to wake up the application. The
interrupt line is released by clearing SDDETF by writing 1 to it.

•

As long as the SAD remains in DETECT state, the application waits to get enough
samples and calls, for example the keyword recognition algorithm (see 3 in
Figure 380).

•

In the case shown in Figure 381, the SAD state (SADST) goes back to MONITOR
before the keyword is recognized. If DETCFG is set to 1, an event signals when the
SAD goes back to MONITOR state. The SAD stops the transfer of samples into the
RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

memory and the application can clean up the receive buffer for the next detection
(see 4 in Figure 380).
Figure 381. SAD timing diagram example

Threshold
Signal provided
by DFLT0

SADST

LEARN

MONITOR

DETECT

MONITOR

adf_sad_det

adf_bus_ckreq

adf_hclk

SDDETF flag cleared !

adf_flt0_it

Sample transfer
to memory
adf_ker_ck

1 2

40.4.11

3

4

MSv63624V2

Data transfer to memory
Data format
The samples processed by DFLT0 are stored into a RXFIFO. The application can read the
samples stored into these FIFOs via the ADF digital filter data register 0 (ADF_DFLT0DR).
The samples inside this register are signed and left aligned. The bit 31 always represents
the sign.
The ADF provides 24-bit left-aligned data. Performing a 16-bit access to ADF_DFLT0DR
allows the application to get the 16 most significant bits. Performing a 32-bit access to
ADF_DFLT0DR allows the application to get a 24-bit data size.
Figure 382. ADF_DFLTxDR data format

23

RXFIFO width

0

1
...
FIFO_DEPTH

16 15

31

ADF_DFLT0DR

PCM[23:0]

7

0
zero’s
MSv63665V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Audio digital filter (ADF)

Data re-synchronization
The samples stored into the RXFIFO can be transferred into the memory by using either
DMA requests or interrupt signaling.
Note:

The RXFIFO is located into the adf_ker_ck clock domain, while ADF_DFLT0DR is located
into the adf_hclk (AHB) clock domain.
When the AHB clock is available, if ADF_DFLT0DR is empty and if a sample is available
into the RXFIFO, this sample is transferred into ADF_DFLT0DR.
The sample transfer from the RXFIFO to ADF_DFLT0DR takes two periods of the AHB
clock (adf_hclk) and two periods of the adf_ker_ck clock. The ADF inserts automatically
wait-states if the application performs a read operation of ADF_DFLT0DR while the transfer
of the new sample from the RXFIFO to ADF_DFLT0DR is not yet completed.
Figure 383. Data re-synchronization

ADF_DLFT0DR

sync

ADF

AHB clock domain

Digital filter processing (DFLT0)

RXFIFO

adf_ker_ck clock domain
MSv63666V1

Data transfer
The content of the RXFIFO can be transfered to the memory either by using a DMA channel
or interrupt services.
Both single and burst, DMA transfers are supported by the ADF, but the application has to
care about the following points:

Note:

•

The RXFIFO must contain at least the same amount of samples than the burst size.

•

The burst mode efficiency may be reduced due to the data re-synchronization
explained in the previous section.

The burst mode is not available in all products (see the DMA section to check if the product
supports it).
In addition, the application can select the RXFIFO threshold (FTH bit) in order to trigger the
data transfer: a data transfer can be triggered as soon as the RXFIFO is not empty, or when
the RXFIFO is half-full (containing depth/2 samples).
For the DMA transfer, as soon as one of the RXFIFO reaches the threshold level, the DMA
request is asserted in order to ask for data transfer. Successive DMA requests are
performed as long as the RXFIFO is not empty.
The DMA mode of the RXFIFO is enabled via the DMAEN bit in ADF_DFLT0DR.
For the interrupt signaling, the following cases must be considered:
•

If FTH = 0, as soon as a data is available in ADF_DFLT0DR, the FTHF is set, allowing
the generation of an interrupt. FTHF is released as soon as ADF_DFLT0DR is read.

•

If FTH = 1, as soon as the RXFIFO reaches the threshold level and a data is available
in ADF_DFLT0DR, the FTHF is set, allowing the generation of an interrupt. FTHF is
released as soon as one data is read. FTHF is set again if the threshold condition is

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

met again. In this mode, every time an interrupt occurs, the application is supposed to
read FIFO_SIZE/2 data.

RXFIFO overrun
A RXFIFO overrun condition is detected when the RXFIFO is full, and a new sample from
the DFLT0 must be written.
In this case, DOVRF is set and the new sample is dropped. When the RXFIFO has at least
one location available, the new incoming sample is written into the RXFIFO.
Figure 384 shows an example based on a RXFIFO depth of four words and FTH set to 1, so
that FTHF goes to 1 when the RXFIFO is half-full.
The S7 sample is lost due to an overrun: the RXFIFO is full while S7 must be written into the
RXFIFO. The S7 write operation is not performed. DOVRF is set to 1 at the moment where
the write operation was expected. The overflow event remains to 1 as long as it is not
cleared by the application.
In this example, DOVRIE is set to 1 to have an interrupt if an overrun condition is detected.
After the S7 sample, the application manages to read data from the RXFIFO, and the ADF
can write the S8 sample and consecutive. Later, the application clears DOVR, allowing the
detection of a new overrun situation.
In the adf_hclk line, the gray boxes indicate that the ADF requested the AHB clock.
Figure 384 shows the AHB clock available only when the ADF requests it. In real
applications, the AHB clock may also be present if the ADF does not request it.
Figure 384. Example of overflow and transfer to memory
adf_ker_ck
Data from
DFLT0

S1

S2

S3

S4

S5

S6

Data write into
RXFIFO

S7

4

S8

S9

S10

S11

S12

4

3
2

2

1

RXFIFO Level

1

0

2

2
1

1
0

0

RXFIFO Full
FTHF
RXNEF
adf_hclk
S1 S2

S3 S4 S5 S6 S8

S9 S10

read access to
RXFIFO
DOVRF
Writing DOVRF to 1

<!-- pagebreak -->

