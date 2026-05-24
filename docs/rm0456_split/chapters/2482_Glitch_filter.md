RM0456 Rev 6

RM0456

Real-time clock (RTC)
Figure 773. RTC block diagram

rtc_tamp_evt
rtc_its

Time stamp detection

TSF

RTC_TS
Time stamp registers
RTC_TSTR
RTC_TSDR
RTC_TSSR
RTC_REFIN
RTC_PRER

RTC_CALR

RTC_PRER

Asynchronous
prescaler
(default = 128)

Smooth
calibration

Synchronous
prescaler
(default = 256)

ck_spre
(default 1 Hz)

rtc_ker_ck

WUCKSEL[1:0]

ck_apre
(default 256 Hz)

rtc_calovf
Calendar

Shadow register
RTC_SSR

Prescaler
2, 4, 8, 16

Shadow
registers
RTC_TR,
RTC_DR

ck_apre clock domain

CALIB
rtc_tamp_evt
TAMPOE

TAMPALRM

Output
control

RTC_OUT1
RTC_OUT2

ALARM
ck_wut

RTC_WUTR
16-bit wake-up
auto reload timer
ck_wut clock domain

WUTF
=0

Alarm A
RTC_ALRMAR
RTC_ALRMASSR
RTC_ALRABINR

ALRAF

rtc_wut_trg

rtc_alra_trg

=

Alarm B
RTC_ALRMBR
RTC_ALRMBSSR
RTC_ALRBBINR

OSEL[1:0]

ALRBF

rtc_alrb_trg

=

rtc_ker_ck clock domain

rtc_pclk
rtc_tzen

rtc_it
IRQ interface
Registers interface
rtc_pclk clock domain
MSv63001V4

RM0456 Rev 6

<!-- pagebreak -->

