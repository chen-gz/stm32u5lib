1861

DSI Host (DSI)

RM0456

the value defined by the low-power reception timeout counter (LPRX_TOCNT) field of the
DSI Host timeout counter configuration register 1 (DSI_TCCR1), the timeout low-power
reception (TOLPRX) bit in the DSI Host interrupt enable register 1 (DSI_IER1) is asserted
and an internal soft reset is generated to the DSI Host.
If the timeout low-power reception interrupt enable (TOLPRXIE) bit of the DSI Host interrupt
enable register 1 (DSI_IER1) is set, an interrupt is generated. Once the software gets
notified by the interrupt, it must reset the D-PHY by deasserting and asserting the digital
enable (DEN) bit of the DSI Host PHY control register (DSI_PCTLR).

44.8.2

Peripheral response timeout counters
A peripheral may not immediately respond correctly to some received packets. For
example, a peripheral receives a read request, but due to its architecture cannot access the
RAM for a while (for example, the panel is being refreshed and takes some time to
respond). In this case, set a timeout to ensure that the host waits long enough so that the
device is able to process the previous data before receiving the new data or responding
correctly to new requests.
Table 440 lists the events belonging to various categories having an associated timeout for
peripheral response.
Table 440. List of events of different categories of the PRESP_TO counter
Category

Event

Items implying a BTA PRESP_TO

Bus-turn-around

READ requests indicating a PRESP_TO
(replicated for HS and LP)

(0x04) Generic read, no parameters short
(0x14) Generic read, 1 parameter short
(0x24) Generic read, 2 parameters short
(0x06) DCS read, no parameters short

(0x03) Generic short write, no parameters short
(0x13) Generic short write, 1 parameter short
(0x23) Generic short write, 2 parameters short
WRITE requests indicating a PRESP_TO (0x29) Generic long write long
(replicated for HS and LP)
(0x05) DCS short write, no parameters short
(0x15) DCS short write, 1 parameter short
(0x39) DCS long write/write_LUT, command packet long
(0x37) Set maximum return packet size

The DSI Host ensures that, on sending an event that triggers a timeout, the D-PHY switches
to the Stop state and a counter starts running until it reaches the value of that timeout. The
link remains in the LP-11 state and unused until the timeout ends, even if there are other
events ready to be transmitted.
Figures 423 to 425 illustrate the flow of counting in the PRESP_TO counter for the three
categories listed in Table 440.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
Figure 423. Timing of PRESP_TO after a bus-turn-around

Host

Device
BTA

ck
er | A

pt

ror R

& Er

rigg
Ack T

PRESP_TO

Timer < PRESP_TO

BTA

LP-11

Device Ready

Arbitra

ry even

t after B

TA

MSv35866V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Figure 424. Timing of PRESP_TO after a read request (HS or LP)

Host

Device

READ

Reque

st

PRESP_TO

Timer < PRESP_TO

LP-11

Device Ready
BTA

ck &

D

REA

|A
Resp

Error

Rpt

BTA

MSv35867V1

<!-- pagebreak -->

