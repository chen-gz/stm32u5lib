3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bits 31:24 PTXQTOP[7:0]: Top of the periodic transmit request queue
This indicates the entry in the periodic Tx request queue that is currently being processed by
the MAC.
This register is used for debugging.
Bit 31: Odd/Even frame
0: send in even frame
1: send in odd frame
Bits 30:27: Channel/endpoint number
Bits 26:25: Type
00: IN/OUT
01: Zero-length packet
11: Disable channel command
Bit 24: Terminate (last entry for the selected channel/endpoint)
Bits 23:16 PTXQSAV[7:0]: Periodic transmit request queue space available
Indicates the number of free locations available to be written in the periodic transmit request
queue. This queue holds both IN and OUT requests.
00: Periodic transmit request queue is full
01: 1 location available
10: 2 locations available
bxn: n locations available (0 ≤ n ≤ 8)
Others: Reserved
Bits 15:0 PTXFSAVL[15:0]: Periodic transmit data FIFO space available
Indicates the number of free locations available to be written to in the periodic Tx FIFO.
Values are in terms of 32-bit words
0000: Periodic Tx FIFO is full
0001: 1 word available
0010: 2 words available
bxn: n words available (where 0 ≤ n ≤ PTXFD)
Others: Reserved

72.15.25 OTG host all channels interrupt register (OTG_HAINT)
Address offset: 0x414
Reset value: 0x0000 0000
When a significant event occurs on a channel, the host all channels interrupt register
interrupts the application using the host channels interrupt bit of the core interrupt register
(HCINT bit in OTG_GINTSTS). This is shown in Figure 901. There is one interrupt bit per
channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the
application sets and clears bits in the corresponding host channel-x interrupt register.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

HAINT[15:0]
r

r

<!-- pagebreak -->

r

r

r

r

r

r

r

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 HAINT[15:0]: Channel interrupts
One bit per channel: Bit 0 for Channel 0, bit 15 for Channel 15

72.15.26 OTG host all channels interrupt mask register
(OTG_HAINTMSK)
Address offset: 0x418
Reset value: 0x0000 0000
The host all channel interrupt mask register works with the host all channel interrupt register
to interrupt the application when an event occurs on a channel. There is one interrupt mask
bit per channel, up to a maximum of 16 bits.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

HAINTM[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 HAINTM[15:0]: Channel interrupt mask
0: Masked interrupt
1: Unmasked interrupt
One bit per channel: Bit 0 for channel 0, bit 15 for channel 15

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.27 OTG host port control and status register (OTG_HPRT)
Address offset: 0x440
Reset value: 0x0000 0000
This register is available only in host mode. Currently, the OTG host supports only one port.
A single register holds USB port-related information such as USB reset, enable, suspend,
resume, connect status, and test mode for each port. It is shown in Figure 901. The rc_w1
bits in this register can trigger an interrupt to the application through the host port interrupt
bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the
application must read this register and clear the bit that caused the interrupt. For the rc_w1
bits, the application must write a 1 to the bit to clear the interrupt.
31
Res.

15

30

29

Res.

Res.

14

13

PTCTL[2:0]
rw

rw

28
Res.

12
PPWR

rw

rw

27
Res.

11

26
Res.

10

PLSTS[1:0]
r

r

25
Res.

9
Res.

24
Res.

23
Res.

8

7

22
Res.

21
Res.

20
Res.

19
Res.

18

17

PSPD[1:0]

16
PTCTL
[3]

r

r

rw

1

0

6

5

4

3

2

POCA

PEN
CHNG

PENA

PCDET PCSTS

r

rc_w1

rc_w1

rc_w1

PRST

PSUSP

PRES

POC
CHNG

rw

rs

rw

rc_w1

r

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:17 PSPD[1:0]: Port speed
Indicates the speed of the device attached to this port.
01: Full speed
10: Low speed
11: Reserved
Bits 16:13 PTCTL[3:0]: Port test control
The application writes a nonzero value to this field to put the port into a Test mode, and the
corresponding pattern is signaled on the port.
0000: Test mode disabled
0001: Test_J mode
0010: Test_K mode
0011: Test_SE0_NAK mode
0100: Test_Packet mode
0101: Test_Force_Enable
Others: Reserved
Bit 12 PPWR: Port power
The application uses this field to control power to this port, and the core clears this bit on an
overcurrent condition.
0: Power off
1: Power on
Bits 11:10 PLSTS[1:0]: Port line status
Indicates the current logic level USB data lines
Bit 10: Logic level of OTG_DP
Bit 11: Logic level of OTG_DM
Bit 9 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bit 8 PRST: Port reset
When the application sets this bit, a reset sequence is started on this port. The application
must time the reset period and clear this bit after the reset sequence is complete.
0: Port not in reset
1: Port in reset
The application must leave this bit set for a minimum duration of at least 10 ms to start a
reset on the port. The application can leave it set for another 10 ms in addition to the required
minimum duration, before clearing the bit, even though there is no maximum limit set by the
USB standard.
High speed: 50 ms
Full speed/Low speed: 10 ms
Bit 7 PSUSP: Port suspend
The application sets this bit to put this port in suspend mode. The core only stops sending
SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit,
which asserts the suspend input pin of the PHY.
The read value of this bit reflects the current suspend status of the port. This bit is cleared by
the core after a remote wake-up signal is detected or the application sets the port reset bit or
port resume bit in this register or the resume/remote wake-up detected interrupt bit or
disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in
OTG_GINTSTS, respectively).
0: Port not in suspend mode
1: Port in suspend mode
Bit 6 PRES: Port resume
The application sets this bit to drive resume signaling on the port. The core continues to drive
the resume signal until the application clears this bit.
If the core detects a USB remote wake-up sequence, as indicated by the port resume/remote
wake-up detected interrupt bit of the core interrupt register (WKUPINT bit in
OTG_GINTSTS), the core starts driving resume signaling without application intervention
and clears this bit when it detects a disconnect condition. The read value of this bit indicates
whether the core is currently driving resume signaling.
0: No resume driven
1: Resume driven
When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow:
1. The application sets this bit to drive resume signaling on the port.
2. The core continues to drive the resume signal until a predetermined time specified in
BESLTHRS[3:0] field of OTG_GLPMCFG register.
3. If the core detects a USB remote wake-up sequence, as indicated by the port
L1Resume/Remote L1Wakeup detected interrupt bit of the core interrupt register (WKUPINT
in OTG_GINTSTS), the core starts driving resume signaling without application intervention
and clears this bit at the end of resume.This bit can be set or cleared by both the core and
the application. This bit is cleared by the core even if there is no device connected to the
host.
Bit 5 POCCHNG: Port overcurrent change
The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register
changes.
Bit 4 POCA: Port overcurrent active
Indicates the overcurrent condition of the port.
0: No overcurrent condition
1: Overcurrent condition
Bit 3 PENCHNG: Port enable/disable change
The core sets this bit when the status of the port enable bit 2 in this register changes.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 2 PENA: Port enable
A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent
condition, a disconnect condition, or by the application clearing this bit. The application
cannot set this bit by a register write. It can only clear it to disable the port. This bit does not
trigger any interrupt to the application.
0: Port disabled
1: Port enabled
Bit 1 PCDET: Port connect detected
The core sets this bit when a device connection is detected to trigger an interrupt to the
application using the host port interrupt bit in the core interrupt register (HPRTINT bit in
OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
Bit 0 PCSTS: Port connect status
0: No device is attached to the port
1: A device is attached to the port

72.15.28 OTG host channel x characteristics register (OTG_HCCHARx)
Address offset: 0x500 + 0x20 * x, (x = 0 to 11)
Reset value: 0x0000 0000
31

30

CHENA CHDIS

29

28

27

26

ODD
FRM

25

24

23

22

DAD[6:0]

21

20

19

18

MCNT[1:0]

EPTYP[1:0]

17

16

LSDEV

Res.

rs

rs

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

EPDIR
rw

EPNUM[3:0]
rw

rw

rw

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

rw

Bit 31 CHENA: Channel enable
This field is set by the application and cleared by the OTG host.
0: Channel disabled
1: Channel enabled
Bit 30 CHDIS: Channel disable
The application sets this bit to stop transmitting/receiving data on a channel, even before
the transfer for that channel is complete. The application must wait for the Channel disabled
interrupt before treating the channel as disabled.
Bit 29 ODDFRM: Odd frame
This field is set (reset) by the application to indicate that the OTG host must perform a
transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt)
transactions.
0: Even frame
1: Odd frame
Bits 28:22 DAD[6:0]: Device address
This field selects the specific device serving as the data source or sink.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 21:20 MCNT[1:0]: Multicount
This field indicates to the host the number of transactions that must be executed per frame
for this periodic endpoint. For non-periodic transfers, this field is not used
00: Reserved. This field yields undefined results
01: 1 transaction
10: 2 transactions per frame to be issued for this endpoint
11: 3 transactions per frame to be issued for this endpoint
Note: This field must be set to at least 01.
Bits 19:18 EPTYP[1:0]: Endpoint type
Indicates the transfer type selected.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 LSDEV: Low-speed device
This field is set by the application to indicate that this channel is communicating to a lowspeed device.
Bit 16 Reserved, must be kept at reset value.
Bit 15 EPDIR: Endpoint direction
Indicates whether the transaction is IN or OUT.
0: OUT
1: IN
Bits 14:11 EPNUM[3:0]: Endpoint number
Indicates the endpoint number on the device serving as the data source or sink.
Bits 10:0 MPSIZ[10:0]: Maximum packet size
Indicates the maximum packet size of the associated endpoint.

72.15.29 OTG host channel x interrupt register (OTG_HCINTx)
Address offset: 0x508 + 0x20 * x, (x = 0 to 11)
Reset value: 0x0000 0000
This register indicates the status of a channel with respect to USB- and AHB-related events.
It is shown in Figure 901. The application must read this register when the host channels
interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the
application can read this register, it must first read the host all channels interrupt
(OTG_HAINT) register to get the exact channel number for the host channel-x interrupt
register. The application must clear the appropriate bit in this register to clear the
corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

DTERR

FRM
OR

BBERR TXERR

Res.

ACK

NAK

STALL

Res.

CHH

XFRC

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Res.

Res.

Res.

Res.

rc_w1

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 DTERR: Data toggle error.
Bit 9 FRMOR: Frame overrun.
Bit 8 BBERR: Babble error.
Bit 7 TXERR: Transaction error.
Indicates one of the following errors occurred on the USB.
CRC check failure
Timeout
Bit stuff error
False EOP
Bit 6 Reserved, must be kept at reset value.
Bit 5 ACK: ACK response received/transmitted interrupt.
Bit 4 NAK: NAK response received interrupt.
Bit 3 STALL: STALL response received interrupt.
Bit 2 Reserved, must be kept at reset value.
Bit 1 CHH: Channel halted
This indicates that the transfer completed:
– because of any USB transaction error
– in response to disable request by the application
– normally
Bit 0 XFRC: Transfer completed.
Transfer completed normally without any errors.

72.15.30 OTG host channel x interrupt mask register (OTG_HCINTMSKx)
Address offset: 0x50C + 0x20 * x, (x = 0 to 11)
Reset value: 0x0000 0000
This register reflects the mask for each channel status described in the previous section.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

DTERR
M

FRM
ORM

ACKM

NAKM

STALL
M

CHHM

XFRC
M

rw

rw

rw

rw

rw

rw

rw

Res.

Res.

<!-- pagebreak -->

