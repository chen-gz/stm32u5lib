1921

True random number generator (RNG)

RM0456

3. The noise source sampling must be 48 MHz or less. Hence, if the RNG clock is different from 48 MHz, this value of CLKDIV
must be adapted. See the CLKDIV bitfield description in Section 48.7.1 for details.
4. This value can be fixed in the RNG driver (it does not depend upon the STM32 product).

Table 466. Configuration selection
Section criteria

Config A

Config B

Config C

Suitable to generate NIST
compliant cryptographic keys

Yes

Entropy(1)

Certified

Good

Very good

Speed(2)

Baseline

Faster

Baseline

No

1. For configurations B and C entropy is verified using AIS-31 test suite (T0 to T8).
2. When speed is not enough for application a NIST compliant DRBG can be used to increase throughput.

For details on data collection and the running of statistical test suites refer to AN4230
“STM32 microcontrollers random number generation validation using NIST statistical test
suite” available from www.st.com.
In bypass mode the bits [31:30] of the fourth word are always stuck at 0. Hence the
continuous capture of samples is started from the fifth word.

48.7

RNG registers
The RNG is associated with a control register, a data register and a status register.

48.7.1

RNG control register (RNG_CR)
Address offset: 0x000
Reset value: 0x0080 0D00

31

30

CONFI COND
GLOCK RST
rs

rw

15

14

29

28

27

26

Res.

Res.

Res.

Res.

13

12

11

10

RNG_CONFIG2[2:0]
rw

rw

rw

NISTC
rw

25

24

rw

22

21

20

19

RNG_CONFIG1[5:0]

18

17

CLKDIV[3:0]

rw

rw

rw

rw

rw

rw

rw

rw

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

ARDIS

Res.

CED

Res.

IE

RNGEN

Res.

Res.

rw

rw

rw

rw

rw

rw

Bit 31 CONFIGLOCK: RNG Config lock
0: Writes to the RNG_NSCR, RNG_HTCR and RNG_CR configuration bits [29:4] are
allowed.
1: Writes to the RNG_NSCR, RNG_HTCR and RNG_CR configuration bits [29:4] are
ignored until the next RNG reset.
Once set, this bit can only be cleared when RNG is reset (set once bit).

<!-- pagebreak -->

16

rw

RNG_CONFIG3[3:0]
rw

23

RM0456 Rev 6

rw

RM0456

True random number generator (RNG)

Bit 30 CONDRST: Conditioning soft reset
Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new
RNG initialization process, with RNG_SR cleared. Registers RNG_CR, RNG_NSCR and
RNG_HTCR are not changed by CONDRST.
This bit must be set to 1 in the same access that set any configuration bits [29:4]. In other
words, when CONDRST bit is set to 1 correct configuration in bits [29:4] must also be
written.
When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is
done. It takes about 2 AHB clock cycles + 2 RNG clock cycles.
Bits 29:26 Reserved, must be kept at reset value.
Bits 25:20 RNG_CONFIG1[5:0]: RNG configuration 1
Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended
value documented in Section 48.6: RNG entropy source validation.
Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in
the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if
CONFIGLOCK = 1.
Bits 19:16 CLKDIV[3:0]: Clock divider factor
This value used to configure an internal programmable divider (from 1 to 16) acting on the
incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0).
0x0: internal RNG clock after divider is similar to incoming RNG clock.
0x1: two RNG clock cycles per internal RNG clock.
0x2: 22 (= 4) RNG clock cycles per internal RNG clock.
...
0xF: 215 RNG clock cycles per internal clock (for example. an incoming 48 MHz RNG clock
becomes a 1.5 kHz internal RNG clock)
Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same
access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.
Bits 15:13 RNG_CONFIG2[2:0]: RNG configuration 2
Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power
consumption is critical. See Section 48.3.8: RNG low-power use. Refer to the
RNG_CONFIG1 bitfield for details.
Bit 12 NISTC: NIST custom
0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output
two conditioning loops are performed and 256 bits of noise source are used.
1: Custom values for NIST compliant RNG. See Section 48.6: RNG entropy source
validation for recommended configuration.
Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access,
while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.
Bits 11:8 RNG_CONFIG3[3:0]: RNG configuration 3
Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details.
If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by
RNG.
Bit 7 ARDIS: Auto reset disable
Set this bit to deactivate the auto-reset feature.
0: Auto-reset enabled
1: Auto-reset disabled
Keeping the auto-reset enabled (automatic clearance of the SECS bit) simplifies the
management of noise source errors, as described in Section 48.3.7: Error management.
Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access,
while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.

RM0456 Rev 6

<!-- pagebreak -->

