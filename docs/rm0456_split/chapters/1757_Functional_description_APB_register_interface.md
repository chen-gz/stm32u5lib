•

DSI Host command mode configuration register (DSI_CMCR): TEARE

•

DSI Host protocol configuration register (DSI_PCR): BTAE.

RM0456 Rev 6

RM0456

44.7

DSI Host (DSI)

Functional description: APB register interface
The APB register interface allows the transmission of generic information in command
mode, and follows a proprietary register interface. Commands sent through this interface
are not constrained to comply with the DCS specification, and can include generic
commands described in the DSI specification as manufacturer-specific.
The DSI Host supports the transmission of write and read command mode packets as
described in the DSI specification. These packets are built using the APB register access.
The DSI Host generic payload data register (DSI_GPDR) has two distinct functions based
on the operation. Writing to this register sends the data as payload when sending a
command mode packet. Reading this register returns the payload of a read back operation.
The DSI Host generic header configuration register (DSI_GHCR) contains the command
mode packet header type and header data. Writing to this register triggers the transmission
of the packet implying that for a long command mode packet, the packet payload needs to
be written in advance in the DSI Host generic payload data register (DSI_GPDR).
The valid packets that can be transmitted through the generic interface are the following:
•

Generic write short packet 0 parameters

•

Generic write short packet 1 parameters

•

Generic write short packet 2 parameters

•

Generic read short packet 0 parameters

•

Generic read short packet 1 parameters

•

Generic read short packet 2 parameters

•

Maximum read packet configuration

•

Generic long write packet

•

DCS write short packet 0 parameters

•

DCS write short packet 1 parameters

•

DCS read short packet 0 parameters

•

DCS write long packet.

A set of bits in the DSI Host generic packet status register (DSI_GPSR) reports the status of
the FIFO associated with APB interface support.
Generic interface packets are always transported using one of the DSI transmission modes,
that is, video mode or command mode. If neither of these modes is selected, the packets
are not transmitted through the link, and the related FIFO eventually becomes overflown.

44.7.1

Packet transmission using the generic interface
The transfer of packets through the APB bus is based on the following conditions:
•

The APB protocol defines that the write and read procedure takes two clock cycles
each to be executed. This means that the maximum input data rate through the APB
interface is always half the speed of the APB clock.

•

The data input bus has a maximum width of 32 bits. This allows for a relation to be
defined between the input APB clock frequency and the maximum bit rate achievable
by the APB interface.

•

The DSI link pixel bit rate when using solely APB is (APB clock frequency) * 16 Mbit/s.

•

When using only the APB interface, the theoretical DSI link maximum bit rate can be
expressed as DSI link maximum bit rate = APB clock frequency (in MHz) * 32 / 2 Mbit/s.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
In this formula, the number 32 represents the APB data bus width, and the division by
two is present because each APB write procedure takes two clock cycles to be
executed.

•

The bandwidth is dependent on the APB clock frequency; the available bandwidth
increases with the clock frequency.

To drive the APB interface to achieve high bandwidth command mode traffic transported by
the DSI link, the DSI Host must operate only in the command mode, and the APB interface
must be the only data source in use. Thus, the APB interface has the entire bandwidth of the
DSI link and does not share it with any another input interface source.
The memory write commands require the maximum throughput from the APB interface,
because they contain the highest amount of data conveyed by the DSI link. While writing the
packet information, first write the payload of a given packet into the payload FIFO using the
DSI Host generic payload data register (DSI_GPDR). When the payload data is for the
command parameters, put the first byte to be transmitted in the least significant byte
position of the APB data bus. After writing the payload, write the packet header into the
command FIFO. For more information about the packet header organization on the 32-bit
APB data bus, so that it is correctly stored inside the command FIFO.
When the payload data is for a memory write command, it contains pixel information and it
must follow the pixel to byte conversion organization referred in the Annex A of the DCS
specification.
Figures 418 to 422 show how the pixel data must be organized in the APB data write bus.
The memory write commands are conveyed in DCS long packets, encapsulated in a DSI
packet. The DSI specifies that the DCS command must be present in the first payload byte
of the packet. This is also included in the diagrams. In figures 418 to 422, the write memory
command can be replaced by the DCS command write memory Start and write memory
Continue.
Figure 418. 24 bpp APB pixel to byte organization

[31 …………………. 0]
8 bit

8 bit

8 bit

8 bit

pwdata(0)

B0[7:0]

G0[7:0]

R0[7:0]

Write_mem
Command

pwdata(1)

R2[7:0]

B1[7:0]

G1[7:0]

R1[7:0]

Pixel
24 bpp
R0
[7:0]

pwdata(2)

G3[7:0]

R3[7:0]

B2[7:0]

G2[7:0]
G0
[7:0]

pwdata(3)

B4[7:0]

G4[7:0]

R4[7:0]

B3[7:0]

pwdata(4)

R6[7:0]

B5[7:0]

G5[7:0]

R5[7:0]

B0
[7:0]

MSv35861V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
Figure 419. 18 bpp APB pixel to byte organization

[31 ………………….0]
8 bit

8 bit

8 bit

8 bit

pwdata(0)

B0[5:0]

2'd0

G0[5:0]

2'd0

R0[5:0]

2'd0

pwdata(1)

R2[5:0]

2'd0

B1[5:0]

2'd0

G1[5:0]

2'd0

Write_mem
Command

R1[5:0]

2'd0

Pixel
18 bpp
R0
[5:0]

pwdata(2)

G3[5:0]

2'd0

R3[5:0]

2'd0

B2[5:0]

2'd0

G2[5:0]

2'd0
G0
[5:0]

pwdata(3)

B4[5:0]

2'd0

G4[5:0]

2'd0

R4[5:0]

2'd0

B3[5:0]

2'd0

pwdata(4)

R6[5:0]

2'd0

B5[5:0]

2'd0

G5[5:0]

2'd0

R5[5:0]

2'd0

B0
[5:0]

MSv35862V1

Figure 420. 16 bpp APB pixel to byte organization

[31 …………………. 0]
8 bit

8 bit

8 bit

8 bit
Write_mem
Command

pwdata(0)

R1[4:0]

G1[5:3]

G0[2:0]

B0[4:0]

R0[4:0]

G0[5:3]

pwdata(1)

R3[4:0]

G3[5:3]

G2[2:0]

B2[4:0]

R2[4:0]

G2[5:3]

G1[2:0]

B1[4:0]

pwdata(2)

R5[4:0]

G5[5:3]

G4[2:0]

B4[4:0]

R4[4:0]

G4[5:3]

G3[2:0]

B3[4:0]

pwdata(3)

R7[4:0]

G7[5:3]

G6[2:0]

B6[4:0]

R6[4:0]

G6[5:3]

G5[2:0]

B5[4:0]

Pixel
16 bpp
R0
[4:0]
G0
[5:0]
B0
[4:0]

MSv35863V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Figure 421. 12 bpp APB pixel to byte organization

[31 …………………. 0]
8 bit

8 bit

8 bit

8 bit

pwdata(0)

G1
[3:0]

B1
[3:0]

B0
[3:0]

R1
[3:0]

R0
[3:0]

G0
[3:0]

pwdata(1)

R4
[3:0]

G4
[3:0]

G3
[3:0]

B3
[3:0]

B2
[3:0]

R3
[3:0]

R2
[3:0]

G2
[3:0]

pwdata(2)

B6
[3:0]

R7
[3:0]

R6
[3:0]

G6
[3:0]

G5
[3:0]

B5
[3:0]

B4
[3:0]

R5
[3:0]

pwdata(3)

G9
[3:0]

B9
[3:0]

B8
[3:0]

R9
[3:0]

R8
[3:0]

G8
[3:0]

G7
[3:0]

B7
[3:0]

Write_mem
Command

Pixel
12 bpp
R0
[3:0]
G0
[3:0]
B0
[3:0]

MSv35864V1

Figure 422. 8 bpp APB pixel to byte organization

[31 …………………. 0]
8 bit

8 bit

8 bit

8 bit

Pixel
8 bpp

Write_mem
Command

R0
[2:0]

pwdata(0)

R2
[2:0]

G2
[2:0]

B2
[1:0]

R1
[2:0]

G1
[2:0]

B1
[1:0]

R0
[2:0]

G0
[2:0]

B0
[1:0]

pwdata(1)

R6
[2:0]

G6
[2:0]

B6
[1:0]

R5
[2:0]

G5
[2:0]

B5
[1:0]

R4
[2:0]

G4
[2:0]

B4
[1:0]

R3
[2:0]

G3
[2:0]

B3
[1:0]

pwdata(2)

R10
[2:0]

G10
[2:0]

B10
[1:0]

R9
[2:0]

G9
[2:0]

B9
[1:0]

R8
[2:0]

G8
[2:0]

B8
[1:0]

R7
[2:0]

G7
[2:0]

B7
[1:0]

G0
[2:0]
B0
[1:0]

MSv35865V1

<!-- pagebreak -->

