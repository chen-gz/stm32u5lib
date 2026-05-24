RM0456 Rev 6

RM0456

CORDIC coprocessor (CORDIC)

Arctangent
Table 202. Arctangent parameters
Parameter

Description

Range

–n

[-1, 1]

ARG2

Not applicable

-

RES1

2-n

RES2

Not applicable

SCALE

n

ARG1

x⋅ 2

-1

· tan x, in radians, divided by p

[-1, 1]
[0 7]

This function calculates the arctangent, or inverse tangent, of the input argument x.
The primary argument, ARG1, is the input value, x = tan θ. If |x| > 1, a scaling factor of 2-n
must be applied in software such that -1 < x · 2-n < 1. The scaled value x · 2-n is programmed
in ARG1 and the scale factor n must be programmed in the SCALE parameter.
Note that the maximum input value allowed is tan θ = 128, which corresponds to an angle
θ = 89.55 degrees. For |x| > 128, a software method must be used to find tan-1 x.
The secondary argument, ARG2, is unused.
The primary result, RES1, is the angle θ = tan-1 x. RES1 must be multiplied by 2n · π to
obtain the angle in radians.
The secondary result, RES2, is unused.

Hyperbolic cosine
Table 203. Hyperbolic cosine parameters
Parameter

Description

Range

ARG1

x⋅ 2

–n

[-0.559 0.559]

ARG2

Not applicable

-

RES1

2

RES2

2

SCALE

–n

⋅ cosh x

[0.5 0.846]

–n

⋅ sinh x

[-0.683 0.683]

n

1

This function calculates the hyperbolic cosine of a hyperbolic angle x. It can also be used to
calculate the exponential functions ex = cosh x + sinh x, and e-x = cosh x - sinh x.
The primary argument is the hyperbolic angle x. Only values of x in the range -1.118 to
+1.118 are supported. Since the minimum value of cosh x is 1, which is beyond the range of
the q1.31 format, a scaling factor of 2-n must be applied in software. The factor n = 1 must
be programmed in the SCALE parameter.
The secondary argument is not used.

RM0456 Rev 6

<!-- pagebreak -->

