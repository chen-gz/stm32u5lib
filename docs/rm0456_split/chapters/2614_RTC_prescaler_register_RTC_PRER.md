•

In automatic end mode (AUTOEND = 1), a NACK and a STOP are automatically sent
after the last received byte.

•

In software end mode (AUTOEND = 0), a NACK is automatically sent after the last
received byte. The TC flag is set and the SCL line is stretched low in order to allow
software actions:
–

A RESTART condition can be requested by setting the START bit of the I2C_CR2
register, with the proper target address configuration and the number of bytes to
transfer. Setting the START bit clears the TC flag and sends the START condition
and the target address on the bus.

–

A STOP condition can be requested by setting the STOP bit of the I2C_CR2
register. This clears the TC flag and sends a STOP condition on the bus.

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 799. Transfer sequence flow for I2C controller receiver, N ≤ 255 bytes
Controller
reception

Controller initialization

NBYTES = N
AUTOEND = 0 for RESTART; 1 for STOP
Configure target address
Set I2C_CR2.START

I2C_ISR.RXNE
=1?

No

Yes
Read I2C_RXDR

NBYTES
received?

No

Yes

I2C_ISR.TC =
1?
No

Yes

Set I2C_CR2.START with
target addess NBYTES
...

End

MSv19863V3

RM0456 Rev 6

<!-- pagebreak -->

