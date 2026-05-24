2952

Serial peripheral interface (SPI)

RM0456

Figure 854. SS interleaving pulses between data (SSOE = 1, SSOM = 1, SSM = 0)
I. CPHA=0, CPOL=0, SSIOP=0, LSBFRST=0

MIDI[3:0] - 1

SS
SCK
MOSI

X

MSB

MISO

MSB

X

t SCK

MSSI[3:0]

LSB

MSB

X

LSB

X

DSIZE[4:0] + 1

X

MSB

MIDI[3:0]

II. CPHA=1, CPOL=0, SSIOP=0, LSBFRST=0

DSIZE[4:0] + 1

MIDI[3:0] - 1

SS
SCK
MOSI

MSB

X

MISO

X

MSB

t SCK/2

MSSI[3:0]

MSB

X

LSB

X

LSB

DSIZE[4:0] + 1

MSB

MIDI[3:0]

III. CPHA=0, CPOL=1, SSIOP=1, LSBFRST=1

DSIZE[4:0] + 1

MIDI[3:0] - 1

SS
SCK
MOSI

X

MISO

LSB

MSB
MSB X

LSB

X

tSCK

MSSI[3:0]

LSB

X

DSIZE[4:0] + 1

X LSB

MIDI[3:0]

IV. CPHA=1, CPOL=1, SSIOP=1, LSBFRST=1

DSIZE[4:0] + 1

MIDI[3:0] - 1

SS
SCK
MOSI
MISO

X

LSB
X

MSSI[3:0]

MSB
LSB

tSCK /2

X

X LSB

MSB

DSIZE[4:0] + 1

LSB

MIDI[3:0]

DSIZE[4:0] + 1
MSv40471V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
1. MSSI[3:0] = 0010, MIDI[3:0] = 0010.
2. SS interleaves between data when MIDI[3:0] > 1 wide of the interleaving pulse is always one SCK period
less than the gap provided between the frames (defined by the MIDI parameter). If MIDI is set, the frames
are separated by a single SCK period but no interleaving pulse appears on SS.

68.4.8

Ready pin (RDY) management
The status of the slave capability to handle data can be checked on the RDY pin. By default,
a low level indicates that the slave is not ready for transfer. The reason can be that the slave
TxFIFO is empty, RxFIFO full or the SPI is disabled. An active level of the signal can be
selected by the RDIOP bit. If the master continues or starts to communicate with the slave
when it indicates a not ready status, it is highly probable that the transfer fails.
The logic to control the RDY output is rather complex, tied closely with the TSIZE and
DSIZE settings. The RDY reaction is more pessimistic and sensitive to TxFIFO becoming
nearly empty and/or RxFIFO nearly full during a frame transfer. This pessimistic logic is
suppressed at the end of a transfer only when RDY stays active, despite TxFIFO becomes
fully empty and/or RxFIFO becomes fully occupied. The target is to prevent any data
corruption and inform the master in time that it is necessary to suspend the transfer
temporarily until the next transferred data can be processed safely again. When the RDY
signal input is enabled at master side, the master suspends the communication once the
slave indicates not ready status. This prevents the master to complete the transfer of an
ongoing frame, which just empties the slave TxFIFO or full fills its RxFIFO until a next data is
written and/or read there (despite the frame still can be completed without any constraint). It
can make a problem if the TSIZE = 0 configuration is applied at slave because slave then
never evaluates the end of the transfer (which suppresses the not ready status just when
the last data is sent). Then the user has to release the RxFIFO and/or write additional (even
dummy) data to TxFIFO by software at slave side to release the not RDY signal, unblock ST
master and so enable it to continue at the communication suspended at middle of a frame
occasionally.
When RDY is not used by the master, it must be disabled (RDIOM = 0). Then an internal
logic of the master simulates the slave status always ready. In this case, the RDIOP bit
setting has no meaning.
Due to synchronization between clock domains and evaluation of the RDY logic on both
master and slave sides, the RDY pin feature is not reliable and cannot be used when the
size of data frames is configured shorter than 8 bits.

68.4.9

Communication formats
During SPI communication, receive and transmit operations are performed simultaneously.
The serial clock (SCK) synchronizes the shifting and sampling of the information on the data
lines. The communication format depends on the clock phase, the clock polarity, and the
data frame format. To be able to communicate together, the master and slave devices must
follow the same communication format and be synchronized correctly.

Clock phase and polarity controls
Four possible timing relationships can be chosen by software, using the CPOL and CPHA
bits in the SPI_CFG2 register. The CPOL (clock polarity) bit controls the idle state value of
the clock when no data is being transferred. This bit affects both master and slave modes. If
CPOL is reset, the SCK pin has a low-level idle state. If CPOL is set, the SCK pin has a
high-level idle state.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

If the CPHA bit is set, the second edge on the SCK pin captures the first data bit transferred
(falling edge if the CPOL bit is reset, rising edge if the CPOL bit is set). Data are latched on
each occurrence of this clock transition type. If the CPHA bit is reset, the first edge on the
SCK pin captures the first data bit transferred (falling edge if the CPOL bit is set, rising edge
if the CPOL bit is reset). Data are latched on each occurrence of this clock transition type.
The combination of the CPOL (clock polarity) and CPHA (clock phase) bits selects the data
capture clock edges (dotted lines in Figure 855).
Figure 855 shows an SPI full-duplex transfer with the four combinations of the CPHA and
CPOL bits.
Note:

Prior to changing the CPOL/CPHA bits the SPI must be disabled by resetting the SPE bit.
The idle state of SCK must correspond to the polarity selected in the SPI_CFG2 register (by
pulling the SCK pin up if CPOL = 1 or pulling it down if CPOL = 0).
Figure 855. Data clock timing diagram
CPHA =1
CPOL = 1

CPOL = 0

MOSI(1)

MSBit

MISO(1)

MSBit

LSBit

LSBit

NSS (to slave)

CPHA =0
CPOL = 1

CPOL = 0

MOSI(1)

MISO(1)

LSBit

MSBit

MSBit

LSBit

NSS (to slave)

ai17154f

1. The order of data bits depends on the LSBFRST bit setting.

<!-- pagebreak -->

