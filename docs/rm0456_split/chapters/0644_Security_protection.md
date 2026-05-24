647

Low-power general-purpose I/Os (LPGPIO)

RM0456

If there is an attempt to set and reset bits of the same index, the set action takes the priority.
Writing LPGPIO_BSRR register does not lock the LPGPIO_ODR bits, that can be anyway
accessed directly. LPGPIO_BSRR provides a way to perform atomic bitwise handling.
The 16-bit LPGPIO_BRR allows individual bit reset. It is the same as LPGPIO_BSRR but
with minimal pattern preparation:
•

14.3.5

LPGPIO_BRR(i): when writing 1 to it, this bit resets the LPGPIO_ODR(i) bit.

Security protection
The LPGPIO includes a security mechanism, that allows or locks the access to the I/O
configuration and data registers. This system is used to protect the I/O against the data
corruption or observation.
The security mechanism within the LPGPIO is directly issued from GPIOx_SECCFGR.
Therefore, no additional configuration is required.
The LPGPIO security means the following:
•

When the executed code is secure, all bits can be accessed.

•

When the executed code is nonsecure, only the bits concerning the nonsecure I/Os
can be accessed.

A nonsecure access to a secure I/O register bit is silent fail:

14.3.6

•

A nonsecure write to a secure I/O register bit is ignored (WI).

•

A nonsecure read to a secure I/O register bit returns 0 (RAZ).

•

No bus error is generated.

Secure clock and reset management
The LPGPIO clock and reset control bits in the RCC are automatically configured
as secured as soon as at least one I/O with LPGPIO alternate function is secure (refer to the
corresponding I/Os in GPIO_SECCFGR registers).

14.4

LPGPIO registers
This section gives a detailed description of the LPGPIO registers.
The peripheral registers can be written in word, half-word or byte mode.

14.4.1

LPGPIO port mode register (LPGPIO_MODER)
Address offset:0x00
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

MODE
15

MODE
14

MODE
13

MODE
12

MODE
11

MODE
10

MODE
9

MODE
8

MODE
7

MODE
6

MODE
5

MODE
4

MODE
3

MODE
2

MODE
1

MODE
0

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

<!-- pagebreak -->

