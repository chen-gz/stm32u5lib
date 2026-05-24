3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.39 OTG all endpoints interrupt mask register
(OTG_DAINTMSK)
Address offset: 0x81C
Reset value: 0x0000 0000
The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt
the application when an event occurs on a device endpoint. However, the OTG_DAINT
register bit corresponding to that interrupt is still set.
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

OEPM[15:0]
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

IEPM[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 OEPM[15:0]: OUT EP interrupt mask bits
One per OUT endpoint:
Bit 16 for OUT EP 0, bit 19 for OUT EP 3
0: Masked interrupt
1: Unmasked interrupt
Bits 15:0 IEPM[15:0]: IN EP interrupt mask bits
One bit per IN endpoint:
Bit 0 for IN EP 0, bit 3 for IN EP 3
0: Masked interrupt
1: Unmasked interrupt

72.15.40 OTG device VBUS discharge time register
(OTG_DVBUSDIS)
Address offset: 0x0828
Reset value: 0x0000 17D7
This register specifies the VBUS discharge time after VBUS pulsing during SRP.
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

VBUSDT[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 VBUSDT[15:0]: Device VBUS discharge time
Specifies the VBUS discharge time after VBUS pulsing during SRP. This value equals:
VBUS discharge time in PHY clocks / 1 024
Depending on VBUS load, this value may need adjusting.

72.15.41 OTG device VBUS pulsing time register
(OTG_DVBUSPULSE)
Address offset: 0x082C
Reset value: 0x0000 05B8
This register specifies the VBUS pulsing time during SRP.
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

DVBUSP[15:0]
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
Bits 15:0 DVBUSP[15:0]: Device VBUS pulsing time. This feature is only relevant to OTG1.3.
Specifies the VBUS pulsing time during SRP. This value equals:
VBUS pulsing time in PHY clocks / 1 024

72.15.42 OTG device IN endpoint FIFO empty interrupt mask register
(OTG_DIEPEMPMSK)
Address offset: 0x834
Reset value: 0x0000 0000
This register is used to control the IN endpoint FIFO empty interrupt generation
(TXFE_OTG_DIEPINTx).
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

INEPTXFEM[15:0]
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

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 INEPTXFEM[15:0]: IN EP Tx FIFO empty interrupt mask bits
These bits act as mask bits for OTG_DIEPINTx.
TXFE interrupt one bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for IN endpoint 3
0: Masked interrupt
1: Unmasked interrupt

72.15.43 OTG device control IN endpoint 0 control register
(OTG_DIEPCTL0)
Address offset: 0x900
Reset value: 0x0000 0000
This section describes the OTG_DIEPCTL0 register for USB_OTG FS. Nonzero control
endpoints use registers for endpoints 1–3.
31

30

EPENA EPDIS

29

28

27

26

25

24

23

22

Res.

Res.

SNAK

CNAK

w

w

rw

rw

rw

rw

rs

TXFNUM[3:0]

21

20

19

18

17

16

STALL

Res.

EPTYP[1:0]

NAK
STS

Res.

rs

rs

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

USBA
EP

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

r

r

r

4

3

2

1

Res.

Res.

Res.

r

0

MPSIZ[1:0]
rw

rw

Bit 31 EPENA: Endpoint enable
The application sets this bit to start transmitting data on the endpoint 0.
The core clears this bit before setting any of the following interrupts on this endpoint:
– Endpoint disabled
– Transfer completed
Bit 30 EPDIS: Endpoint disable
The application sets this bit to stop transmitting data on an endpoint, even before the
transfer for that endpoint is complete. The application must wait for the endpoint disabled
interrupt before treating the endpoint as disabled. The core clears this bit before setting the
endpoint disabled interrupt. The application must set this bit only if endpoint enable is
already set for this endpoint.
Bits 29:28 Reserved, must be kept at reset value.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on
that endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 TXFNUM[3:0]: Tx FIFO number
This value is set to the FIFO number that is assigned to IN endpoint 0.

<!-- pagebreak -->

