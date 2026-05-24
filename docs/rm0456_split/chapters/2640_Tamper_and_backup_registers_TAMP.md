RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Bits 9:0 SADD[9:0]: Target address (controller mode)
Condition: In 7-bit addressing mode (ADD10 = 0):
SADD[7:1] must be written with the 7-bit target address to be sent. Bits SADD[9], SADD[8]
and SADD[0] are don't care.
Condition: In 10-bit addressing mode (ADD10 = 1):
SADD[9:0] must be written with the 10-bit target address to be sent.
Note: Changing these bits when the START bit is set is not allowed.

65.9.3

I2C own address 1 register (I2C_OAR1)
Address offset: 0x08
Reset value: 0x0000 0000
Access: no wait states, except if a write access occurs while a write access is ongoing. In
this case, wait states are inserted in the second write access until the previous one is
completed. The latency of the second write access can be up to 2 x i2c_pclk +
6 x i2c_ker_ck.

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

OA1M
ODE

rw

rw

rw

rw

OA1EN

Res.

Res.

Res.

rw

rw

OA1[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 OA1EN: Own address 1 enable
0: Own address 1 disabled. The received target address OA1 is NACKed.
1: Own address 1 enabled. The received target address OA1 is ACKed.
Bits 14:11 Reserved, must be kept at reset value.
Bit 10 OA1MODE: Own address 1 10-bit mode
0: Own address 1 is a 7-bit address.
1: Own address 1 is a 10-bit address.
Note: This bit can be written only when OA1EN = 0.
Bits 9:0 OA1[9:0]: Interface own target address
7-bit addressing mode: OA1[7:1] contains the 7-bit own target address. Bits OA1[9], OA1[8]
and OA1[0] are don't care.
10-bit addressing mode: OA1[9:0] contains the 10-bit own target address.
Note: These bits can be written only when OA1EN = 0.

65.9.4

I2C own address 2 register (I2C_OAR2)
Address offset: 0x0C
Reset value: 0x0000 0000
Access: no wait states, except if a write access occurs while a write access is ongoing. In
this case, wait states are inserted in the second write access, until the previous one is

RM0456 Rev 6

<!-- pagebreak -->

