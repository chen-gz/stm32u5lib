960

CORDIC coprocessor (CORDIC)

RM0456
Table 200. Phase parameters (continued)

Parameter

Description

RES2

Modulus m

SCALE

Not applicable

Range
[0, 1]
0

This function calculates the phase angle in the range -π to π of a vector v = [x y] (also
known as atan2(y,x). It can also be used to perform rectangular to polar conversion.
The primary argument is the x coordinate, that is, the magnitude of the vector in the
direction of the x axis. If |x| > 1, a scaling must be applied in software to adapt it to the q1.31
range of ARG1.
The secondary argument is the y coordinate, that is, the magnitude of the vector in the
direction of the y axis. If |y| > 1, a scaling must be applied in software to adapt it to the q1.31
range of ARG2.
The primary result, RES1, is the phase angle θ of the vector v. RES1 must be multiplied by
π to obtain the angle in radians. Note that values close to π may sometimes wrap to -π due
to the circular nature of the phase angle.
The secondary result, RES2, is the modulus, given by: v =
RES2 is saturated to 1.

2

2

x + y . If |v| > 1 the result in

Modulus
Table 201. Modulus parameters
Parameter

Description

Range

ARG1

x coordinate

[-1, 1]

ARG2

y coordinate

[-1, 1]

RES1

Modulus m

[0, 1]

RES2

Phase angle θ

[-1, 1]

SCALE

Not applicable

0

This function calculates the magnitude, or modulus, of a vector v = [x y]. It can also be used
to perform rectangular to polar conversion.
The primary argument is the x coordinate, that is, the magnitude of the vector in the
direction of the x axis. If |x| > 1, a scaling must be applied in software to adapt it to the q1.31
range of ARG1.
The secondary argument is the y coordinate, that is, the magnitude of the vector in the
direction of the y axis. If |y| > 1, a scaling must be applied in software to adapt it to the q1.31
range of ARG2.
2
2
The primary result, RES1, is the modulus, given by: v = x + y . If |v| > 1 the result in
RES1 is saturated to 1.
The secondary result, RES2, is the phase angle θ of the vector v. RES2 must be multiplied
by π to obtain the angle in radians. Note that values close to π may sometimes wrap to -π
due to the circular nature of the phase angle.

<!-- pagebreak -->

