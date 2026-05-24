2687

Tamper and backup registers (TAMP)

RM0456

Table 645. Device resource x tamper protection
Potential tamper
or BKBLOCK

-

Confirmed tamper
or BKERASE

RPCFGx = 0

No effect on device resource x

No effect on device resource x

RPCFGx = 1

Device secret x protected as
described by
tamp_potential_ercfgx(1)

Device secret x protected as
described by
tamp_confirmed_ercfgx(1)

1. Refer to Table 644: TAMP interconnection.

Device secrets access blocked by software
By default, the device secrets can be accessed by the application, except if a tamper event
flag is detected: the device secrets access is not possible as long as a tamper flag is set.
It is possible to block the access to the device secrets by software, by setting the BKBLOCK
bit of the TAMP_CR2 register. The device secrets access is possible only when
BKBLOCK = 0 and no tamper flag is set.

64.4.11

Tamper detection configuration and initialization
Each input can be enabled by setting the corresponding TAMPxE bits to 1 in the TAMP_CR
register.
Each TAMP_INx tamper detection input is associated with a flag TAMPxF in the TAMP_SR
register.
By setting the TAMPxIE bit in the TAMP_IER register, an interrupt is generated when a
tamper detection event occurs (when TAMPxF is set). Setting TAMPxIE is not allowed when
the corresponding TAMPxMSK is set.

Trigger output generation on tamper event
The tamper event detection can be used as trigger input by the low-power timers.
When TAMPxMSK bit is cleared in TAMP_CR register, the TAMPxF flag must be cleared by
software in order to allow a new tamper detection on the same pin.
When TAMPxMSK bit is set, the TAMPxF flag is masked, and kept cleared in TAMP_SR
register. This configuration permits the low-power timers to be triggered automatically in
Stop mode, without requiring the system wake-up to perform the TAMPxF clearing. In this
case, the backup registers are not cleared.
This feature is available only when the tamper is configured in level detection with filtering
mode (TAMPFLT ≠ 00 and active mode is not selected). Refer to Section : Level detection
with filtering on tamper inputs (passive mode).

Timestamp on tamper event
With TAMPTS set to 1 in the RTC_CR, any internal or external tamper event causes a
timestamp to occur. In case a timestamp occurs due to tamper event, either the TSF bit or
the TSOVF bit is set in RTC_SR, in the same manner as if a normal timestamp event
occurs.
Note:

<!-- pagebreak -->

TSF is set up to 3 ck_apre cycles after TAMPxF flags. TSF is not set if RTCCLK is stopped
(it is set when RTCCLK restarts).

RM0456 Rev 6

RM0456
Note:

Tamper and backup registers (TAMP)
If TAMPxF is cleared before the expected rise of TSF, TSF is not set. Consequently, in case
TAMPTS = 1, the software should either wait for timestamp flag before clearing the tamper
flag, or should read the RTC counters values in the TAMP interrupt routine.

Edge detection on tamper inputs (passive mode)
If the TAMPFLT bits are 00, the TAMP_INx pins generate tamper detection events when
either a rising edge or a falling edge is observed depending on the corresponding
TAMPxTRG bit. The internal pull-up resistors on the TAMP_INx inputs are deactivated when
edge detection is selected.
Caution:

When TAMPFLT = 00 and TAMPxTRG = 0 (rising edge detection), a tamper event may be
detected by hardware if the tamper input is already at high level before enabling the tamper
detection.
After a tamper event has been detected and cleared, the TAMP_INx should be disabled and
then re-enabled (TAMPxE set to 1) before re-programming the backup registers
(TAMP_BKPxR). This prevents the application from writing to the backup registers while the
TAMP_INx input value still indicates a tamper detection. This is equivalent to a level
detection on the TAMP_INx input.

Note:

Tamper detection is still active when VDD power is switched off. To avoid unwanted resetting
of the backup registers, the pin to which the TAMPx is mapped should be externally tied to
the correct level.

Level detection with filtering on tamper inputs (passive mode)
Level detection with filtering is performed by setting TAMPFLT to a non-zero value. A tamper
detection event is generated when either 2, 4, or 8 (depending on TAMPFLT) consecutive
samples are observed at the level designated by the TAMPxTRG bits.
The TAMP_INx inputs are precharged through the I/O internal pull-up resistance before its
state is sampled, unless disabled by setting TAMPPUDIS to 1. The duration of the
precharge is determined by the TAMPPRCH bits, allowing for larger capacitances on the
TAMP_INx inputs.
The trade-off between tamper detection latency and power consumption through the pull-up
can be optimized by using TAMPFREQ to determine the frequency of the sampling for level
detection. The TAMP_IN I/O schmitt trigger is enabled only during the precharge duration to
avoid any extra consumption if the tamper switch is open (floating state).

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

RM0456

Figure 776. Tamper sampling with precharge pulse
RTC clock

Sampling
Floating input

Switch opened
Precharge = 1 RTCCLK
Precharge = 2 RTCCLK
Precharge = 4 RTCCLK
Precharge = 8 RTCCLK (not shown)
MSv30115V2

Figure 777. Low level detection with precharge and filtering
Sampling

Sampling

Sampling

Sampling

§

RTCCLK

§

§

TAMPxF

Internal pull-up enable
(TAMPPUDIS=0)

TAMP_INx
(TAMPPUDIS=0))

Schmitt trigger enable
TAMPPRCH

TAMPPRCH
TAMPFREQ

TAMP_INx floating

TAMPPRCH
TAMPFREQ

TAMP_INx floating

TAMP_INx tied to 0

Configuration:
TAMPxTRG=0: Low level detection
TAMPPRCH=1: 1 RTCCLK cycle pre-charge duration (internal pull-up is applied)
TAMPFLT=1: Tamper event is activated after 2 consecutive samples at active level

Note:

TAMPPRCH
TAMPFREQ
TAMP_INx tied to 0

MSv74148V1

Refer to the microcontroller datasheet for the electrical characteristics of the pull-up
resistors.

Active tamper detection
When the TAMPxAM bit is set in the TAMP_ATCR, the tamper events are configured in
active mode, which is based on a comparison between a TAMP_OUTy pin and a TAMP_INx
pin. By default (ATOSHARE = 0) the comparison is made between TAMP_INx and
TAMP_OUTx (y = x). When ATOSHARE bit is set, the same output can be used for several
tamper inputs. The TAMP_OUTy function is enabled on the I/O as soon as it is selected for
comparison with an active tamper input TAMP_INx (TAMPxEN = TAMPxAM = 1), thanks to

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)
ATOSHARE and ATOSELx bits. Refer to ATOSHARE and ATOSEL bits descriptions in the
TAMP_ATCRx (x = 1, 2) registers.
Every two CK_ATPER cycles (CK_ATPER = 2ATPER × CK_ATPRE), TAMP_OUTy output pin
provides a value provided by a pseudo random number generator (PRNG). After outputting
this value, the TAMP_OUTy pin outputs its opposite value one CK_ATPER cycle after.
Table 646. Active tamper output change period

ATCKSEL[3:0]

0x0

…

0x7

0xB(2)

CK_ATPRE
frequency

fRTCCLK

…

fRTCCLK/128

fRTCCLK/2048(3)

ATPER[2:0]

Tamper output
change
(CK_ATPER)
frequency

Tamper output
change period(1)
(ms)

0x0

fRTCCLK

0.030

0x1

fRTCCLK/2

0.061

0x2

fRTCCLK/4

0.122

0x3

fRTCCLK/8

0.244

0x4

fRTCCLK/16

0.488

0x5

fRTCCLK/32

0.977

0x6

fRTCCLK/64

1.953

0x7

fRTCCLK/128

3.906

…

…

…

0x0

fRTCCLK/128

3.906

0x1

fRTCCLK/256

7.8125

0x2

fRTCCLK/512

15.625

0x3

fRTCCLK/1024

31.250

0x4

fRTCCLK/2048

62.5

0x5

fRTCCLK/4096

125

0x6

fRTCCLK/8192

250

0x7

fRTCCLK/16384

500

0x0

fRTCCLK/2048

62.5

0x1

fRTCCLK/4096

125

0x2

fRTCCLK/8192

250

0x3

fRTCCLK/16384

500

0x4

fRTCCLK/32768

1000

0x5

fRTCCLK/65536

2000

0x6

fRTCCLK/131072

4000

0x7

fRTCCLK/262144

8000

1. Assuming fRTCCLK = 32768 Hz.
2. These values are supported only when the active tamper prescaler extension is supported. Refer to
Section 64.3: TAMP implementation.
3. This setting requires that (PREDIV_A+1) = 128 and (PREDIV_S+1) is a multiple of 16.

RM0456 Rev 6

<!-- pagebreak -->

