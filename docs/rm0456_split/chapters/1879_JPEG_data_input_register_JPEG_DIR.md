RM0456 Rev 6

RM0456

JPEG codec (JPEG)

46.5.9

JPEG data input register (JPEG_DIR)
Address offset: 0x040
Reset value: 0x0000 0000

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

DATAIN[31:16]
w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

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

w

w

w

w

w

w

w

20

19

18

17

16

DATAIN[15:0]
w

w

w

w

w

w

w

w

w

Bits 31:0 DATAIN[31:0]: Data input FIFO
Input FIFO data register

46.5.10

JPEG data output register (JPEG_DOR)
Address offset: 0x044
Reset value: 0x0000 0000

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

DATAOUT[31:16]
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

r

r

r

r

r

r

r

DATAOUT[15:0]
r

r

Bits 31:0 DATAOUT[31:0]: Data output FIFO
Output FIFO data register.

RM0456 Rev 6

<!-- pagebreak -->

1886

JPEG codec (JPEG)

46.5.11

RM0456

JPEG quantization memory x (JPEG_QMEMx_y)
Address offset: 0x050 + 0x40 * x + 0x4 * y, (x = 0 to 3; y = 0 to 15)
Reset value: 0xXXXX XXXX
Four quantization tables as specified by ISO documentation.
For decoding with header parsing, no quantization table programming is required, the
coefficients are directly written in the quantization memories by the header parser.
For decoding without header parsing or for encoding, the quantization table must be written
by software in zig zag order.

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

QCOEF{4*y+3}[7:0]

20

19

18

17

16

QCOEF{4*y+2}[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

QCOEF{4*y+1}[7:0]
rw

rw

rw

rw

rw

QCOEF{4*y}[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 QCOEF{4*y+3}[7:0]: Quantization coefficient {4*y+3}
8-bit quantization coefficient.
Bits 23:16 QCOEF{4*y+2}[7:0]: Quantization coefficient {4*y+2}
8-bit quantization coefficient.
Bits 15:8 QCOEF{4*y+1}[7:0]: Quantization coefficient {4*y+1}
8-bit quantization coefficient.
Bits 7:0 QCOEF{4*y}[7:0]: Quantization coefficient {4*y}
8-bit quantization coefficient.

46.5.12

JPEG Huffman min (JPEG_HUFFMINx_y)
Address offset: 0x150 + 0x10 * x + 0x4 * y, (x = 0 to 3; y = 0 to 2)
Reset value: 0xXXXX XXXX
This memory stores the minimum Huffman values used internally by the JPEG decoder. The
memory content is written by hardware during the header parsing.

31

30

•

DATA0: Min AC0 value

•

DATA1: Min DC0 value

•

DATA2: Min AC1 value

•

DATA3: Min DC1 value
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

DATA{x}[{32*y+31}:{32*y+16}]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

DATA{x}[{32*y+15}:{32*y}]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

rw

RM0456

JPEG codec (JPEG)

Bits 31:0 DATA{x}[{32*y+31}:{32*y}]: Minimum Huffman value
100-bit minimum Huffman value used internally by the JPEG decoder.

46.5.13

JPEG Huffman min x (JPEG_HUFFMINx_y)
Address offset: 0x150 + 0x10 * x + 0x4 * y, (x = 0 to 3; y = 3)
Reset value: 0xXXXX XXXX
This memory stores the minimum Huffman values used internally by the JPEG decoder. The
memory content is written by hardware during the header parsing.
•

DATA0: Min AC0 value

•

DATA1: Min DC0 value

•

DATA2: Min AC1 value

•

DATA3: Min DC1 value

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

Res.

Res.

Res.

Res.

Res.

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

Res.

DATA{x}[99: 96]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 DATA{x}[99:96]: Minimum Huffman value
100-bit minimum Huffman value used internally by the JPEG decoder.

46.5.14

JPEG Huffman base (JPEG_HUFFBASEx)
Address offset: 0x190 + 0x4*x, (x = 0 to 31)
Reset value: 0xXXXX XXXX
This memory stores the base Huffman values used internally by the JPEG decoder. The
memory content is written by hardware during the header parsing:
•

DATA0 to DATA15: Base AC0 value

•

DATA16 to DATA31: Base DC0 value

•

DATA32 to DATA47: Base AC1 value

•

DATA48 to DATA63: Base DC1 value

31

30

29

28

27

26

25

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

Res.

Res.

Res.

Res.

Res.

Res.

Res.

24

23

22

21

20

19

18

17

16

DATA{2*x+1}[8:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

DATA{2*x}[8:0]
rw

rw

RM0456 Rev 6

rw

rw

rw

<!-- pagebreak -->

1886

JPEG codec (JPEG)

RM0456

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:16 DATA{2*x+1}[8:0]: Data {2*x+1}
Base Huffman value.
Bits 15:9 Reserved, must be kept at reset value.
Bits 8:0 DATA{2*x}[8:0]: Data {2*x}
Base Huffman value.

46.5.15

JPEG Huffman symbol (JPEG_HUFFSYMBx)
Address offset: 0x210 + 0x4 * x, (x = 0 to 83)
Reset value: 0xXXXX XXXX
This memory stores the Huffman symbols used internally by the JPEG decoder. The
memory content is written by hardware during the header parsing:

31

30

•

DATA0 to DATA161: AC0 symbols

•

DATA162 to DATA173: DC0 and DC1 symbols

•

DATA174 to DATA335: AC1 symbols
29

28

27

26

25

24

23

22

21

DATA{4*x+3}[7:0]

20

19

18

17

16

DATA{4*x+2}[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

DATA{4*x+1}[7:0]
rw

rw

rw

rw

rw

DATA{4*x}[7:0]
rw

rw

rw

rw

Bits 31:24 DATA{4*x+3}[7:0]: Data {4*x+3}
Huffman symbol.
Bits 23:16 DATA{4*x+2}[7:0]: Data {4*x+2}
Huffman symbol.
Bits 15:8 DATA{4*x+1}[7:0]: Data {4*x+1}
Huffman symbol.
Bits 7:0 DATA{4*x}[7:0]: Data {4*x}
Huffman symbol.

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

rw

rw

RM0456

JPEG codec (JPEG)

46.5.16

JPEG DHT memory (JPEG_DHTMEMx)
Address offset: 0x360 + 0x4 * x, (x = 0 to 102)
Reset value: 0xXXXX XXXX
For encoding process with header generation, this memory stores the DHT marker segment
AC and DC Huffman tables in the ISO/IEC specification format:

31

30

•

DATA0 to DATA27: DC Huffman table0

•

DATA28 to DATA205: AC Huffman table0

•

DATA206 to DATA233: DC Huffman table1

•

DATA234 to DATA411: AC Huffman table1
29

28

27

26

25

24

23

22

21

DATA{4*x+3}[7:0]

20

19

18

17

16

DATA{4*x+2}[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

DATA{4*x+1}[7:0]
rw

rw

rw

rw

rw

DATA{4*x}[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 DATA{4*x+3}[7:0]: Huffman table data {4*x+3}
Huffman table data for DHT marker segment generation.
Bits 23:16 DATA{4*x+2}[7:0]: Huffman table data {4*x+2}
Huffman table data for DHT marker segment generation.
Bits 15:8 DATA{4*x+1}[7:0]: Huffman table data {4*x+1}
Huffman table data for DHT marker segment generation.
Bits 7:0 DATA{4*x}[7:0]: Huffman table data {4*x}
Huffman table data for DHT marker segment generation.

46.5.17

JPEG Huffman encoder ACx (JPEG_HUFFENC_ACx_y)
Address offset: 0x500 + 0x160 * x + 0x4 * y, (x = 0 to 1; y = 0 to 87)
Reset value: 0xXXXX XXXX
This memory defines the Huffman codes used during the encoding process of AC
components.

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

23

22

21

HLEN{2*y+1}[3:0]

20

19

18

17

16

HCODE{2*y+1}[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

HLEN{2*y}[3:0]
rw

rw

rw

HCODE{2*y}[7:0]
rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1886

JPEG codec (JPEG)

RM0456

Bits 27:24 HLEN{2*y+1}[3:0]: Huffman length {2*y+1}
Number of bits in the Huffman code HCODE{2*y+1} minus 1.
Bits 23:16 HCODE{2*y+1}[7:0]: Huffman code {2*y+1}
8 least significant bits of the Huffman code.
If the Huffman code is less than 8 bits long, the unused bits must be 0.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:8 HLEN{2*y}[3:0]: Huffman length {2*y}
Number of bits in the Huffman code HCODE{2*y} minus 1.
Bits 7:0 HCODE{2*y}[7:0]: Huffman code {2*y}
8 least significant bits of the Huffman code.
If the Huffman code is less than 8 bits long, the unused bits must be 0.

46.5.18

JPEG Huffman encoder DCx (JPEG_HUFFENC_DCx_y)
Address offset: 0x7C0 + 0x20*x + 0x4*y, (x = 0 to 1; y = 0 to 7)
Reset value: 0xXXXX XXXX
This memory defines the Huffman codes used during the encoding process of DC
components.

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

23

22

21

HLEN{2*y+1}[3:0]

20

19

17

16

HCODE{2*y+1}[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

HLEN{2*y}[3:0]
rw

rw

rw

HCODE{2*y}[7:0]
rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:24 HLEN{2*y+1}[3:0]: Huffman length {2*y+1}
Number of bits in the Huffman code HCODE{2*y+1} minus 1.
Bits 23:16 HCODE{2*y+1}[7:0]: Huffman code {2*y+1}
8 least significant bits of the Huffman code.
If the Huffman code is less than 8 bits long, the unused bits must be 0.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:8 HLEN{2*y}[3:0]: Huffman length {2*y}
Number of bits in the Huffman code HCODE{2*y} minus 1.
Bits 7:0 HCODE{2*y}[7:0]: Huffman code {2*y}
8 least significant bits of the Huffman code.
If the Huffman code is less than 8 bits long, the unused bits must be 0.

<!-- pagebreak -->

