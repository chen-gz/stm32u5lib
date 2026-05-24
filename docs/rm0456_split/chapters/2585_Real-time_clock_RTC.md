2756

Inter-integrated circuit interface (I2C)

65.4

RM0456

I2C functional description
In addition to receiving and transmitting data, the peripheral converts them from serial to
parallel format and vice versa. The interrupts are enabled or disabled by software. The
peripheral is connected to the I²C-bus through a data pin (SDA) and a clock pin (SCL). It
supports Standard-mode (up to 100 kHz), Fast-mode (up to 400 kHz), and Fast-mode Plus
(up to 1 MHz) I²C-bus.
The peripheral can also be connected to an SMBus, through the data pin (SDA), the clock
pin (SCL), and an optional SMBus alert pin (SMBA).
The independent clock function allows the I2C communication speed to be independent of
the i2c_pclk frequency.

<!-- pagebreak -->

