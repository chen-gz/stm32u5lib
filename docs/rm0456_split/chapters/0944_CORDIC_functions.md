960

CORDIC coprocessor (CORDIC)

RM0456

Table 197 lists the functions supported by the CORDIC coprocessor.
Table 197. CORDIC functions
Function

Primary
Secondary
argument (ARG1) argument (ARG2)

Primary
result (RES1)

Secondary
result (RES2)

Cosine

angle θ

modulus m

m ⋅ cos θ

m ⋅ sin θ

Sine

angle θ

modulus m

m ⋅ sin θ

m ⋅ cos θ

Phase

x

y

atan2(y,x)

x +y

Modulus

x

y

2

atan2(y,x)

Arctangent

x

none

tan-1 x

none

Hyperbolic cosine

x

none

cosh x

sinh x

Hyperbolic sine

x

none

sinh x

cosh x

Hyperbolic arctangent

x

none

tanh-1 x

none

Natural logarithm

x

none

ln x

none

Square root

x

none

x

none

x +y

2

2

2

Several functions take two input arguments (ARG1 and ARG2), and some generate two
results (RES1 and RES2) simultaneously. This is a side-effect of the algorithm and means
that only one operation is needed to obtain two values. This is the case, for example, when
performing polar-to-rectangular conversion: sin θ also generates cos θ, and cos θ also
generates sin θ. Similarly for rectangular-to-polar conversion (phase(x,y), modulus(x,y)) and
for hyperbolic functions (cosh θ, sinh θ).
Note:

The exponential function, exp x, can be obtained as the sum of sinh x and cosh x.
Furthermore, base N logarithms, logN x, can be derived by multiplying ln x by a constant K,
where K = 1/ln N.
For certain functions (atan, log, sqrt) a scaling factor (see Section 25.3.4) can be applied to
extend the range beyond the maximum [-1, 1] supported by the q1.31 fixed point format.
The scaling factor must be set to 0 for all other circular functions, and to 1 for hyperbolic
functions.

Cosine
Table 198. Cosine parameters
Parameter

<!-- pagebreak -->

