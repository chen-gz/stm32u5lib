3134

Universal serial bus full-speed host/device interface (USB)

71.7.1

RM0456

Channel/endpoint transmit buffer descriptor n
(USB_CHEP_TXRXBD_n)
Address offset: 0x0 + 0x8 * n, (n = 0 to 7)
Reset value: 0xXXXX XXXX
This register description applies when corresponding CHEPnR register does not program
the use of double buffering working in receive mode (otherwise refer to following register
description)

31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

20

19

18

17

16

rw

rw

rw

rw

rw

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

COUNT_TX[9:0]

ADDR_TX[15:0]
rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:16 COUNT_TX[9:0]: Transmission byte count
These bits contain the number of bytes to be transmitted by the endpoint/channel associated
with the USB_CHEPnR register at the next IN token addressed to it.
Bits 15:0 ADDR_TX[15:0]: Transmission buffer address
These bits point to the starting address of the packet buffer containing data to be transmitted
by the endpoint/channel associated with the USB_CHEPnR register at the next IN token
addressed to it. Bits 1 and 0 must always be written as “00” since packet memory is word
wide and all packet buffers must be word aligned.

71.7.2

Channel/endpoint receive buffer descriptor n [alternate]
(USB_CHEP_TXRXBD_n)
Address offset: 0x0 + 0x8 * n, (n = 0 to 7)
Reset value: 0xXXXX XXXX
This register description applies when corresponding CHEPnR register programs the use of
double buffering and activates receive buffers (otherwise refer to previous register
description).
This table location is used to store two different values, both required during packet
reception. The most significant bits contains the definition of allocated buffer size, to allow
buffer overflow detection, while the least significant part of this location is written back by the
USB peripheral at the end of reception to give the actual number of received bytes. Due to
the restrictions on the number of available bits, buffer size is represented using the number
of allocated memory blocks, where block size can be selected to choose the trade-off
between fine-granularity/small-buffer and coarse-granularity/large-buffer. The size of
allocated buffer is a part of the endpoint/channel descriptor and it is normally defined during
the enumeration process according to its maxPacketSize parameter value (see “Universal
Serial Bus Specification”).

31

30

BLSIZE
rw

29

28

27

26

25

24

23

22

NUM_BLOCK[4:0]
rw

<!-- pagebreak -->

