3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Table 741. Reception status encoding
STATRX[1:0]

Meaning

00

DISABLED: all reception requests addressed to this endpoint/channel are ignored.

01

STALL:
Device mode: the endpoint is stalled and all reception requests result in a STALL
handshake.
Host mode: this indicates that the device has STALLed the channel.

10

NAK:
Device mode: the endpoint is NAKed and all reception requests result in a NAK
handshake.
Host mode: this indicates that the device has NAKed the reception request.

11

VALID: this endpoint/channel is enabled for reception.

Table 742. Endpoint/channel type encoding
UTYPE[1:0]

Meaning

00

BULK

01

CONTROL

10

ISO

11

INTERRUPT

Table 743. Endpoint/channel kind meaning
UTYPE[1:0]

EPKIND meaning

00

BULK

DBL_BUF

01

CONTROL

STATUS_OUT

10

ISO

SBUF_ISO: This bit is set by the software to enable the
single-buffering feature for isochronous endpoint

11

INTERRUPT

Not used

Table 744. Transmission status encoding
STATTX[1:0]

<!-- pagebreak -->

Meaning

00

DISABLED: all transmission requests addressed to this endpoint/channel are
ignored.

01

STALL:
Device mode: the endpoint is stalled and all transmission requests result in a
STALL handshake.
Host mode: this indicates that the device has STALLed the channel.

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
Table 744. Transmission status encoding (continued)
STATTX[1:0]

Meaning

10

NAK:
Device mode: the endpoint is NAKed and all transmission requests result in a
NAK handshake.
Host mode: this indicates that the device has NAKed the transmission request.

11

VALID: this endpoint/channel is enabled for transmission.

RM0456 Rev 6

<!-- pagebreak -->

