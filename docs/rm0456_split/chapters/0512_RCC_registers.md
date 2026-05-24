609

Reset and clock control (RCC)

RM0456

11.8

RCC registers

11.8.1

RCC clock control register (RCC_CR)
Address offset: 0x000
Reset value: 0x0000 0035
Access: no wait state; word, half-word, and byte access
HSEBYP and HSEEXT are cleared upon power-on reset. They are not affected upon other
types of reset.

31

30

29

28

27

26

25

24

Res.

Res.

PLL3R
DY

PLL3O
N

PLL2R
DY

PLL2O
N

PLL1R
DY

PLL1O
N

r

rw

r

rw

r

rw

15

14

13

12

11

10

9

Res.

HSIRD
Y
r

SHSIR
DY

SHSIO HSI48R HSI48
N
DY
ON

r

rw

r

rw

23

22

21

20

19

HSEEX
T

CSSO
N

rw

rs

rw

r

rw

4

3

2

1

0

Res.

Res.

Res.

8

7

6

5

HSIKE
RON

HSION

MSIPL
LFAST

MSIPL
LSEL

rw

rw

rw

rw

MSIKR MSIKO
DY
N
r

rw

MSIPL
LEN
rw

18

17

MSISR MSIKE MSISO
DY
RON
N
r

rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 PLL3RDY: PLL3 clock ready flag
This bit is set by hardware to indicate that the PLL3 is locked.
0: PLL3 unlocked
1: PLL3 locked
Bit 28 PLL3ON: PLL3 enable
This bit is set and cleared by software to enable PLL3. It is cleared by hardware when
entering Stop, Standby, or Shutdown mode.
0: PLL3 OFF
1: PLL3 ON
Bit 27 PLL2RDY: PLL2 clock ready flag
This bit is set by hardware to indicate that the PLL2 is locked.
0: PLL2 unlocked
1: PLL2 locked
Bit 26 PLL2ON: PLL2 enable
This bit is set and cleared by software to enable PLL2. It is cleared by hardware when
entering Stop, Standby, or Shutdown mode.
0: PLL2 OFF
1: PLL2 ON
Bit 25 PLL1RDY: PLL1 clock ready flag
This bit is set by hardware to indicate that the PLL1 is locked.
0: PLL1 unlocked
1: PLL1 locked

<!-- pagebreak -->

RM0456 Rev 6

16

HSEBY HSERD HSEO
P
Y
N

rw

RM0456

Reset and clock control (RCC)

Bit 24 PLL1ON: PLL1 enable
This bit is set and cleared by software to enable the main PLL. It is cleared by hardware
when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the PLL1 clock is
used as the system clock.
0: PLL1 OFF
1: PLL1 ON
Bits 23:21 Reserved, must be kept at reset value.
Bit 20 HSEEXT: HSE external clock bypass mode
This bit is set and reset by software to select the external clock mode in bypass mode.
External clock mode must be configured with HSEON bit to be used by the device. This bit
can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass
mode is enabled.
0: external HSE clock analog mode
1: external HSE clock digital mode (through I/O Schmitt trigger)
Bit 19 CSSON: Clock security system enable
This bit is set by software to enable the clock security system. When CSSON is set, the clock
detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware
if a HSE clock failure is detected. This bit is set only and is cleared by reset.
0: clock security system OFF (clock detector OFF)
1: clock security system ON (clock detector ON if the HSE oscillator is stable, OFF if not).
Bit 18 HSEBYP: HSE crystal oscillator bypass
This bit is set and cleared by software to bypass the oscillator with an external clock.
The external clock must be enabled with the HSEON bit set, to be used by the device.
This bit can be written only if the HSE oscillator is disabled.
0: HSE crystal oscillator not bypassed
1: HSE crystal oscillator bypassed with external clock
Bit 17 HSERDY: HSE clock ready flag
This bit is set by hardware to indicate that the HSE oscillator is stable.
0: HSE oscillator not ready
1: HSE oscillator ready
Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.
Bit 16 HSEON: HSE clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the HSE oscillator
when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE
oscillator is used directly or indirectly as the system clock.
0: HSE oscillator off
1: HSE oscillator on
Bit 15 SHSIRDY: SHSI clock ready flag
This bit is set by hardware to indicate that the SHSI oscillator is stable. It is set only when
SHSI is enabled by software (by setting SHSION).
0: SHSI oscillator not ready
1: SHSI oscillator ready
Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.
Bit 14 SHSION: SHSI clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the SHSI when
entering in Stop, Standby, or Shutdown modes.
0: SHSI oscillator off
1: SHSI oscillator on

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 13 HSI48RDY: HSI48 clock ready flag
This bit is set by hardware to indicate that HSI48 oscillator is stable. Itis set only when HSI48
is enabled by software (by setting HSI48ON).
0: HSI48 oscillator not ready
1: HSI48 oscillator ready
Bit 12 HSI48ON: HSI48 clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the HSI48 when
entering in Stop, Standby, or Shutdown modes.
0: HSI48 oscillator off
1: HSI48 oscillator on
Bit 11 Reserved, must be kept at reset value.
Bit 10 HSIRDY: HSI16 clock ready flag
This bit is set by hardware to indicate that HSI16 oscillator is stable. It is set only when HSI16
is enabled by software (by setting HSION).
0: HSI16 oscillator not ready
1: HSI16 oscillator ready
Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
Bit 9 HSIKERON: HSI16 enable for some peripheral kernels
This bit is set and cleared by software to force HSI16 ON even in Stop modes. Keeping
HSI16 on in Stop mode allows the communication speed not to be reduced by the HSI16
startup time. This bit has no effect on HSION value. Refer to Section 11.4.24 for more details.
This bit must be configured at 0 before entering Stop 3 mode.
0: No effect on HSI16 oscillator
1: HSI16 oscillator forced on even in Stop mode
Bit 8 HSION: HSI16 clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the HSI16 oscillator
when entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force
the HSI16 oscillator on when STOPWUCK = 1 when leaving Stop modes, or in case of failure
of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or
indirectly as system clock.
0: HSI16 oscillator off
1: HSI16 oscillator on
Bit 7 MSIPLLFAST: MSI PLL mode fast startup
This bit is set and reset by software to enable/disable the fast PLL mode start-up of the MSI
clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1).
Caution: The fast start-up feature is not active the first time the PLL mode is selected.
The fast start-up is active when the MSI in PLL mode returns from switch off.
0: MSI PLL normal start-up
1: MSI PLL fast start-up
Bit 6 MSIPLLSEL: MSI clock with PLL mode selection
This bit is set and cleared by software to select which MSI output clock uses the PLL mode.
It can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0).
0: PLL mode applied to MSIK (MSI kernel) clock output
1: PLL mode applied to MSIS (MSI system) clock output
Note: If the MSI kernel clock output uses the same oscillator source than the MSI system
clock output, then the PLL mode is applied to both clock outputs.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 5 MSIKRDY: MSIK clock ready flag
This bit is set by hardware to indicate that the MSIK is stable. It is set only when MSI kernel
oscillator is enabled by software by setting MSIKON.
0: MSIK (MSI kernel) oscillator not ready
1: MSIK (MSI kernel) oscillator ready
Note: Once MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.
Bit 4 MSIKON: MSIK clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the MSIK when
entering Stop, Standby, or Shutdown mode. This bit is set by hardware to force the MSIK
oscillator ON when exiting Standby or Shutdown mode. It is set by hardware to force the
MSIK oscillator on when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes,
or in case of a failure of the HSE oscillator.
0: MSIK (MSI kernel) oscillator disabled
1: MSIK (MSI kernel) oscillator enabled
Bit 3 MSIPLLEN: MSI clock PLL-mode enable
This bit is set and cleared by software to enable/disable the PLL part of the MSI clock source.
MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set
by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready.
This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE
detects a LSE failure (see RCC_CSR).
0: MSI PLL-mode OFF
1: MSI PLL-mode ON
Bit 2 MSISRDY: MSIS clock ready flag
This bit is set by hardware to indicate that the MSIS oscillator is stable. It is set only when
MSIS is enabled by software (by setting MSISON).
0: MSIS (MSI system) oscillator not ready
1: MSIS (MSI system) oscillator ready
Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.
Bit 1 MSIKERON: MSI enable for some peripheral kernels
This bit is set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI
on in Stop mode allows the communication speed not to be reduced by the MSI startup time.
This bit has no effect on MSISON and MSIKON values (see Section 11.4.24 for more
details). This bit must be configured at 0 before entering Stop 3 mode.
0: No effect on MSI oscillator
1: MSI oscillator forced ON even in Stop mode
Bit 0 MSISON: MSIS clock enable
This bit is set and cleared by software. It is cleared by hardware to stop the MSIS oscillator
when entering Stop, Standby or Shutdown mode. This bit is set by hardware to force
the MSIS oscillator on when exiting Standby or Shutdown mode. It is set by hardware to force
the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes, or in case of a
failure of the HSE oscillator.
Set by hardware when used directly or indirectly as system clock.
0: MSIS (MSI system) oscillator off
1: MSIS (MSI system) oscillator on

RM0456 Rev 6

<!-- pagebreak -->

