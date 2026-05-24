RM0456 Rev 6

RM0456

Parallel synchronous slave interface (PSSI)

42.5.6

PSSI interrupt clear register (PSSI_ICR)
Address offset: 0x14
Reset value: 0x0000 0000
The PSSI_ICR register is write-only. Writing a 1 into a bit of this register clears the
corresponding bit in the PSSI_RIS and PSSI_MIS registers. Writing a 0 has no effect.
Reading this register always gives zeros.

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

OVR_I
SC

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

w

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 OVR_ISC: Data buffer overrun/underrun interrupt status clear
Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS.
Bit 0 Reserved, must be kept at reset value.

42.5.7

PSSI data register (PSSI_DR)
Address offset: 0x28
Reset value: 0x0000 0000
In receive mode (OUTEN = 0), the DMA controller must read the received data from this
register. Write operations to PSSI_DR result in an error response. When more bytes than
the number of valid bytes are read in the FIFO, the invalid bytes return zeros.
In transmit mode (OUTEN = 1), the DMA controller must write the data to be transmitted into
this register. Read operations to PSSI_DR result in an error response.
32-bit, 16-bit, and 8-bit accesses are all supported for PSSI_DR. For instance, 16-bit
read/write operations remove/add two bytes from/to the FIFO. However, 8-bit accesses are
permitted only when the PSSI is configured to transfer 8 data bits at a time (EDM=00 in
PSSI_CR). 8-bit accesses to PSSI_DR when EDM is not set to 0 result in an error response.
All accesses must include byte 0: 8-bit accesses must be performed to bits 7 to 0 and 16-bit
accesses from bits 15 to 0. Accesses that do not include byte 0 results in an error response.
Accessing PSSI_DR when ENABLE bit in PSSI_CR is set to 0 results in an error response.

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

BYTE3[7:0]
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

BYTE1[7:0]
rw

rw

rw

rw

19

18

17

16

BYTE2[7:0]

rw

rw

20

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

BYTE0[7:0]
rw

rw

rw

rw

RM0456 Rev 6

rw

rw

rw

rw

<!-- pagebreak -->

