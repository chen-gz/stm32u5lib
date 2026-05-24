•

3D perspective correct projections

•

Texture mapping with bilinear filtering or point sampling

RM0456 Rev 6

RM0456

Neo-Chrom graphic processor (GPU2D)

Blit support
•

Rotation, mirroring, stretching (independently on x and y axis)

•

Source and/or destination color keying

•

Pixel format conversions

Text rendering support
•

A1, A2, A4, and A8 bitmap anti-aliased

•

Subsampled anti-aliased

Color formats
•

ABGR8888, ARGB8888, BGRA8888, RGBA8888

•

xBGR8888, xRGB8888, BGRx8888, RGBx8888, RGB888, BGR888

•

BGR565, RGB565

•

ABGR1555, BGRA5551, RGBA5551, ARGB1555

•

ABGR4444, BGRA4444, RGBA4444, ARGB4444

•

RGB322, BGR322

•

ABGR2222, BGRA2222, RGBA2222, ARGB2222

•

TSC4, TSC6, TSC6A

•

LUT1, LUT2, LUT4, LUT8

•

L1, L2, L4, L8 (grayscale)

•

A1, A2, A4, A8

Full alpha blending with hardware blender

45.3

•

Programmable blending modes

•

Source/destination color keying

GPU2D implementation
The following table shows the implementation of GPU2D.
Table 449. GPU2D implementation
GPU2D features

STM32U599/5A9 STM32U5Fx/5Gx

Raster operations

X

X

Vector graphic acceleration

-

X

Additional color modes: ABGR1555, BGRA5551, RGBA5551, ARGB1555,
ABGR4444, BGRA4444, RGBA4444, ARGB4444, ABGR2222, BGRA2222,
RGBA2222, ARGB2222, LUT1, LUT2, LUT4, LUT8

-

X

RM0456 Rev 6

<!-- pagebreak -->

