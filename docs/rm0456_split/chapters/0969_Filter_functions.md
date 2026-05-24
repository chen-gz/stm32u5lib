RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)

Load X2 buffer
This function pre-loads the X2 buffer with N + M values, starting from the address in
X2_BASE. Successive writes to the FMAC_WDATA register load the write data into the X2
buffer and increment the write address.
The function can be used to pre-load the buffer with the elements of a vector, or the
coefficients of a filter. In the case of an IIR, the N feed-forward and M feed-back coefficients
are concatenated and loaded together into the X2 buffer. The total number of coefficients is
equal to N + M. For an FIR, there are no feedback coefficients, so M = 0.
Parameters
•

The parameter P contains the number of values, N, to be loaded into the X2 buffer
starting from address X2_BASE.

•

The parameter Q contains the number of values, M, to be loaded into the X2 buffer
starting from address X2_BASE + N.

•

The parameter R is not used.

The function completes when N + M writes have been performed to the FMAC_WDATA
register.

Load Y buffer
This function pre-loads the Y buffer with N values, starting from the address in Y_BASE.
Successive writes to the FMAC_WDATA register load the write data into the Y buffer and
increment the write address. The read pointer points to the address Y_BASE + N when the
function completes.
The function can be used to pre-load the feedback storage elements of an IIR filter.
Parameters
•

The parameter P contains the number of values to be loaded into the Y buffer.

•

The parameters Q and R are not used.

The function completes when N writes have been performed to the FMAC_WDATA register.

26.3.6

Filter functions
The following filter functions are supported by the FMAC unit. These functions are triggered
by writing the corresponding value in the FUNC bitfield of the FMAC_PARAM register with
the START bit set. The P, Q and R bitfields must also contain the appropriate parameter
values for each function as detailed below. The filter functions continue to run until the
START bit is reset by software.

Convolution (FIR filter)
Y = B*X
N
R

yn = 2 ⋅

∑ bk xn – k
k=0

This function performs a convolution of a vector B of length N+1 and a vector X of indefinite
length. The elements of Y for incrementing values of n are calculated as the dot product,

RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

RM0456

yn = B.Xn, where Xn = [xn-N,...,xn] is composed of the N+1 elements of X at indexes n - N to
n.
This function corresponds to a finite impulse response (FIR) filter, where vector B contains
the filter coefficients and vector X the sampled data.
The structure of the filter (direct form) is shown in Figure 114.
Figure 114. FIR filter structure

b[0]

2R

x[n]

y[n]
Z-1

b[1]

x[n-1]
Z-1

b[2]

x[n-2]
Z-1

b[3]

x[n-3]
Z-1
b[N]
x[n-N]
MSv47126V1

Note that the cross correlation vector can be calculated by reversing the order of the
coefficient vector B.
Input:
•

X1 buffer contains the elements of vector X. It is a circular buffer of length N + 1 + d.

•

X2 buffer contains the elements of vector B. It is a fixed buffer of length N + 1.

Output:
•

<!-- pagebreak -->

Y buffer contains the output values, yn. It is a circular buffer of length d.

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
Parameters:
•

The parameter P contains the length, N+1, of the coefficient vector B in the range
[2:127].

•

The parameter R contains the gain to be applied to the accumulator output. The value
output to the Y buffer is multiplied by 2R, where R is in the range [0:7]

•

The parameter Q is not used.

The function completes when the START bit in the FMAC_PARAM register is reset by
software.

IIR filter
Y = B*X+A*Y’

M
⎛ N
⎞
⎜
yn = 2 ⋅
b
x
+
a
y
∑ k n – k⎟⎟
⎜ ∑ k n–k
⎝k = 0
⎠
k=1
R

This function implements an infinite impulse response (IIR) filter. The filter output vector Y is
the convolution of a coefficient vector B of length N+1 and a vector X of indefinite length,
plus the convolution of the delayed output vector Y’ with a second coefficient vector A, of
length M. The elements of Y for incrementing values of n are calculated as yn = B.Xn +
A.Yn-1, where Xn = [xn-N,...,xn] comprises the N+1 elements of X at indexes n - N to n, while
Yn-1 = [yn-M,...,yn-1] comprises the M elements of Y at indexes n - M to n - 1. The structure of
the filter (direct form 1) is shown in Figure 115.

RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

RM0456
Figure 115. IIR filter structure (direct form 1)

2R

b[0]
x[n]

y[n]

Z-1

a[1]

b[1]

y[n-1]

x[n-1]

Z-1

b[2]

a[2]

x[n-2]

Z-1

Z-1

Z-1
y[n-2]

b[3]

a[3]

x[n-3]

Z-1
y[n-3]

Z-1

Z-1
b[N]

a[M]

x[n-N]

y[n-M]
MSv47127V1

Input:
•

X1 buffer contains the elements of vector X. It is a circular buffer of length N + 1+ d.

•

X2 buffer contains the elements of coefficient vectors B and A concatenated
(b0, b1, b2..., bN, a1, a2, ..., aM). It is a fixed buffer of length M+N+1.

Output:
•

Y buffer contains the output values, yn. It is a circular buffer of length M + d.

Parameters

<!-- pagebreak -->

