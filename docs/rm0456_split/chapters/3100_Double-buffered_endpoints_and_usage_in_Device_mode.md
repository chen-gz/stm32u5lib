Res.

Res.

BBERR TXERR
M
M
rw

rw

RM0456 Rev 6

Res.

Res.

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 DTERRM: Data toggle error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 9 FRMORM: Frame overrun mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 8 BBERRM: Babble error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 7 TXERRM: Transaction error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 6 Reserved, must be kept at reset value.
Bit 5 ACKM: ACK response received/transmitted interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 4 NAKM: NAK response received interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 3 STALLM: STALL response received interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 2 Reserved, must be kept at reset value.
Bit 1 CHHM: Channel halted mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed mask
0: Masked interrupt
1: Unmasked interrupt

72.15.31 OTG host channel x transfer size register (OTG_HCTSIZx)
Address offset: 0x510 + 0x20 * x, (x = 0 to 11)
Reset value: 0x0000 0000
31

30

DO
PNG

29

DPID[1:0]

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

PKTCNT[9:0]

17

16

XFRSIZ[18:16]

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

rw

rw

XFRSIZ[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 31 DOPNG: Do Ping
This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING
protocol.
Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the
channel.
Bits 30:29 DPID[1:0]: Data PID
The application programs this field with the type of PID to use for the initial transaction. The
host maintains this field for the rest of the transfer.
00: DATA0
10: DATA1
11: SETUP (control) / reserved (non-control)
Bits 28:19 PKTCNT[9:0]: Packet count
This field is programmed by the application with the expected number of packets to be
transmitted (OUT) or received (IN).
The host decrements this count on every successful transmission or reception of an OUT/IN
packet. Once this count reaches zero, the application is interrupted to indicate normal
completion.
Bits 18:0 XFRSIZ[18:0]: Transfer size
For an OUT, this field is the number of data bytes the host sends during the transfer.
For an IN, this field is the buffer size that the application has reserved for the transfer. The
application is expected to program this field as an integer multiple of the maximum packet
size for IN transactions (periodic and non-periodic).

72.15.32 Device-mode registers
These registers must be programmed every time the core changes to device mode

72.15.33 OTG device configuration register (OTG_DCFG)
Address offset: 0x800
Reset value: 0x0220 0000
This register configures the core in device mode after power-on or after certain control
commands or enumeration. Do not make changes to this register after initial programming.
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

ERRAT
IM

Res.

Res.

PFIVL[1:0]

Res.

NZLSO
HSK

rw

<!-- pagebreak -->

