RM0456 Rev 6

RM0456

Device electronic signature
Address offset: 0x04
Read only = 0xXXXX XXXX where X is factory-programmed

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

UID[63:48]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

UID[47:32]
r

r

r

r

r

r

r

r

r

Bits 31:8 UID[63:40]: LOT_NUM[23:0]
Lot number (ASCII encoded)
Bits 7:0 UID[39:32]: WAF_NUM[7:0]
Wafer number (8-bit unsigned number)

Address offset: 0x08
Read only = 0xXXXX XXXX where X is factory-programmed
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

UID[95:80]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

UID[79:64]
r

r

r

r

r

r

r

r

r

Bits 31:0 UID[95:64]: LOT_NUM[55:24]
Lot number (ASCII encoded)

76.2

Flash size data register
Base address: 0x0BFA 07A0
Address offset: 0x00
Read only = 0xXXXX where X is factory-programmed

15

14

13

12

11

10

9

r

r

r

r

r

r

r

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

FLASH_SIZE
r

r

Bits 15:0 FLASH_SIZE[15:0]: Flash memory size
This field indicates the size of the device flash memory expressed in Kbytes.
As an example, 0x800 corresponds to 2048 Kbytes.

RM0456 Rev 6

<!-- pagebreak -->

3634

Device electronic signature

76.3

RM0456

Package data register
Base address: 0x0BFA 0500
Address offset: 0x00
Read only = 0xXXXX where X is factory-programmed

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

4

3

2

r

r

r

Bits 15:5 Reserved, must be kept at reset value.
Bits 4:0 PKG[4:0]: Package type
00000: LQFP64
00001: WLCSP72 SMPS
00010: LQFP100
00011: UFBGA132
00100: LQFP144
00101: LQFP48
00111: UFBGA169 or TFBGA169
01000: LQFP64 SMPS
01001: WLSCP90 SMPS
01010: LQFP100 SMPS
01011: UFBGA132 SMPS
01100: LQFP144 SMPS
01101: LQFP48 SMPS
01111: UFBGA169 SMPS or TFBGA169 SMPS
10010: UFBGA64
10011: UFBGA100
10100: LQFP100 DSI SMPS
10101: LQFP144 DSI SMPS
11001: UFBGA144 DSI SMPS
11011: WLCSP208 DSI SMPS
11100: TFBGA216 DSI SMPS
11101: UFBGA100 SMPS
11110: WLCSP56 SMPS
11111: WLCSP150 SMPS or WLCSP150 DSI SMPS
Note: Refer to product datasheet for availability of packages on a specific device.

<!-- pagebreak -->

RM0456 Rev 6

1

0

r

r

PKG[4:0]

RM0456

77

Important security notice

Important security notice
The STMicroelectronics group of companies (ST) places a high value on product security,
which is why the ST product(s) identified in this documentation may be certified by various
security certification bodies and/or may implement our own security measures as set forth
herein. However, no level of security certification and/or built-in security measures can
guarantee that ST products are resistant to all forms of attacks. As such, it is the
responsibility of each of ST's customers to determine if the level of security provided in an
ST product meets the customer needs both in relation to the ST product alone, as well as
when combined with other components and/or software for the customer end product or
application. In particular, take note that:
•

ST products may have been certified by one or more security certification bodies, such
as Platform Security Architecture (www.psacertified.org) and/or Security Evaluation
standard for IoT Platforms (www.trustcb.com). For details concerning whether the ST
product(s) referenced herein have received security certification along with the level
and current status of such certification, either visit the relevant certification standards
website or go to the relevant product page on www.st.com for the most up to date
information. As the status and/or level of security certification for an ST product can
change from time to time, customers should re-check security certification status/level
as needed. If an ST product is not shown to be certified under a particular security
standard, customers should not assume it is certified.

•

Certification bodies have the right to evaluate, grant and revoke security certification in
relation to ST products. These certification bodies are therefore independently
responsible for granting or revoking security certification for an ST product, and ST
does not take any responsibility for mistakes, evaluations, assessments, testing, or
other activity carried out by the certification body with respect to any ST product.

•

Industry-based cryptographic algorithms (such as AES, DES, or MD5) and other open
standard technologies which may be used in conjunction with an ST product are based
on standards which were not developed by ST. ST does not take responsibility for any
flaws in such cryptographic algorithms or open technologies or for any methods which
have been or may be developed to bypass, decrypt or crack such algorithms or
technologies.

•

While robust security testing may be done, no level of certification can absolutely
guarantee protections against all attacks, including, for example, against advanced
attacks which have not been tested for, against new or unidentified forms of attack, or
against any form of attack when using an ST product outside of its specification or
intended use, or in conjunction with other components or software which are used by
customer to create their end product or application. ST is not responsible for resistance
against such attacks. As such, regardless of the incorporated security features and/or
any information or support that may be provided by ST, each customer is solely
responsible for determining if the level of attacks tested for meets their needs, both in
relation to the ST product alone and when incorporated into a customer end product or
application.

•

All security features of ST products (inclusive of any hardware, software,
documentation, and the like), including but not limited to any enhanced security
features added by ST, are provided on an "AS IS" BASIS. AS SUCH, TO THE EXTENT
PERMITTED BY APPLICABLE LAW, ST DISCLAIMS ALL WARRANTIES, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED WARRANTIES OF
MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, unless the
applicable written and signed contract terms specifically provide otherwise.
RM0456 Rev 6

<!-- pagebreak -->

3635

Revision history

78

RM0456

Revision history
Table 813. Document revision history
Date

Revision

22-Jun-2021

1

Initial release

2

Updated:
– Figure 1: System architecture
– Figure 3: Memory map based on IDAU mapping for STM32U575/585
– End of Section 6.3.2: Error code correction (SRAM2, SRAM3, BKPSRAM)
– Sentence added in intro of Section 7.6.2: Readout protection (RDP)
– Desc of bit 21 in Section 7.9.14: FLASH option register (FLASH_OPTR)
– First sentence of Section 10.5.3: LDO and SMPS step down converter fast startup
– One sentence removed in Exiting Stop 0 mode, Exiting Stop 2 mode and Exiting Stop
3 mode
– Note added on BREN bit in Section 10.10.9: PWR Backup domain control register 1
(PWR_BDCR1)
– Desc of MSISSRANGE and MSIKSRANGE in Section 11.8.50: RCC control/status
register (RCC_CSR)
– Table 125: Peripherals interconnect matrix
– Table 124: Programmed GPDMA1 request
– Bit 7 in Section 25.7.1: OCTOSPI control register (OCTOSPI_CR)
– Section 37.2: PSSI main features
– Table 212: RNG configurations
– Section 34.4.13: AES data registers and data swapping
– Section 42.4.14: SAES operation with shared keys
– Address offset of Section 23.6.19: TIMx option register 1 (TIMx_OR1)(x = 16 to 17)
– Table 121: LPTIM1/2/3 input/output pins, Table 122: LPTIM4 input/output pins and
Table 146: LPTIM1/2/3/4 external trigger connection
– Table 533: I2C1, I2C2, I2C4 interconnection and Table 534: I2C3 interconnection
– Table 294: Comparison of analog vs. digital filters
– usart/lpuart_trg6/7 in Table 553: USART interconnection (USART1/2/3 and UART4/5)
– New notes in Section 31.5.20: Continuous communication using USART and DMA
– New Determining the maximum USART baud rate that enables to correctly wake up
the microcontroller from low-power mode
– Table 322: LPUART input/output pins
– usart/lpuart_trg6/7 in Table 565: LPUART interconnections (LPUART1)
– New
– Table 576: SPI interconnection (SPI1 and SPI2) and Table 577: SPI interconnection
(SPI3)
– New Control of the I/Os
– Structure of Section 65.3: Serial-wire and JTAG debug port (SWJ-DP)
– APSEL range in DP access port select register (DP_SELECTR)
– REV_ID[15:0] in DBGMCU identity code register (DBGMCU_IDCODE)
– TREVISION[3:0] in DP target identification register (DP_TARGETIDR)

20-Sep-2021

<!-- pagebreak -->

