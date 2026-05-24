621

Clock recovery system (CRS)

RM0456

Table 121. CRS internal input/output signals (continued)
Internal signal name

Signal type

Description

crs_sync_in_0
crs_sync_in_1
crs_sync_in_2

Digital input

SYNC signal source

crs_sync_in_3

Table 122. CRS interconnection for STM32U535/545/575/585
Internal signal name

Description

crs_sync_in_0

GPIO selected as SYNC signal source

crs_sync_in_1

LSE selected as SYNC signal source

crs_sync_in_2

USB SOF selected as SYNC signal source (default)

crs_sync_in_3

Reserved

Table 123. CRS interconnection for STM32U59x/5Ax/5Fx/5Gx
Internal signal name

12.4.3

Description

crs_sync_in_0

00: GPIO AF selected as SYNC signal source

crs_sync_in_1

01: LSE selected as SYNC signal source

crs_sync_in_2

01: Reserved (default)

crs_sync_in_3

11: Reserved

Synchronization input
The CRS synchronization source (crs_sync_in_x) can be selected through SYNCSRC[1:0]
bitfield of CRS_CFGR register (refer to Section 12.4.2: CRS internal signals for the possible
sources). For a better robustness of the crs_sync_in_x input, a simple digital filter (2 out of 3
majority votes, sampled by the 48 MHz clock) is implemented to filter out glitches. In
addition, the source signal has a configurable polarity (selected through SYNCPOL bit of
CRS_CFGR). The signal can then be divided by a programmable binary prescaler to obtain
a synchronization signal in a suitable frequency range (usually around 1 kHz).
For more information on the CRS synchronization source configuration, refer to
CRS_CFGR register.
It is also possible to generate a synchronization event by software, by setting the SWSYNC
bit in the CRS_CR register.
For more information on the CRS synchronization source configuration, refer to CRS
configuration register (CRS_CFGR).
It is also possible to generate a synchronization event by software, by setting the SWSYNC
bit in the CRS_CR register.

<!-- pagebreak -->

