3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.35 OTG device status register (OTG_DSTS)
Address offset: 0x808
Reset value: 0x0000 0010
This register indicates the status of the core with respect to USB-related events. It must be
read on interrupts from the device all interrupts (OTG_DAINT) register.
31

30

29

28

27

26

25

24

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

r

r

r

r

r

21

20

19

18

17

16

r

FNSOF[13:8]

r

r

r

r

r

r

r

7

6

5

4

3

2

1

Res.
r

22

DEVLNSTS[1:0]

8

FNSOF[7:0]
r

23

Res.

r

Res.

Res.

EERR
r

ENUMSPD[1:0]
r

0
SUSP
STS

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:22 DEVLNSTS[1:0]: Device line status
Indicates the current logic level USB data lines.
Bit [23]: Logic level of D+
Bit [22]: Logic level of DBits 21:8 FNSOF[13:0]: Frame number of the received SOF
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 EERR: Erratic error
The core sets this bit to report any erratic errors.
Due to erratic errors, the OTG_FS controller goes into suspended state and an interrupt is
generated to the application with Early suspend bit of the OTG_GINTSTS register (ESUSP
bit in OTG_GINTSTS). If the early suspend is asserted due to an erratic error, the application
can only perform a soft disconnect recover.
Bits 2:1 ENUMSPD[1:0]: Enumerated speed
Indicates the speed at which the OTG_FS controller has come up after speed detection
through a chirp sequence.
11: Full speed using embedded FS PHY
Others: reserved
Bit 0 SUSPSTS: Suspend status
In device mode, this bit is set as long as a suspend condition is detected on the USB. The
core enters the suspended state when there is no activity on the USB data lines for a period
of 3 ms. The core comes out of the suspend:
– When there is an activity on the USB data lines
– When the application writes to the remote wake-up signaling bit in the OTG_DCTL register
(RWUSIG bit in OTG_DCTL).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.36 OTG device IN endpoint common interrupt mask register
(OTG_DIEPMSK)
Address offset: 0x810
Reset value: 0x0000 0000
This register works with each of the OTG_DIEPINTx registers for all endpoints to generate
an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the
OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register.
Status bits are masked by default.
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

Res.

NAKM

Res.

Res.

Res.

Res.

Res.

Res.

INEPN
EM

TOM

Res.

EPDM

XFRC
M

rw

rw

rw

rw

INEPN ITTXFE
MM
MSK
rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 NAKM: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 12:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 Reserved, must be kept at reset value.
Bit 6 INEPNEM: IN endpoint NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Bit 5 INEPNMM: IN token received with EP mismatch mask
0: Masked interrupt
1: Unmasked interrupt
Bit 4 ITTXFEMSK: IN token received when Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 3 TOM: Timeout condition mask (Non-isochronous endpoints)
0: Masked interrupt
1: Unmasked interrupt

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 2 Reserved, must be kept at reset value.
Bit 1 EPDM: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

72.15.37 OTG device OUT endpoint common interrupt mask register
(OTG_DOEPMSK)
Address offset: 0x814
Reset value: 0x0000 0000
This register works with each of the OTG_DOEPINTx registers for all endpoints to generate
an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the
OTG_DOEPINTx register can be masked by writing into the corresponding bit in this
register. Status bits are masked by default.
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

OUT
PKT
ERRM

Res.

STS
PHSR
XM

Res.

EPDM

XFRC
M

rw

rw

Res.

Res.

NAK
MSK

BERR
M

rw

rw

Res.

Res.

Res.

rw

OTEPD
STUPM
M

rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 NAKMSK: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 12 BERRM: Babble error interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 OUTPKTERRM: Out packet error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 7 Reserved, must be kept at reset value.
Bit 6 Reserved, must be kept at reset value.
Bit 5 STSPHSRXM: Status phase received for control write mask
0: Masked interrupt
1: Unmasked interrupt

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

USB on-the-go full-speed (OTG_FS)

Bit 4 OTEPDM: OUT token received when endpoint disabled mask. Applies to control OUT
endpoints only.
0: Masked interrupt
1: Unmasked interrupt
Bit 3 STUPM: STUPM: SETUP phase done mask. Applies to control endpoints only.
0: Masked interrupt
1: Unmasked interrupt
Bit 2 Reserved, must be kept at reset value.
Bit 1 EPDM: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

72.15.38 OTG device all endpoints interrupt register (OTG_DAINT)
Address offset: 0x818
Reset value: 0x0000 0000
When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the
application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit
of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There
is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits
for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits
are used. Bits in this register are set and cleared when the application sets and clears bits in
the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).
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

OEPINT[15:0]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

IEPINT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 OEPINT[15:0]: OUT endpoint interrupt bits
One bit per OUT endpoint:
Bit 16 for OUT endpoint 0, bit 19 for OUT endpoint 3.
Bits 15:0 IEPINT[15:0]: IN endpoint interrupt bits
One bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for endpoint 3.

RM0456 Rev 6

<!-- pagebreak -->

