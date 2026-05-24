Source/Destination

tamp_tzen

From FLASH option bytes: TZEN

tamp_evt

rtc_tamp_evt used to generate a timestamp event

tamp_potential

The tamp_potential signal is used to block the read and write accesses to
the device secrets listed hereafter:
– backup registers
– SRAM2
RHUK (root hardware unique key) in system Flash memory and BHK
(boot hardware key) hardware buses to SAES are blocked.
The tamp_potential signal is used to erase the device secrets listed
hereafter:
– ICACHE content
– SAES, AES, HASH peripherals
– PKA SRAM
The device secrets access is blocked when erase is ongoing.

RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)
Table 644. TAMP interconnection (continued)
Signal name

Source/Destination

tamp_confirmed

The tamp_confirmed signal is used to erase the device secrets listed
hereafter:
– backup registers
– SRAM2
– ICACHE/DCACHE1 content
– OTFDEC keys and CRC registers
– SAES, AES, HASH peripherals
– PKA SRAM
The device secrets access is blocked when erase is ongoing.
RHUK in system Flash memory (root hardware unique key) hardware bus
to SAES is blocked.

tamp_potential_ercfg0

When the bit ERCFG0 is set in the TAMP_ERCFGR, the
tamp_potential_ercfg0 signal is used to block the read and write accesses
to the device secrets listed hereafter:
– Backup SRAM

tamp_confirmed_ercfg0

When the bit ERCFG0 is set in the TAMP_ERCFGR, the
tamp_confirmed_ercfg0 signal is used to erase the device secrets listed
hereafter:
– Backup SRAM
The device secrets access is blocked when erase is on-going.

tamp_itamp1

Backup domain voltage threshold monitoring(1)

tamp_itamp2

Temperature monitoring(1)

tamp_itamp3

LSE monitoring (LSECSS)(2)

tamp_itamp5

RTC calendar overflow (rtc_calovf)

tamp_itamp6

JTAG/SWD access when RDP > 0

tamp_itamp7
tamp_itamp8

ADC4 (adc4_awd1) analog watchdog monitoring 1
(3)

Monotonic counter 1 overflow

tamp_itamp9

Cryptographic peripherals fault (SAES or AES or PKA or TRNG)

tamp_itamp11

IWDG reset when tamper flag is set (potential tamper timeout)

tamp_itamp12

ADC4 (adc4_awd2) analog watchdog monitoring 2

tamp_itamp13

ADC4 (adc4_awd3) analog watchdog monitoring 3

tamp_bhk

saes_bhk. This bus is used to load the boot hardware key in the secure
AES co-processor.

1. This monitoring must be enabled by setting MONEN in PWR backup domain control register 1
(PWR_BDCR1).
2. This monitoring must be enabled by setting LSECSSON in RCC backup domain control register
(RCC_BDCR).
3. This signal is generated in the TAMP peripheral.

The TZEN option bit is used to activate TrustZone in the device.
TZEN = 1: TrustZone activated.
TZEN = 0: TrustZone disabled.
RM0456 Rev 6

<!-- pagebreak -->

