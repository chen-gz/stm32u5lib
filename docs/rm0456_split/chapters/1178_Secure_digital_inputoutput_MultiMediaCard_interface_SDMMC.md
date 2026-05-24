1177

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

31

Secure digital input/output MultiMediaCard interface
(SDMMC)

31.1

SDMMC main features
The SD/SDIO, embedded MultiMediaCard (e•MMC) host interface (SDMMC) provides an
interface between the AHB bus and SD memory cards, SDIO cards and e•MMC devices.
The MultiMediaCard system specifications are available through the MultiMediaCard
Association website at www.jedec.org, published by the MMCA technical committee.
SD memory card and SD I/O card system specifications are available through the SD card
Association website at www.sdcard.org.
The SDMMC features include the following:
•

Compliance with Embedded MultiMediaCard System Specification Version 5.1.
Card support for three different databus modes: 1-bit (default), 4-bit and 8-bit.
(HS200 SDMMC_CK speed limited to maximum allowed I/O speed)(HS400 is not
supported).

•

Full compatibility with previous versions of MultiMediaCards (backward compatibility).

•

Full compliance with SD memory card specifications version 6.0.
(SDR104 SDMMC_CK speed limited to maximum allowed I/O speed, SPI mode and
UHS-II mode not supported).

•

Full compliance with SDIO card specification version 4.0.
Card support for two different databus modes: 1-bit (default) and 4-bit.
(SDR104 SDMMC_CK speed limited to maximum allowed I/O speed, SPI mode and
UHS-II mode not supported).

•

Data transfer up to 208 Mbyte/s for the 8-bit mode.
(depending maximum allowed I/O speed).

•

Data and command output enable signals to control external bidirectional drivers.

•

IDMA linked list support

The MultiMediaCard/SD bus connects cards to the host.
The current version of the SDMMC supports only one SD/SDIO/e•MMC card at any one
time and a stack of e•MMC.

31.2

SDMMC implementation
Table 272. SDMMC features
SDMMC modes/features(1)

SDMMC1

SDMMC2(2)

Variable delay (SDR104, HS200)

X

X

SDMMC_CKIN

X

-

SDMMC_CDIR, SDMMC_D0DIR

X

-

SDMMC_D123DIR

X

-

1. X = supported.
2. Not available in STM32U535/545.

<!-- pagebreak -->

