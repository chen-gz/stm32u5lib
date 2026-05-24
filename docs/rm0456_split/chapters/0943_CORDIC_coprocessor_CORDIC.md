0

CRC_INIT[31:0]

CRC_POL
Reset value

POLYSIZE[1:0]

0

REV_IN[1:0]

0

REV_OUT

Reset value

Res.

IDR[31:0]

Res.

CRC_IDR

1

Res.

0x08

12

Reset value

0x04

29

CRC_DR

13

0x00

Register name

30

Offset

31

Table 196. CRC register map and reset values

RM0456 Rev 6

RM0456

CORDIC coprocessor (CORDIC)

25

CORDIC coprocessor (CORDIC)

25.1

CORDIC introduction
The CORDIC coprocessor provides hardware acceleration of mathematical functions
(mainly trigonometric ones) commonly used in motor control, metering, signal processing,
and many other applications.
It speeds up the calculation of these functions compared to a software implementation,
making possible the use of a lower operating frequency, or freeing up processor cycles to
perform other tasks.

25.2

CORDIC main features
•

24-bit CORDIC rotation engine

•

Circular and Hyperbolic modes

•

Rotation and Vectoring modes

•

Functions: sine, cosine, sinh, cosh, atan, atan2, atanh, modulus, square root, natural
logarithm

•

Programmable precision

•

Low latency AHB slave interface

•

Results can be read as soon as ready, without polling or interrupt

•

DMA read and write channels

•

Multiple register read/write by DMA

25.3

CORDIC functional description

25.3.1

General description
The CORDIC is a cost-efficient successive approximation algorithm for evaluating
trigonometric and hyperbolic functions.
In trigonometric (circular) mode, the sine and cosine of an angle θ are determined by
rotating the unit vector [1, 0] through decreasing angles until the cumulative sum of the
rotation angles equals the input angle θ. The x and y cartesian components of the rotated
vector then correspond, respectively, to the cosine and sine of θ. Inversely, the angle of a
vector [x, y] corresponding to arctangent (y / x), is determined by rotating [x, y] through
successively decreasing angles to obtain the unit vector [1, 0]. The cumulative sum of the
rotation angles gives the angle of the original vector.
The CORDIC algorithm can also be used for calculating hyperbolic functions (sinh, cosh,
atanh), by replacing the successive circular rotations by steps along a hyperbole.
Other functions can be derived from the basic functions described above.

25.3.2

CORDIC functions
The first step when using the coprocessor is to select the required function, by programming
the FUNC field of the CORDIC_CR register.

RM0456 Rev 6

<!-- pagebreak -->

