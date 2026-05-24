RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bit 5 INEPNM: IN token received with EP mismatch
Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other
than the one for which the IN token was received. This interrupt is asserted on the endpoint
for which the IN token was received.
Bit 4 ITTXFE: IN token received when Tx FIFO is empty
Indicates that an IN token was received when the associated Tx FIFO (periodic/nonperiodic) was empty. This interrupt is asserted on the endpoint for which the IN token was
received.
Bit 3 TOC: Timeout condition
Indicates that the core has detected a timeout condition on the USB for the last IN token on
this endpoint.
Bit 2 Reserved, must be kept at reset value.
Bit 1 EPDISD: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application’s request.
Bit 0 XFRC: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the
USB, for this endpoint.

72.15.46 OTG device IN endpoint 0 transfer size register
(OTG_DIEPTSIZ0)
Address offset: 0x910
Reset value: 0x0000 0000
The application must modify this register before enabling endpoint 0. Once endpoint 0 is
enabled using the endpoint enable bit in the device control endpoint 0 control registers
(EPENA in OTG_DIEPCTL0), the core modifies this register. The application can only read
this register once the core has cleared the endpoint enable bit.
Nonzero endpoints use the registers for endpoints 1–3.
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

PKTCNT[1:0]

Res.

Res.

Res.

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

rw

rw

4

3
XFRSIZ[6:0]

rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

