609

Reset and clock control (RCC)

RM0456

Bits 19:15 MSICAL0[4:0]: MSIRC0 clock calibration for MSI ranges 0 to 3
These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim
value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of
MSITRIM0[4:0] and the factory-programmed calibration trim value MSIRC0[4:0].
Caution: There is no hardware protection to limit a potential overflow due to the addition of
MSITRIM bitfield and factory program bitfield for this calibration value. Control must
be managed by software at user level.
Bits 14:10 MSICAL1[4:0]: MSIRC1 clock calibration for MSI ranges 4 to 7
These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim
value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of
MSITRIM1[4:0] and the factory calibration trim value MSIRC1[4:0].
Caution: There is no hardware protection to limit a potential overflow due to the addition of
MSITRIM bitfield and factory program bitfield for this calibration value. Control must
be managed by software at user level.
Bits 9:5 MSICAL2[4:0]: MSIRC2 clock calibration for MSI ranges 8 to 11
These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim
value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of
MSITRIM2[4:0] and the factory calibration trim value MSIRC2[4:0].
Caution: There is no hardware protection to limit a potential overflow due to the addition of
MSITRIM bitfield and factory program bitfield for this calibration value. Control must
be managed by software at user level.
Bits 4:0 MSICAL3[4:0]: MSIRC3 clock calibration for MSI ranges 12 to 15
These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim
value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of
MSITRIM3[4:0] and the factory calibration trim value MSIRC3[4:0].
Caution: There is no hardware protection to limit a potential overflow due to the addition of
MSITRIM bitfield and factory program bitfield for this calibration value. Control must
be managed by software at user level.

11.8.3

RCC internal clock sources calibration register 2 (RCC_ICSCR2)
Address offset: 0x00C
Reset value: 0x0008 4210
Access: no wait state; word, half-word, and byte access

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

MSITRI
M0[0]
rw

MSITRIM1[4:0]
rw

rw

rw

19

rw

rw

rw

rw

17

16

rw

rw

rw

rw

3

2

1

0

rw

rw

MSITRIM2[4:0]
rw

18

MSITRIM0[4:1]

MSITRIM3[4:0]
rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:15 MSITRIM0[4:0]: MSI clock trimming for ranges 0 to 3
These bits provide an additional user-programmable trimming value that is added to the
factory-programmed calibration trim value MSIRC0[4:0] bits. It can be programmed to adjust
to voltage and temperature variations that influence the frequency of the MSI.

<!-- pagebreak -->

