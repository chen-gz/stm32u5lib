RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

68.8.13

SPI receiver CRC register (SPI_RXCRC)
Address offset: 0x048
Reset value: 0x0000 0000

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

RXCRC[31:16]
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

r

r

r

r

r

r

r

RXCRC[15:0]
r

r

Bits 31:0 RXCRC[31:0]: CRC register for receiver
When CRC calculation is enabled, the RXCRC[31:0] bits contain the computed CRC value
of the subsequently received bytes. CRC calculation is initialized when the CRCEN bit of
the SPI_CR1 register is set or when a data block is transferred completely. The CRC is
calculated serially using the polynomial programmed in the SPI_CRCPOLY register.
The number of bits considered at calculation depends on the SPI_CRCPOLY register and
CRCSIZE bits settings in the SPI_CFG1 register.
Note: A read to this register when the communication is ongoing may return an incorrect
value.
Note: RXCRC[31-16] bits are reserved for the peripheral instances with data size limited to
16 bits. There is no constrain when 32-bit access is applied at these addresses.
Reserved bits 31-16 are always read zero while any write to them is ignored.
Note: The configuration of CRCSIZE bit field is not taken into account when the content of
this register is read by software. No masking is applied for unused bits in this case.

68.8.14

SPI underrun data register (SPI_UDRDR)
Address offset: 0x04C
Reset value: 0x0000 0000
The content of this register is write protected when SPI is enabled.

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

UDRDR[31:16]
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

UDRDR[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 UDRDR[31:0]: data at slave underrun condition
The register is taken into account in slave mode and at underrun condition only. The number
of bits considered depends on the DSIZE bit setting in the SPI_CFG1 register. Underrun
condition handling depends on the UDRCFG bit setting in the SPI_CFG1 register.
Note: UDRDR[31-16] bits are reserved for the peripheral instances with data size limited to 16
bits. There is no constraint when 32-bit access is applied at these addresses. Reserved
bits 31-16 are always read zero while any write to them is ignored.

RM0456 Rev 6

<!-- pagebreak -->

