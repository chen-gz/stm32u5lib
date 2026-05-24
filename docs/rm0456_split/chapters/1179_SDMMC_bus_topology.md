RM0456 Rev 6

RM0456

31.3

Secure digital input/output MultiMediaCard interface (SDMMC)

SDMMC bus topology
Communication over the bus is based on command/response and data transfers.
The basic transaction on the SD/SDIO/e•MMC bus is the command/response transaction.
These types of bus transaction transfer their information directly within the command or
response structure. In addition, some operations have a data token.
Data transfers are done in the following ways:
•

Block mode: data block(s) with block size 2N bytes with N in the range 0-14

•

SDIO multibyte mode: single data block with block size range 1-512 bytes

•

e•MMC Stream mode: continuous data stream

Data transfers to/from e•MMC cards are done in data blocks or streams.
Figure 186. SDMMC “no response” and “no data” operations
SDMMC_CMD

CMD

CMD

Response

SDMMC_D
Operation no response and no data

Operation response no data

From host to card
From card to host

MSv40124V1

Figure 187. SDMMC (multiple) block read operation
Data stop operation
SDMMC_CMD

SDMMC_D

CMD

Response

CMD

Data block

CRC

Data block

CRC

Data block

Response

CRC

Block read operation
Multiple block read operation
From host to card
From card to host

Note:

MSv40155V1

The Stop Transmission command is not required at the end of a e•MMC multiple block read
with predefined block count.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 188. SDMMC (multiple) block write operation
Data stop operation
SDMMC_CMD

CMD

Response

SDMMC_D

CMD

CRC
status

Data block CRC

Busy

Data block CRC

CRC
status

Response

Busy

Block write operation
Multiple block write operation
From host to card
From card to host

Note:

MSv40156V1

The Stop Transmission command is not required at the end of an e•MMC multiple block
write with predefined block count.
The SDMMC does not send any data as long as the Busy signal is asserted (SDMMC_D0
pulled low).
Figure 189. SDMMC (sequential) stream read operation
Data stop operation

SDMMC_CMD

CMD

Response

SDMMC_D

CMD

Response

Data stream

Stream read operation
From host to card
From card to host

MSv40157V2

Figure 190. SDMMC (sequential) stream write operation
Data stop operation
SDMMC_CMD

SDMMC_D

CMD

Response

CMD

Data stream

Response

Busy

Stream write operation
From host to card
From card to host

MSv40158V2

Stream data transfer operates only in a 1-bit wide bit bus configuration on SDMMC_D0 in
single data rate modes (DS, HS, and SDR).

<!-- pagebreak -->

