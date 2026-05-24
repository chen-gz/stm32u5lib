1045

Flexible static memory controller (FSMC)

RM0456

Figure 135. Asynchronous wait during a write access waveforms
Memory transaction

A[25:0]
address phase

data setup phase

NEx

NWAIT

don’t care

don’t care
1HCLK

NWE

D[15:0]

data driven by FMC
3HCLK
MSv40168V1

1. NWAIT polarity depends on WAITPOL bit setting in FMC_BCRx register.

CellularRAM™ (PSRAM) refresh management
The CellularRAM™ does not enable maintaining the chip select signal (NE) low for longer
than the tCEM timing specified for the memory device. This timing can be programmed in the
FMC_PCSCNTR register. It defines the maximum duration of the NE low pulse in HCLK
cycles for asynchronous accesses and FMC_CLK cycles for synchronous accesses

27.6.5

Synchronous transactions
The memory clock, FMC_CLK, is a submultiple of HCLK. It depends on the value of
CLKDIV and the MWID/ AHB data size, following the formula given below:
Whatever MWID size: 16 or 8-bit, the FMC_CLK divider ratio is always defined by the
programmed CLKDIV value.
Example:
•

If CLKDIV=1, MWID = 16 bits, AHB data size=8 bits, FMC_CLK=HCLK/2.

NOR flash memories specify a minimum time from NADV assertion to CLK high. To meet
this constraint, the FMC does not issue the clock to the memory during the first internal
clock cycle of the synchronous access (before NADV assertion). This guarantees that the
rising edge of the memory clock occurs in the middle of the NADV low pulse.

Data latency versus NOR memory latency
The data latency is the number of cycles to wait before sampling the data. The DATLAT
value must be consistent with the latency value specified in the NOR flash configuration

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
register. The FMC does not include the clock cycle when NADV is low in the data latency
count.

Caution:

Some NOR flash memories include the NADV Low cycle in the data latency count, so that
the exact relation between the NOR flash latency and the FMC DATLAT parameter can be
either:
•

NOR flash latency = (DATLAT + 2) CLK clock cycles

•

or NOR flash latency = (DATLAT + 3) CLK clock cycles

Some recent memories assert NWAIT during the latency phase. In such cases DATLAT can
be set to its minimum value. As a result, the FMC samples the data and waits long enough
to evaluate if the data are valid. Thus the FMC detects when the memory exits latency and
real data are processed.
Other memories do not assert NWAIT during latency. In this case the latency must be set
correctly for both the FMC and the memory, otherwise invalid data are mistaken for good
data, or valid data are lost in the initial phase of the memory access.

Single-burst transfer
When the selected bank is configured in Burst mode for synchronous accesses, if for
example an AHB single-burst transaction is requested on 16-bit memories, the FMC
performs a burst transaction of length 1 (if the AHB transfer is 16 bits), or length 2 (if the
AHB transfer is 32 bits) and de-assert the chip select signal when the last data is strobed.
Such transfers are not the most efficient in terms of cycles compared to asynchronous read
operations. Nevertheless, a random asynchronous access would first require to re-program
the memory access mode, which would altogether last longer.

Cross boundary page for CellularRAM™ 1.5
CellularRAM™ 1.5 does not allow burst access to cross the page boundary. The FMC
controller is used to split automatically the burst access when the memory page size is
reached by configuring the CPSIZE bits in the FMC_BCR1 register following the memory
page size.

Wait management
For synchronous NOR flash memories, NWAIT is evaluated after the programmed latency
period, which corresponds to (DATLAT+2) CLK clock cycles.
If NWAIT is active (low level when WAITPOL = 0, high level when WAITPOL = 1), wait
states are inserted until NWAIT is inactive (high level when WAITPOL = 0, low level when
WAITPOL = 1).
When NWAIT is inactive, the data is considered valid either immediately (bit WAITCFG = 1)
or on the next clock edge (bit WAITCFG = 0).
During wait-state insertion via the NWAIT signal, the controller continues to send clock
pulses to the memory, keeping the chip select and output enable signals valid. It does not
consider the data as valid.
In Burst mode, there are two timing configurations for the NOR flash NWAIT signal:
•

The flash memory asserts the NWAIT signal one data cycle before the wait state
(default after reset).

•

The flash memory asserts the NWAIT signal during the wait state

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

The FMC supports both NOR flash wait state configurations, for each chip select, thanks to
the WAITCFG bit in the FMC_BCRx registers (x = 0..3).
Figure 136. Wait configuration waveforms
Memory transaction = burst of 4 half words
HCLK

CLK
A[25:16]

addr[25:16]

NADV
NWAIT
(WAITCFG = 0)
NWAIT
(WAITCFG = 1)
inserted wait state
A/D[15:0]

addr[15:0]

data

data

data
ai15798c

<!-- pagebreak -->

