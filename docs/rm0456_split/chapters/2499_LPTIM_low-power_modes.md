2687

Real-time clock (RTC)

RM0456

to 1 effectively inserts an extra ck_cal pulse every 211 ck_cal cycles, which means that 512
clocks are added during every calibration cycle.
Using CALM together with CALP, an offset ranging from -511 to +512 ck_cal cycles can be
added during the calibration cycle, which translates to a calibration range of -487.1 ppm to
+488.5 ppm with a resolution of about 0.954 ppm.
The formula to calculate the effective calibrated frequency (FCAL) given the input frequency
(FRTCCLK) is as follows:
FCAL = FRTCCLK x [1 + (CALP x 512 - CALM) / (220 + CALM - CALP x 512)]
Caution:

PREDIV_A must be greater or equal to 3.

Calibration when PREDIV_A < 3
The CALP bit can not be set to 1 when the asynchronous prescaler value (PREDIV_A bits in
RTC_PRER register) is less than 3. If CALP was already set to 1 and PREDIV_A bits are
set to a value less than 3, CALP is ignored and the calibration operates as if CALP was
equal to 0.
It is however possible to perform a calibration with PREDIV_A less than 3 in BCD mode, the
synchronous prescaler value (PREDIV_S) should be reduced so that each second is
accelerated by 8 ck_cal clock cycles, which is equivalent to adding 256 clock cycles every
calibration cycle. As a result, between 255 and 256 clock pulses (corresponding to a
calibration range from 243.3 to 244.1 ppm) can effectively be added during each calibration
cycle using only the CALM bits.
With a nominal RTCCLK frequency of 32768 Hz, when PREDIV_A equals 1 (division factor
of 2), PREDIV_S should be set to 16379 rather than 16383 (4 less). The only other
interesting case is when PREDIV_A equals 0, PREDIV_S should be set to 32759 rather
than 32767 (8 less).
If PREDIV_S is reduced in this way, the formula given the effective frequency of the
calibrated input clock is as follows:
FCAL = FRTCCLK x [1 + (256 - CALM) / (220 + CALM - 256)]
In this case, CALM[7:0] equals 0x100 (the midpoint of the CALM range) is the correct
setting if RTCCLK is exactly 32768.00 Hz.

Verifying the RTC calibration
It is recommended to verify the RTC calibration with LPCAL = 0, in order to have a
32-second calibration cycle.
RTC precision is ensured by measuring the precise frequency of RTCCLK and calculating
the correct CALM value and CALP values. An optional 1 Hz output is provided to allow
applications to measure and verify the RTC precision.
Measuring the precise frequency of the RTC over a limited interval can result in a
measurement error of up to 2 RTCCLK clock cycles over the measurement period,
depending on how the digital calibration cycle is aligned with the measurement period.

<!-- pagebreak -->

