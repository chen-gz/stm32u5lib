275

Global TrustZone controller (GTZC)

5.6.8

RM0456

GTZC1 TZSC memory x sub-region z watermark configuration
register (GTZC1_TZSC_MPCWMxzCFGR) (z = A to B)
Address offset: Block A: 0x40 + 0x10 * (x - 1) (x = 1 to 6)
Address offset: Block B: 0x48 + 0x10 * (x - 1) (x = 1, 2, 5, 6)
Reset value: 0x0000 0000
Secure privilege access only.

Note:

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.

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

SRLOC
K

SREN

rs

rw

Res.

Res.

Res.

Res.

Res.

Res.

PRIV

SEC

rw

rw

Res.

Res.

Res.

Res.

Res.

Bits 31:10 Reserved, must be kept at reset value.
Bit 9 PRIV: Privileged sub-region z of base region x
This bit is taken into account only if SREN is set.
0: Privileged and unprivileged accesses are granted in sub-region z.
1: Only privileged accesses are granted in sub-region z of region x.
Bit 8 SEC: Secure sub-region z of base region x
This bit is taken into account only if SREN is set.
0: Only nonsecure data accesses are granted to sub-region z of region x.
1: Only secure data accesses are granted to sub-region z of region x.
Bits 7:2 Reserved, must be kept at reset value.
Bit 1 SRLOCK: Sub-region z lock
This bit, once set, can be cleared only by a system reset.
0: GTZC1_TZSC_MPCWMxCFGR, GTZC1_TZSC_MPCWMxAR and
GTZC1_TZSC_MPCWMxBR can be written.
1: Writes to GTZC1_TZSC_MPCWMxCFGR, GTZC1_TZSC_MPCWMxAR and
GTZC1_TZSC_MPCWMxBR are ignored.
Bit 0 SREN: Sub-region z enable
0: Sub-region z is disabled. Access control of base region x applies to any access between
this sub-region start- and end-addresses.
1: Sub-region z of region x is enabled. Access control defined in
GTZC1_TZSC_MPCWMx_CFGR applies to any access between this sub-region start- and
end-addresses, both defined in GTZC1_TZSC_MPCWMxAR and
GTZC1_TZSC_MPCWMxBR.
Note: External memories that are watermark controlled start fully nonsecure/unprivileged at
reset when TZEN = 0. When TZEN = 1, external memories start fully secure/fully
privileged (inverted reset-value).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

5.6.9

GTZC1 TZSC memory x sub-region A watermark register
(GTZC1_TZSC_MPCWMxAR)
Address offset: 0x44 + 0x10 * (x - 1) (x = 1 to 6)
Reset value: 0x0000 0000
Secure privilege access only.
When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the
memory, a saturation of SUBA_LENGTH is applied automatically.
When an overlap of sub-region A and B exists, secure/privileged attributes of both
sub-regions apply on the common section (see Section 5.4.3).

Note:

Some registers are only available on some devices in the STM32U5 Series.
Refer to the device datasheet for availability of its associated memory region.

31

30

29

28

Res.

Res.

Res.

Res.

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

SUBA_LENGTH[11:0]
rw

rw

rw

rw

rw

10

9

8

7

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

rw

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

SUBA_START[10:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 SUBA_LENGTH[11:0]: Length of sub-region A in region x
This field defines the length of the sub-region A, to be multiplied by the granularity defined in
Table 31.
When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the
memory, a saturation of SUBA_LENGTH is applied automatically.
If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in
GTZC1_TZSC_MPCMWxACFGR is cleared).
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 SUBA_START[10:0]: Start of sub-region A in region x
This field defines the address offset of the sub-region A, to be multiplied by the granularity
defined in Table 31, versus the start of the region x.
External memories that are watermark controlled, start fully nonsecure at reset when
TZEN = 0. When TZEN = 1, external memories start fully secure (inverted reset-value).

RM0456 Rev 6

<!-- pagebreak -->

