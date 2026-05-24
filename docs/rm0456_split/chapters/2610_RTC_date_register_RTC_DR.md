RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 796. Transfer sequence flow for I2C controller transmitter, N ≤ 255 bytes
Controller
transmission

Controller initialization

NBYTES = N
AUTOEND = 0 for RESTART; 1 for STOP
Configure target address
Set I2C_CR2.START

No
I2C_ISR.TXIS
=1?

I2C_ISR.NACKF =
1?

No

Yes

Yes

Write I2C_TXDR

End

NBYTES
transmitted?

No

Yes

I2C_ISR.TC =
1?
No

Yes

Set I2C_CR2.START with
target address NBYTES
...

End

MSv19860V3

RM0456 Rev 6

<!-- pagebreak -->

