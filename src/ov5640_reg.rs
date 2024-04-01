pub const OV5640_I2C_ADDR: u16 = 0x78;
pub const OV5640_SYSREM_RESET00: u16 = 0x3000;
pub const OV5640_SYSREM_RESET01: u16 = 0x3001;
pub const OV5640_SYSREM_RESET02: u16 = 0x3002;
pub const OV5640_SYSREM_RESET03: u16 = 0x3003;
pub const OV5640_CLOCK_ENABLE00: u16 = 0x3004;
pub const OV5640_CLOCK_ENABLE01: u16 = 0x3005;
pub const OV5640_CLOCK_ENABLE02: u16 = 0x3006;
pub const OV5640_CLOCK_ENABLE03: u16 = 0x3007;
pub const OV5640_SYSTEM_CTROL0: u16 = 0x3008;
pub const OV5640_CHIP_ID_HIGH_BYTE: u16 = 0x300A;
pub const OV5640_CHIP_ID_LOW_BYTE: u16 = 0x300B;
pub const OV5640_MIPI_CONTROL00: u16 = 0x300E;
pub const OV5640_PAD_OUTPUT_ENABLE00: u16 = 0x3016;
pub const OV5640_PAD_OUTPUT_ENABLE01: u16 = 0x3017;
pub const OV5640_PAD_OUTPUT_ENABLE02: u16 = 0x3018;
pub const OV5640_PAD_OUTPUT_VALUE00: u16 = 0x3019;
pub const OV5640_PAD_OUTPUT_VALUE01: u16 = 0x301A;
pub const OV5640_PAD_OUTPUT_VALUE02: u16 = 0x301B;
pub const OV5640_PAD_SELECT00: u16 = 0x301C;
pub const OV5640_PAD_SELECT01: u16 = 0x301D;
pub const OV5640_PAD_SELECT02: u16 = 0x301E;
pub const OV5640_CHIP_REVISION: u16 = 0x302A;
pub const OV5640_PAD_CONTROL00: u16 = 0x301C;
pub const OV5640_SC_PWC: u16 = 0x3031;
pub const OV5640_SC_PLL_CONTRL0: u16 = 0x3034;
pub const OV5640_SC_PLL_CONTRL1: u16 = 0x3035;
pub const OV5640_SC_PLL_CONTRL2: u16 = 0x3036;
pub const OV5640_SC_PLL_CONTRL3: u16 = 0x3037;
pub const OV5640_SC_PLL_CONTRL4: u16 = 0x3038;
pub const OV5640_SC_PLL_CONTRL5: u16 = 0x3039;
pub const OV5640_SC_PLLS_CTRL0: u16 = 0x303A;
pub const OV5640_SC_PLLS_CTRL1: u16 = 0x303B;
pub const OV5640_SC_PLLS_CTRL2: u16 = 0x303C;
pub const OV5640_SC_PLLS_CTRL3: u16 = 0x303D;
pub const OV5640_IO_PAD_VALUE00: u16 = 0x3050;
pub const OV5640_IO_PAD_VALUE01: u16 = 0x3051;
pub const OV5640_IO_PAD_VALUE02: u16 = 0x3052;

// /* SCCB control [0x3100 ~ 0x3108]                       */
pub const OV5640_SCCB_ID: u16 = 0x3100;
pub const OV5640_SCCB_SYSTEM_CTRL0: u16 = 0x3102;
pub const OV5640_SCCB_SYSTEM_CTRL1: u16 = 0x3103;
pub const OV5640_SYSTEM_ROOT_DIVIDER: u16 = 0x3108;
//
// /* SRB control [0x3200 ~ 0x3213]                        */
pub const OV5640_GROUP_ADDR0: u16 = 0x3200;
pub const OV5640_GROUP_ADDR1: u16 = 0x3201;
pub const OV5640_GROUP_ADDR2: u16 = 0x3202;
pub const OV5640_GROUP_ADDR3: u16 = 0x3203;
pub const OV5640_SRM_GROUP_ACCESS: u16 = 0x3212;
pub const OV5640_SRM_GROUP_STATUS: u16 = 0x3213;
pub const OV5640_AWB_R_GAIN_MSB: u16 = 0x3400;
pub const OV5640_AWB_R_GAIN_LSB: u16 = 0x3401;
pub const OV5640_AWB_G_GAIN_MSB: u16 = 0x3402;
pub const OV5640_AWB_G_GAIN_LSB: u16 = 0x3403;
pub const OV5640_AWB_B_GAIN_MSB: u16 = 0x3404;
pub const OV5640_AWB_B_GAIN_LSB: u16 = 0x3405;
pub const OV5640_AWB_MANUAL_CONTROL: u16 = 0x3406;
pub const OV5640_AEC_PK_EXPOSURE_19_16: u16 = 0x3500;
pub const OV5640_AEC_PK_EXPOSURE_HIGH: u16 = 0x3501;
pub const OV5640_AEC_PK_EXPOSURE_LOW: u16 = 0x3502;
pub const OV5640_AEC_PK_MANUAL: u16 = 0x3503;
pub const OV5640_AEC_PK_REAL_GAIN_9_8: u16 = 0x350A;
pub const OV5640_AEC_PK_REAL_GAIN_LOW: u16 = 0x350B;
pub const OV5640_AEC_PK_VTS_HIGH: u16 = 0x350C;
pub const OV5640_AEC_PK_VTS_LOW: u16 = 0x350D;
pub const OV5640_VCM_CONTROL_0: u16 = 0x3602;
pub const OV5640_VCM_CONTROL_1: u16 = 0x3603;
pub const OV5640_VCM_CONTROL_2: u16 = 0x3604;
pub const OV5640_VCM_CONTROL_3: u16 = 0x3605;
pub const OV5640_VCM_CONTROL_4: u16 = 0x3606;
pub const OV5640_TIMING_HS_HIGH: u16 = 0x3800;
pub const OV5640_TIMING_HS_LOW: u16 = 0x3801;
pub const OV5640_TIMING_VS_HIGH: u16 = 0x3802;
pub const OV5640_TIMING_VS_LOW: u16 = 0x3803;
pub const OV5640_TIMING_HW_HIGH: u16 = 0x3804;
pub const OV5640_TIMING_HW_LOW: u16 = 0x3805;
pub const OV5640_TIMING_VH_HIGH: u16 = 0x3806;
pub const OV5640_TIMING_VH_LOW: u16 = 0x3807;
pub const OV5640_TIMING_DVPHO_HIGH: u16 = 0x3808;
pub const OV5640_TIMING_DVPHO_LOW: u16 = 0x3809;
pub const OV5640_TIMING_DVPVO_HIGH: u16 = 0x380A;
pub const OV5640_TIMING_DVPVO_LOW: u16 = 0x380B;
pub const OV5640_TIMING_HTS_HIGH: u16 = 0x380C;
pub const OV5640_TIMING_HTS_LOW: u16 = 0x380D;
pub const OV5640_TIMING_VTS_HIGH: u16 = 0x380E;
pub const OV5640_TIMING_VTS_LOW: u16 = 0x380F;
pub const OV5640_TIMING_HOFFSET_HIGH: u16 = 0x3810;
pub const OV5640_TIMING_HOFFSET_LOW: u16 = 0x3811;
pub const OV5640_TIMING_VOFFSET_HIGH: u16 = 0x3812;
pub const OV5640_TIMING_VOFFSET_LOW: u16 = 0x3813;
pub const OV5640_TIMING_X_INC: u16 = 0x3814;
pub const OV5640_TIMING_Y_INC: u16 = 0x3815;
pub const OV5640_HSYNC_START_HIGH: u16 = 0x3816;
pub const OV5640_HSYNC_START_LOW: u16 = 0x3817;
pub const OV5640_HSYNC_WIDTH_HIGH: u16 = 0x3818;
pub const OV5640_HSYNC_WIDTH_LOW: u16 = 0x3819;
pub const OV5640_TIMING_TC_REG20: u16 = 0x3820;
pub const OV5640_TIMING_TC_REG21: u16 = 0x3821;
pub const OV5640_AEC_CTRL00: u16 = 0x3A00;
pub const OV5640_AEC_CTRL01: u16 = 0x3A01;
pub const OV5640_AEC_CTRL02: u16 = 0x3A02;
pub const OV5640_AEC_CTRL03: u16 = 0x3A03;
pub const OV5640_AEC_CTRL04: u16 = 0x3A04;
pub const OV5640_AEC_CTRL05: u16 = 0x3A05;
pub const OV5640_AEC_CTRL06: u16 = 0x3A06;
pub const OV5640_AEC_CTRL07: u16 = 0x3A07;
pub const OV5640_AEC_B50_STEP_HIGH: u16 = 0x3A08;
pub const OV5640_AEC_B50_STEP_LOW: u16 = 0x3A09;
pub const OV5640_AEC_B60_STEP_HIGH: u16 = 0x3A0A;
pub const OV5640_AEC_B60_STEP_LOW: u16 = 0x3A0B;
pub const OV5640_AEC_AEC_CTRL0C: u16 = 0x3A0C;
pub const OV5640_AEC_CTRL0D: u16 = 0x3A0D;
pub const OV5640_AEC_CTRL0E: u16 = 0x3A0E;
pub const OV5640_AEC_CTRL0F: u16 = 0x3A0F;
pub const OV5640_AEC_CTRL10: u16 = 0x3A10;
pub const OV5640_AEC_CTRL11: u16 = 0x3A11;
pub const OV5640_AEC_CTRL13: u16 = 0x3A13;
pub const OV5640_AEC_MAX_EXPO_HIGH: u16 = 0x3A14;
pub const OV5640_AEC_MAX_EXPO_LOW: u16 = 0x3A15;
pub const OV5640_AEC_CTRL17: u16 = 0x3A17;
pub const OV5640_AEC_GAIN_CEILING_HIGH: u16 = 0x3A18;
pub const OV5640_AEC_GAIN_CEILING_LOW: u16 = 0x3A19;
pub const OV5640_AEC_DIFF_MIN: u16 = 0x3A1A;
pub const OV5640_AEC_CTRL1B: u16 = 0x3A1B;
pub const OV5640_LED_ADD_ROW_HIGH: u16 = 0x3A1C;
pub const OV5640_LED_ADD_ROW_LOW: u16 = 0x3A1D;
pub const OV5640_AEC_CTRL1E: u16 = 0x3A1E;
pub const OV5640_AEC_CTRL1F: u16 = 0x3A1F;
pub const OV5640_AEC_CTRL20: u16 = 0x3A20;
pub const OV5640_AEC_CTRL21: u16 = 0x3A21;
pub const OV5640_AEC_CTRL25: u16 = 0x3A25;
pub const OV5640_STROBE_CTRL: u16 = 0x3B00;
pub const OV5640_FREX_EXPOSURE02: u16 = 0x3B01;
pub const OV5640_FREX_SHUTTER_DLY01: u16 = 0x3B02;
pub const OV5640_FREX_SHUTTER_DLY00: u16 = 0x3B03;
pub const OV5640_FREX_EXPOSURE01: u16 = 0x3B04;
pub const OV5640_FREX_EXPOSURE00: u16 = 0x3B05;
pub const OV5640_FREX_CTRL07: u16 = 0x3B06;
pub const OV5640_FREX_MODE: u16 = 0x3B07;
pub const OV5640_FREX_RQST: u16 = 0x3B08;
pub const OV5640_FREX_HREF_DLY: u16 = 0x3B09;
pub const OV5640_FREX_RST_LENGTH: u16 = 0x3B0A;
pub const OV5640_STROBE_WIDTH_HIGH: u16 = 0x3B0B;
pub const OV5640_STROBE_WIDTH_LOW: u16 = 0x3B0C;
pub const OV5640_5060HZ_CTRL00: u16 = 0x3C00;
pub const OV5640_5060HZ_CTRL01: u16 = 0x3C01;
pub const OV5640_5060HZ_CTRL02: u16 = 0x3C02;
pub const OV5640_5060HZ_CTRL03: u16 = 0x3C03;
pub const OV5640_5060HZ_CTRL04: u16 = 0x3C04;
pub const OV5640_5060HZ_CTRL05: u16 = 0x3C05;
pub const OV5640_LIGHTMETER1_TH_HIGH: u16 = 0x3C06;
pub const OV5640_LIGHTMETER1_TH_LOW: u16 = 0x3C07;
pub const OV5640_LIGHTMETER2_TH_HIGH: u16 = 0x3C08;
pub const OV5640_LIGHTMETER2_TH_LOW: u16 = 0x3C09;
pub const OV5640_SAMPLE_NUMBER_HIGH: u16 = 0x3C0A;
pub const OV5640_SAMPLE_NUMBER_LOW: u16 = 0x3C0B;
pub const OV5640_SIGMA_DELTA_CTRL0C: u16 = 0x3C0C;
pub const OV5640_SUM50_BYTE4: u16 = 0x3C0D;
pub const OV5640_SUM50_BYTE3: u16 = 0x3C0E;
pub const OV5640_SUM50_BYTE2: u16 = 0x3C0F;
pub const OV5640_SUM50_BYTE1: u16 = 0x3C10;
pub const OV5640_SUM60_BYTE4: u16 = 0x3C11;
pub const OV5640_SUM60_BYTE3: u16 = 0x3C12;
pub const OV5640_SUM60_BYTE2: u16 = 0x3C13;
pub const OV5640_SUM60_BYTE1: u16 = 0x3C14;
pub const OV5640_SUM5060_HIGH: u16 = 0x3C15;
pub const OV5640_SUM5060_LOW: u16 = 0x3C16;
pub const OV5640_BLOCK_CNTR_HIGH: u16 = 0x3C17;
pub const OV5640_BLOCK_CNTR_LOW: u16 = 0x3C18;
pub const OV5640_B6_HIGH: u16 = 0x3C19;
pub const OV5640_B6_LOW: u16 = 0x3C1A;
pub const OV5640_LIGHTMETER_OUTPUT_BYTE3: u16 = 0x3C1B;
pub const OV5640_LIGHTMETER_OUTPUT_BYTE2: u16 = 0x3C1C;
pub const OV5640_LIGHTMETER_OUTPUT_BYTE1: u16 = 0x3C1D;
pub const OV5640_SUM_THRESHOLD: u16 = 0x3C1E;
pub const OV5640_BLC_CTRL00: u16 = 0x4000;
pub const OV5640_BLC_CTRL01: u16 = 0x4001;
pub const OV5640_BLC_CTRL02: u16 = 0x4002;
pub const OV5640_BLC_CTRL03: u16 = 0x4003;
pub const OV5640_BLC_CTRL04: u16 = 0x4004;
pub const OV5640_BLC_CTRL05: u16 = 0x4005;
pub const OV5640_FRAME_CTRL01: u16 = 0x4201;
pub const OV5640_FRAME_CTRL02: u16 = 0x4202;
pub const OV5640_FORMAT_CTRL00: u16 = 0x4300;
pub const OV5640_FORMAT_CTRL01: u16 = 0x4301;
pub const OV5640_YMAX_VAL_HIGH: u16 = 0x4302;
pub const OV5640_YMAX_VAL_LOW: u16 = 0x4303;
pub const OV5640_YMIN_VAL_HIGH: u16 = 0x4304;
pub const OV5640_YMIN_VAL_LOW: u16 = 0x4305;
pub const OV5640_UMAX_VAL_HIGH: u16 = 0x4306;
pub const OV5640_UMAX_VAL_LOW: u16 = 0x4307;
pub const OV5640_UMIN_VAL_HIGH: u16 = 0x4308;
pub const OV5640_UMIN_VAL_LOW: u16 = 0x4309;
pub const OV5640_VMAX_VAL_HIGH: u16 = 0x430A;
pub const OV5640_VMAX_VAL_LOW: u16 = 0x430B;
pub const OV5640_VMIN_VAL_HIGH: u16 = 0x430C;
pub const OV5640_VMIN_VAL_LOW: u16 = 0x430D;
pub const OV5640_JPEG_CTRL00: u16 = 0x4400;
pub const OV5640_JPEG_CTRL01: u16 = 0x4401;
pub const OV5640_JPEG_CTRL02: u16 = 0x4402;
pub const OV5640_JPEG_CTRL03: u16 = 0x4403;
pub const OV5640_JPEG_CTRL04: u16 = 0x4404;
pub const OV5640_JPEG_CTRL05: u16 = 0x4405;
pub const OV5640_JPEG_CTRL06: u16 = 0x4406;
pub const OV5640_JPEG_CTRL07: u16 = 0x4407;
pub const OV5640_JPEG_ISI_CTRL1: u16 = 0x4408;
pub const OV5640_JPEG_CTRL09: u16 = 0x4409;
pub const OV5640_JPEG_CTRL0A: u16 = 0x440A;
pub const OV5640_JPEG_CTRL0B: u16 = 0x440B;
pub const OV5640_JPEG_CTRL0C: u16 = 0x440C;
pub const OV5640_JPEG_QT_DATA: u16 = 0x4410;
pub const OV5640_JPEG_QT_ADDR: u16 = 0x4411;
pub const OV5640_JPEG_ISI_DATA: u16 = 0x4412;
pub const OV5640_JPEG_ISI_CTRL2: u16 = 0x4413;
pub const OV5640_JPEG_LENGTH_BYTE3: u16 = 0x4414;
pub const OV5640_JPEG_LENGTH_BYTE2: u16 = 0x4415;
pub const OV5640_JPEG_LENGTH_BYTE1: u16 = 0x4416;
pub const OV5640_JFIFO_OVERFLOW: u16 = 0x4417;
pub const OV5640_VFIFO_CTRL00: u16 = 0x4600;
pub const OV5640_VFIFO_HSIZE_HIGH: u16 = 0x4602;
pub const OV5640_VFIFO_HSIZE_LOW: u16 = 0x4603;
pub const OV5640_VFIFO_VSIZE_HIGH: u16 = 0x4604;
pub const OV5640_VFIFO_VSIZE_LOW: u16 = 0x4605;
pub const OV5640_VFIFO_CTRL0C: u16 = 0x460C;
pub const OV5640_VFIFO_CTRL0D: u16 = 0x460D;
pub const OV5640_DVP_VSYNC_WIDTH0: u16 = 0x4709;
pub const OV5640_DVP_VSYNC_WIDTH1: u16 = 0x470A;
pub const OV5640_DVP_VSYNC_WIDTH2: u16 = 0x470B;
pub const OV5640_PAD_LEFT_CTRL: u16 = 0x4711;
pub const OV5640_PAD_RIGHT_CTRL: u16 = 0x4712;
pub const OV5640_JPG_MODE_SELECT: u16 = 0x4713;
pub const OV5640_656_DUMMY_LINE: u16 = 0x4715;
pub const OV5640_CCIR656_CTRL: u16 = 0x4719;
pub const OV5640_HSYNC_CTRL00: u16 = 0x471B;
pub const OV5640_DVP_VSYN_CTRL: u16 = 0x471D;
pub const OV5640_DVP_HREF_CTRL: u16 = 0x471F;
pub const OV5640_VSTART_OFFSET: u16 = 0x4721;
pub const OV5640_VEND_OFFSET: u16 = 0x4722;
pub const OV5640_DVP_CTRL23: u16 = 0x4723;
pub const OV5640_CCIR656_CTRL00: u16 = 0x4730;
pub const OV5640_CCIR656_CTRL01: u16 = 0x4731;
pub const OV5640_CCIR656_FS: u16 = 0x4732;
pub const OV5640_CCIR656_FE: u16 = 0x4733;
pub const OV5640_CCIR656_LS: u16 = 0x4734;
pub const OV5640_CCIR656_LE: u16 = 0x4735;
pub const OV5640_CCIR656_CTRL06: u16 = 0x4736;
pub const OV5640_CCIR656_CTRL07: u16 = 0x4737;
pub const OV5640_CCIR656_CTRL08: u16 = 0x4738;
pub const OV5640_POLARITY_CTRL: u16 = 0x4740;
pub const OV5640_TEST_PATTERN: u16 = 0x4741;
pub const OV5640_DATA_ORDER: u16 = 0x4745;
pub const OV5640_MIPI_CTRL00: u16 = 0x4800;
pub const OV5640_MIPI_CTRL01: u16 = 0x4801;
pub const OV5640_MIPI_CTRL05: u16 = 0x4805;
pub const OV5640_MIPI_DATA_ORDER: u16 = 0x480A;
pub const OV5640_MIN_HS_ZERO_HIGH: u16 = 0x4818;
pub const OV5640_MIN_HS_ZERO_LOW: u16 = 0x4819;
pub const OV5640_MIN_MIPI_HS_TRAIL_HIGH: u16 = 0x481A;
pub const OV5640_MIN_MIPI_HS_TRAIL_LOW: u16 = 0x481B;
pub const OV5640_MIN_MIPI_CLK_ZERO_HIGH: u16 = 0x481C;
pub const OV5640_MIN_MIPI_CLK_ZERO_LOW: u16 = 0x481D;
pub const OV5640_MIN_MIPI_CLK_PREPARE_HIGH: u16 = 0x481E;
pub const OV5640_MIN_MIPI_CLK_PREPARE_LOW: u16 = 0x481F;
pub const OV5640_MIN_CLK_POST_HIGH: u16 = 0x4820;
pub const OV5640_MIN_CLK_POST_LOW: u16 = 0x4821;
pub const OV5640_MIN_CLK_TRAIL_HIGH: u16 = 0x4822;
pub const OV5640_MIN_CLK_TRAIL_LOW: u16 = 0x4823;
pub const OV5640_MIN_LPX_PCLK_HIGH: u16 = 0x4824;
pub const OV5640_MIN_LPX_PCLK_LOW: u16 = 0x4825;
pub const OV5640_MIN_HS_PREPARE_HIGH: u16 = 0x4826;
pub const OV5640_MIN_HS_PREPARE_LOW: u16 = 0x4827;
pub const OV5640_MIN_HS_EXIT_HIGH: u16 = 0x4828;
pub const OV5640_MIN_HS_EXIT_LOW: u16 = 0x4829;
pub const OV5640_MIN_HS_ZERO_UI: u16 = 0x482A;
pub const OV5640_MIN_HS_TRAIL_UI: u16 = 0x482B;
pub const OV5640_MIN_CLK_ZERO_UI: u16 = 0x482C;
pub const OV5640_MIN_CLK_PREPARE_UI: u16 = 0x482D;
pub const OV5640_MIN_CLK_POST_UI: u16 = 0x482E;
pub const OV5640_MIN_CLK_TRAIL_UI: u16 = 0x482F;
pub const OV5640_MIN_LPX_PCLK_UI: u16 = 0x4830;
pub const OV5640_MIN_HS_PREPARE_UI: u16 = 0x4831;
pub const OV5640_MIN_HS_EXIT_UI: u16 = 0x4832;
pub const OV5640_PCLK_PERIOD: u16 = 0x4837;
pub const OV5640_ISP_FRAME_CTRL01: u16 = 0x4901;
pub const OV5640_ISP_FRAME_CTRL02: u16 = 0x4902;
pub const OV5640_ISP_CONTROL00: u16 = 0x5000;
pub const OV5640_ISP_CONTROL01: u16 = 0x5001;
pub const OV5640_ISP_CONTROL03: u16 = 0x5003;
pub const OV5640_ISP_CONTROL05: u16 = 0x5005;
pub const OV5640_ISP_MISC0: u16 = 0x501D;
pub const OV5640_ISP_MISC1: u16 = 0x501E;
pub const OV5640_FORMAT_MUX_CTRL: u16 = 0x501F;
pub const OV5640_DITHER_CTRL0: u16 = 0x5020;
pub const OV5640_DRAW_WINDOW_CTRL00: u16 = 0x5027;
pub const OV5640_DRAW_WINDOW_LEFT_CTRL_HIGH: u16 = 0x5028;
pub const OV5640_DRAW_WINDOW_LEFT_CTRL_LOW: u16 = 0x5029;
pub const OV5640_DRAW_WINDOW_RIGHT_CTRL_HIGH: u16 = 0x502A;
pub const OV5640_DRAW_WINDOW_RIGHT_CTRL_LOW: u16 = 0x502B;
pub const OV5640_DRAW_WINDOW_TOP_CTRL_HIGH: u16 = 0x502C;
pub const OV5640_DRAW_WINDOW_TOP_CTRL_LOW: u16 = 0x502D;
pub const OV5640_DRAW_WINDOW_BOTTOM_CTRL_HIGH: u16 = 0x502E;
pub const OV5640_DRAW_WINDOW_BOTTOM_CTRL_LOW: u16 = 0x502F;
pub const OV5640_DRAW_WINDOW_HBW_CTRL_HIGH: u16 = 0x5030; /* HBW: Horizontal Boundary Width */
pub const OV5640_DRAW_WINDOW_HBW_CTRL_LOW: u16 = 0x5031;
pub const OV5640_DRAW_WINDOW_VBW_CTRL_HIGH: u16 = 0x5032; /* VBW: Vertical Boundary Width */
pub const OV5640_DRAW_WINDOW_VBW_CTRL_LOW: u16 = 0x5033;
pub const OV5640_DRAW_WINDOW_Y_CTRL: u16 = 0x5034;
pub const OV5640_DRAW_WINDOW_U_CTRL: u16 = 0x5035;
pub const OV5640_DRAW_WINDOW_V_CTRL: u16 = 0x5036;
pub const OV5640_PRE_ISP_TEST_SETTING1: u16 = 0x503D;
pub const OV5640_ISP_SENSOR_BIAS_I: u16 = 0x5061;
pub const OV5640_ISP_SENSOR_GAIN1_I: u16 = 0x5062;
pub const OV5640_ISP_SENSOR_GAIN2_I: u16 = 0x5063;
pub const OV5640_AWB_CTRL00: u16 = 0x5180;
pub const OV5640_AWB_CTRL01: u16 = 0x5181;
pub const OV5640_AWB_CTRL02: u16 = 0x5182;
pub const OV5640_AWB_CTRL03: u16 = 0x5183;
pub const OV5640_AWB_CTRL04: u16 = 0x5184;
pub const OV5640_AWB_CTRL05: u16 = 0x5185;
pub const OV5640_AWB_CTRL06: u16 = 0x5186; /* Advanced AWB control registers: 0x5186 ~ 0x5190 */
pub const OV5640_AWB_CTRL07: u16 = 0x5187;
pub const OV5640_AWB_CTRL08: u16 = 0x5188;
pub const OV5640_AWB_CTRL09: u16 = 0x5189;
pub const OV5640_AWB_CTRL10: u16 = 0x518A;
pub const OV5640_AWB_CTRL11: u16 = 0x518B;
pub const OV5640_AWB_CTRL12: u16 = 0x518C;
pub const OV5640_AWB_CTRL13: u16 = 0x518D;
pub const OV5640_AWB_CTRL14: u16 = 0x518E;
pub const OV5640_AWB_CTRL15: u16 = 0x518F;
pub const OV5640_AWB_CTRL16: u16 = 0x5190;
pub const OV5640_AWB_CTRL17: u16 = 0x5191;
pub const OV5640_AWB_CTRL18: u16 = 0x5192;
pub const OV5640_AWB_CTRL19: u16 = 0x5193;
pub const OV5640_AWB_CTRL20: u16 = 0x5194;
pub const OV5640_AWB_CTRL21: u16 = 0x5195;
pub const OV5640_AWB_CTRL22: u16 = 0x5196;
pub const OV5640_AWB_CTRL23: u16 = 0x5197;
pub const OV5640_AWB_CTRL24: u16 = 0x5198;
pub const OV5640_AWB_CTRL25: u16 = 0x5199;
pub const OV5640_AWB_CTRL26: u16 = 0x519A;
pub const OV5640_AWB_CTRL27: u16 = 0x519B;
pub const OV5640_AWB_CTRL28: u16 = 0x519C;
pub const OV5640_AWB_CTRL29: u16 = 0x519D;
pub const OV5640_AWB_CTRL30: u16 = 0x519E;
pub const OV5640_AWB_CURRENT_R_GAIN_HIGH: u16 = 0x519F;
pub const OV5640_AWB_CURRENT_R_GAIN_LOW: u16 = 0x51A0;
pub const OV5640_AWB_CURRENT_G_GAIN_HIGH: u16 = 0x51A1;
pub const OV5640_AWB_CURRENT_G_GAIN_LOW: u16 = 0x51A2;
pub const OV5640_AWB_CURRENT_B_GAIN_HIGH: u16 = 0x51A3;
pub const OV5640_AWB_CURRENT_B_GAIN_LOW: u16 = 0x51A4;
pub const OV5640_AWB_AVERAGE_R: u16 = 0x51A5;
pub const OV5640_AWB_AVERAGE_G: u16 = 0x51A6;
pub const OV5640_AWB_AVERAGE_B: u16 = 0x51A7;
pub const OV5640_AWB_CTRL74: u16 = 0x5180;
pub const OV5640_CIP_SHARPENMT_TH1: u16 = 0x5300;
pub const OV5640_CIP_SHARPENMT_TH2: u16 = 0x5301;
pub const OV5640_CIP_SHARPENMT_OFFSET1: u16 = 0x5302;
pub const OV5640_CIP_SHARPENMT_OFFSET2: u16 = 0x5303;
pub const OV5640_CIP_DNS_TH1: u16 = 0x5304;
pub const OV5640_CIP_DNS_TH2: u16 = 0x5305;
pub const OV5640_CIP_DNS_OFFSET1: u16 = 0x5306;
pub const OV5640_CIP_DNS_OFFSET2: u16 = 0x5307;
pub const OV5640_CIP_CTRL: u16 = 0x5308;
pub const OV5640_CIP_SHARPENTH_TH1: u16 = 0x5309;
pub const OV5640_CIP_SHARPENTH_TH2: u16 = 0x530A;
pub const OV5640_CIP_SHARPENTH_OFFSET1: u16 = 0x530B;
pub const OV5640_CIP_SHARPENTH_OFFSET2: u16 = 0x530C;
pub const OV5640_CIP_EDGE_MT_AUTO: u16 = 0x530D;
pub const OV5640_CIP_DNS_TH_AUTO: u16 = 0x530E;
pub const OV5640_CIP_SHARPEN_TH_AUTO: u16 = 0x530F;
pub const OV5640_CMX_CTRL: u16 = 0x5380;
pub const OV5640_CMX1: u16 = 0x5381;
pub const OV5640_CMX2: u16 = 0x5382;
pub const OV5640_CMX3: u16 = 0x5383;
pub const OV5640_CMX4: u16 = 0x5384;
pub const OV5640_CMX5: u16 = 0x5385;
pub const OV5640_CMX6: u16 = 0x5386;
pub const OV5640_CMX7: u16 = 0x5387;
pub const OV5640_CMX8: u16 = 0x5388;
pub const OV5640_CMX9: u16 = 0x5389;
pub const OV5640_CMXSIGN_HIGH: u16 = 0x538A;
pub const OV5640_CMXSIGN_LOW: u16 = 0x538B;
pub const OV5640_GAMMA_CTRL00: u16 = 0x5480;
pub const OV5640_GAMMA_YST00: u16 = 0x5481;
pub const OV5640_GAMMA_YST01: u16 = 0x5482;
pub const OV5640_GAMMA_YST02: u16 = 0x5483;
pub const OV5640_GAMMA_YST03: u16 = 0x5484;
pub const OV5640_GAMMA_YST04: u16 = 0x5485;
pub const OV5640_GAMMA_YST05: u16 = 0x5486;
pub const OV5640_GAMMA_YST06: u16 = 0x5487;
pub const OV5640_GAMMA_YST07: u16 = 0x5488;
pub const OV5640_GAMMA_YST08: u16 = 0x5489;
pub const OV5640_GAMMA_YST09: u16 = 0x548A;
pub const OV5640_GAMMA_YST0A: u16 = 0x548B;
pub const OV5640_GAMMA_YST0B: u16 = 0x548C;
pub const OV5640_GAMMA_YST0C: u16 = 0x548D;
pub const OV5640_GAMMA_YST0D: u16 = 0x548E;
pub const OV5640_GAMMA_YST0E: u16 = 0x548F;
pub const OV5640_GAMMA_YST0F: u16 = 0x5490;
pub const OV5640_SDE_CTRL0: u16 = 0x5580;
pub const OV5640_SDE_CTRL1: u16 = 0x5581;
pub const OV5640_SDE_CTRL2: u16 = 0x5582;
pub const OV5640_SDE_CTRL3: u16 = 0x5583;
pub const OV5640_SDE_CTRL4: u16 = 0x5584;
pub const OV5640_SDE_CTRL5: u16 = 0x5585;
pub const OV5640_SDE_CTRL6: u16 = 0x5586;
pub const OV5640_SDE_CTRL7: u16 = 0x5587;
pub const OV5640_SDE_CTRL8: u16 = 0x5588;
pub const OV5640_SDE_CTRL9: u16 = 0x5589;
pub const OV5640_SDE_CTRL10: u16 = 0x558A;
pub const OV5640_SDE_CTRL11: u16 = 0x558B;
pub const OV5640_SDE_CTRL12: u16 = 0x558C;
pub const OV5640_SCALE_CTRL0: u16 = 0x5600;
pub const OV5640_SCALE_CTRL1: u16 = 0x5601;
pub const OV5640_SCALE_CTRL2: u16 = 0x5602;
pub const OV5640_SCALE_CTRL3: u16 = 0x5603;
pub const OV5640_SCALE_CTRL4: u16 = 0x5604;
pub const OV5640_SCALE_CTRL5: u16 = 0x5605;
pub const OV5640_SCALE_CTRL6: u16 = 0x5606;
pub const OV5640_X_START_HIGH: u16 = 0x5680;
pub const OV5640_X_START_LOW: u16 = 0x5681;
pub const OV5640_Y_START_HIGH: u16 = 0x5682;
pub const OV5640_Y_START_LOW: u16 = 0x5683;
pub const OV5640_X_WINDOW_HIGH: u16 = 0x5684;
pub const OV5640_X_WINDOW_LOW: u16 = 0x5685;
pub const OV5640_Y_WINDOW_HIGH: u16 = 0x5686;
pub const OV5640_Y_WINDOW_LOW: u16 = 0x5687;
pub const OV5640_WEIGHT00: u16 = 0x5688;
pub const OV5640_WEIGHT01: u16 = 0x5689;
pub const OV5640_WEIGHT02: u16 = 0x568A;
pub const OV5640_WEIGHT03: u16 = 0x568B;
pub const OV5640_WEIGHT04: u16 = 0x568C;
pub const OV5640_WEIGHT05: u16 = 0x568D;
pub const OV5640_WEIGHT06: u16 = 0x568E;
pub const OV5640_WEIGHT07: u16 = 0x568F;
pub const OV5640_AVG_CTRL10: u16 = 0x5690;
pub const OV5640_AVG_WIN_00: u16 = 0x5691;
pub const OV5640_AVG_WIN_01: u16 = 0x5692;
pub const OV5640_AVG_WIN_02: u16 = 0x5693;
pub const OV5640_AVG_WIN_03: u16 = 0x5694;
pub const OV5640_AVG_WIN_10: u16 = 0x5695;
pub const OV5640_AVG_WIN_11: u16 = 0x5696;
pub const OV5640_AVG_WIN_12: u16 = 0x5697;
pub const OV5640_AVG_WIN_13: u16 = 0x5698;
pub const OV5640_AVG_WIN_20: u16 = 0x5699;
pub const OV5640_AVG_WIN_21: u16 = 0x569A;
pub const OV5640_AVG_WIN_22: u16 = 0x569B;
pub const OV5640_AVG_WIN_23: u16 = 0x569C;
pub const OV5640_AVG_WIN_30: u16 = 0x569D;
pub const OV5640_AVG_WIN_31: u16 = 0x569E;
pub const OV5640_AVG_WIN_32: u16 = 0x569F;
pub const OV5640_AVG_WIN_33: u16 = 0x56A0;
pub const OV5640_AVG_READOUT: u16 = 0x56A1;
pub const OV5640_AVG_WEIGHT_SUM: u16 = 0x56A2;
pub const OV5640_GMTRX00: u16 = 0x5800;
pub const OV5640_GMTRX01: u16 = 0x5801;
pub const OV5640_GMTRX02: u16 = 0x5802;
pub const OV5640_GMTRX03: u16 = 0x5803;
pub const OV5640_GMTRX04: u16 = 0x5804;
pub const OV5640_GMTRX05: u16 = 0x5805;
pub const OV5640_GMTRX10: u16 = 0x5806;
pub const OV5640_GMTRX11: u16 = 0x5807;
pub const OV5640_GMTRX12: u16 = 0x5808;
pub const OV5640_GMTRX13: u16 = 0x5809;
pub const OV5640_GMTRX14: u16 = 0x580A;
pub const OV5640_GMTRX15: u16 = 0x580B;
pub const OV5640_GMTRX20: u16 = 0x580C;
pub const OV5640_GMTRX21: u16 = 0x580D;
pub const OV5640_GMTRX22: u16 = 0x580E;
pub const OV5640_GMTRX23: u16 = 0x580F;
pub const OV5640_GMTRX24: u16 = 0x5810;
pub const OV5640_GMTRX25: u16 = 0x5811;
pub const OV5640_GMTRX30: u16 = 0x5812;
pub const OV5640_GMTRX31: u16 = 0x5813;
pub const OV5640_GMTRX32: u16 = 0x5814;
pub const OV5640_GMTRX33: u16 = 0x5815;
pub const OV5640_GMTRX34: u16 = 0x5816;
pub const OV5640_GMTRX35: u16 = 0x5817;
pub const OV5640_GMTRX40: u16 = 0x5818;
pub const OV5640_GMTRX41: u16 = 0x5819;
pub const OV5640_GMTRX42: u16 = 0x581A;
pub const OV5640_GMTRX43: u16 = 0x581B;
pub const OV5640_GMTRX44: u16 = 0x581C;
pub const OV5640_GMTRX45: u16 = 0x581D;
pub const OV5640_GMTRX50: u16 = 0x581E;
pub const OV5640_GMTRX51: u16 = 0x581F;
pub const OV5640_GMTRX52: u16 = 0x5820;
pub const OV5640_GMTRX53: u16 = 0x5821;
pub const OV5640_GMTRX54: u16 = 0x5822;
pub const OV5640_GMTRX55: u16 = 0x5823;
pub const OV5640_BRMATRX00: u16 = 0x5824;
pub const OV5640_BRMATRX01: u16 = 0x5825;
pub const OV5640_BRMATRX02: u16 = 0x5826;
pub const OV5640_BRMATRX03: u16 = 0x5827;
pub const OV5640_BRMATRX04: u16 = 0x5828;
pub const OV5640_BRMATRX05: u16 = 0x5829;
pub const OV5640_BRMATRX06: u16 = 0x582A;
pub const OV5640_BRMATRX07: u16 = 0x582B;
pub const OV5640_BRMATRX08: u16 = 0x582C;
pub const OV5640_BRMATRX09: u16 = 0x582D;
pub const OV5640_BRMATRX20: u16 = 0x582E;
pub const OV5640_BRMATRX21: u16 = 0x582F;
pub const OV5640_BRMATRX22: u16 = 0x5830;
pub const OV5640_BRMATRX23: u16 = 0x5831;
pub const OV5640_BRMATRX24: u16 = 0x5832;
pub const OV5640_BRMATRX30: u16 = 0x5833;
pub const OV5640_BRMATRX31: u16 = 0x5834;
pub const OV5640_BRMATRX32: u16 = 0x5835;
pub const OV5640_BRMATRX33: u16 = 0x5836;
pub const OV5640_BRMATRX34: u16 = 0x5837;
pub const OV5640_BRMATRX40: u16 = 0x5838;
pub const OV5640_BRMATRX41: u16 = 0x5839;
pub const OV5640_BRMATRX42: u16 = 0x583A;
pub const OV5640_BRMATRX43: u16 = 0x583B;
pub const OV5640_BRMATRX44: u16 = 0x583C;
pub const OV5640_LENC_BR_OFFSET: u16 = 0x583D;
pub const OV5640_MAX_GAIN: u16 = 0x583E;
pub const OV5640_MIN_GAIN: u16 = 0x583F;
pub const OV5640_MIN_Q: u16 = 0x5840;
pub const OV5640_LENC_CTRL59: u16 = 0x5841;
pub const OV5640_BR_HSCALE_HIGH: u16 = 0x5842;
pub const OV5640_BR_HSCALE_LOW: u16 = 0x5843;
pub const OV5640_BR_VSCALE_HIGH: u16 = 0x5844;
pub const OV5640_BR_VSCALE_LOW: u16 = 0x5845;
pub const OV5640_G_HSCALE_HIGH: u16 = 0x5846;
pub const OV5640_G_HSCALE_LOW: u16 = 0x5847;
pub const OV5640_G_VSCALE_HIGH: u16 = 0x5848;
pub const OV5640_G_VSCALE_LOW: u16 = 0x5849;
pub const OV5640_AFC_CTRL00: u16 = 0x6000;
pub const OV5640_AFC_CTRL01: u16 = 0x6001;
pub const OV5640_AFC_CTRL02: u16 = 0x6002;
pub const OV5640_AFC_CTRL03: u16 = 0x6003;
pub const OV5640_AFC_CTRL04: u16 = 0x6004;
pub const OV5640_AFC_CTRL05: u16 = 0x6005;
pub const OV5640_AFC_CTRL06: u16 = 0x6006;
pub const OV5640_AFC_CTRL07: u16 = 0x6007;
pub const OV5640_AFC_CTRL08: u16 = 0x6008;
pub const OV5640_AFC_CTRL09: u16 = 0x6009;
pub const OV5640_AFC_CTRL10: u16 = 0x600A;
pub const OV5640_AFC_CTRL11: u16 = 0x600B;
pub const OV5640_AFC_CTRL12: u16 = 0x600C;
pub const OV5640_AFC_CTRL13: u16 = 0x600D;
pub const OV5640_AFC_CTRL14: u16 = 0x600E;
pub const OV5640_AFC_CTRL15: u16 = 0x600F;
pub const OV5640_AFC_CTRL16: u16 = 0x6010;
pub const OV5640_AFC_CTRL17: u16 = 0x6011;
pub const OV5640_AFC_CTRL18: u16 = 0x6012;
pub const OV5640_AFC_CTRL19: u16 = 0x6013;
pub const OV5640_AFC_CTRL20: u16 = 0x6014;
pub const OV5640_AFC_CTRL21: u16 = 0x6015;
pub const OV5640_AFC_CTRL22: u16 = 0x6016;
pub const OV5640_AFC_CTRL23: u16 = 0x6017;
pub const OV5640_AFC_CTRL24: u16 = 0x6018;
pub const OV5640_AFC_CTRL25: u16 = 0x6019;
pub const OV5640_AFC_CTRL26: u16 = 0x601A;
pub const OV5640_AFC_CTRL27: u16 = 0x601B;
pub const OV5640_AFC_CTRL28: u16 = 0x601C;
pub const OV5640_AFC_CTRL29: u16 = 0x601D;
pub const OV5640_AFC_CTRL30: u16 = 0x601E;
pub const OV5640_AFC_CTRL31: u16 = 0x601F;
pub const OV5640_AFC_CTRL32: u16 = 0x6020;
pub const OV5640_AFC_CTRL33: u16 = 0x6021;
pub const OV5640_AFC_CTRL34: u16 = 0x6022;
pub const OV5640_AFC_CTRL35: u16 = 0x6023;
pub const OV5640_AFC_CTRL36: u16 = 0x6024;
pub const OV5640_AFC_CTRL37: u16 = 0x6025;
pub const OV5640_AFC_CTRL38: u16 = 0x6026;
pub const OV5640_AFC_CTRL39: u16 = 0x6027;
pub const OV5640_AFC_CTRL40: u16 = 0x6028;
pub const OV5640_AFC_CTRL41: u16 = 0x6029;
pub const OV5640_AFC_CTRL42: u16 = 0x602A;
pub const OV5640_AFC_CTRL43: u16 = 0x602B;
pub const OV5640_AFC_CTRL44: u16 = 0x602C;
pub const OV5640_AFC_CTRL45: u16 = 0x602D;
pub const OV5640_AFC_CTRL46: u16 = 0x602E;
pub const OV5640_AFC_CTRL47: u16 = 0x602F;
pub const OV5640_AFC_CTRL48: u16 = 0x6030;
pub const OV5640_AFC_CTRL49: u16 = 0x6031;
pub const OV5640_AFC_CTRL50: u16 = 0x6032;
pub const OV5640_AFC_CTRL51: u16 = 0x6033;
pub const OV5640_AFC_CTRL52: u16 = 0x6034;
pub const OV5640_AFC_CTRL53: u16 = 0x6035;
pub const OV5640_AFC_CTRL54: u16 = 0x6036;
pub const OV5640_AFC_CTRL55: u16 = 0x6037;
pub const OV5640_AFC_CTRL56: u16 = 0x6038;
pub const OV5640_AFC_CTRL57: u16 = 0x6039;
pub const OV5640_AFC_CTRL58: u16 = 0x603A;
pub const OV5640_AFC_CTRL59: u16 = 0x603B;
pub const OV5640_AFC_CTRL60: u16 = 0x603C;
pub const OV5640_AFC_READ58: u16 = 0x603D;
pub const OV5640_AFC_READ59: u16 = 0x603E;
pub const OV5640_AFC_READ60: u16 = 0x603F;
pub const OV5640_PF_JPEG: &[(u16, u16)] = &[
    /*  SET PIXEL FORMAT: JPEG */
    (OV5640_FORMAT_CTRL00, 0x30),
    (OV5640_FORMAT_MUX_CTRL, 0x00),
    // set resolution
    (OV5640_TIMING_DVPHO_HIGH, 0x07),
    (OV5640_TIMING_DVPHO_LOW, 0x80),
    (OV5640_TIMING_DVPVO_HIGH, 0x04),
    (OV5640_TIMING_DVPVO_LOW, 0x38),
];

pub const OV5640_COMMON: &[(u16, u16)] = &[
    (OV5640_SCCB_SYSTEM_CTRL1, 0x11),
    (OV5640_SYSTEM_CTROL0, 0x82),
    (OV5640_SCCB_SYSTEM_CTRL1, 0x03),
    (OV5640_PAD_OUTPUT_ENABLE01, 0xFF),
    (OV5640_PAD_OUTPUT_ENABLE02, 0xf3),
    (OV5640_SC_PLL_CONTRL0, 0x18),
    (OV5640_SYSTEM_CTROL0, 0x02),
    (OV5640_SC_PLL_CONTRL1, 0x41),
    (OV5640_SC_PLL_CONTRL2, 0x30),
    (OV5640_SC_PLL_CONTRL3, 0x13),
    (OV5640_SYSTEM_ROOT_DIVIDER, 0x01),
    (0x3630, 0x36),
    (0x3631, 0x0e),
    (0x3632, 0xe2),
    (0x3633, 0x12),
    (0x3621, 0xe0),
    (0x3704, 0xa0),
    (0x3703, 0x5a),
    (0x3715, 0x78),
    (0x3717, 0x01),
    (0x370b, 0x60),
    (0x3705, 0x1a),
    (0x3905, 0x02),
    (0x3906, 0x10),
    (0x3901, 0x0a),
    (0x3731, 0x12),
    (0x3600, 0x08),
    (0x3601, 0x33),
    (0x302d, 0x60),
    (0x3620, 0x52),
    (0x371b, 0x20),
    (0x471c, 0x50),
    (OV5640_AEC_CTRL13, 0x43),
    (OV5640_AEC_GAIN_CEILING_HIGH, 0x00),
    (OV5640_AEC_GAIN_CEILING_LOW, 0xf8),
    (0x3635, 0x13),
    (0x3636, 0x03),
    (0x3634, 0x40),
    (0x3622, 0x01),
    (OV5640_5060HZ_CTRL01, 0x34),
    (OV5640_5060HZ_CTRL04, 0x28),
    (OV5640_5060HZ_CTRL05, 0x98),
    (OV5640_LIGHTMETER1_TH_HIGH, 0x00),
    (OV5640_LIGHTMETER1_TH_LOW, 0x00),
    (OV5640_LIGHTMETER2_TH_HIGH, 0x01),
    (OV5640_LIGHTMETER2_TH_LOW, 0x2c),
    (OV5640_SAMPLE_NUMBER_HIGH, 0x9c),
    (OV5640_SAMPLE_NUMBER_LOW, 0x40),
    (OV5640_TIMING_TC_REG20, 0x06),
    (OV5640_TIMING_TC_REG21, 0x00),
    (0x3618, 0x00),
    (0x3612, 0x29),
    (0x3708, 0x64),
    (0x3709, 0x52),
    (0x370c, 0x03),
    (OV5640_AEC_CTRL02, 0x03),
    (OV5640_AEC_CTRL03, 0xd8),
    (OV5640_AEC_B50_STEP_HIGH, 0x01),
    (OV5640_AEC_B50_STEP_LOW, 0x27),
    (OV5640_AEC_B60_STEP_HIGH, 0x00),
    (OV5640_AEC_B60_STEP_LOW, 0xf6),
    (OV5640_AEC_CTRL0E, 0x03),
    (OV5640_AEC_CTRL0D, 0x04),
    (OV5640_AEC_MAX_EXPO_HIGH, 0x03),
    (OV5640_AEC_MAX_EXPO_LOW, 0xd8),
    (OV5640_BLC_CTRL01, 0x02),
    (OV5640_BLC_CTRL04, 0x02),
    (OV5640_SYSREM_RESET00, 0x00),
    (OV5640_SYSREM_RESET02, 0x1c),
    (OV5640_CLOCK_ENABLE00, 0xff),
    (OV5640_CLOCK_ENABLE02, 0xc3),
    (OV5640_MIPI_CONTROL00, 0x58),
    (0x302e, 0x00),
    (OV5640_POLARITY_CTRL, 0x22),
    (OV5640_FORMAT_CTRL00, 0x6F),
    (OV5640_FORMAT_MUX_CTRL, 0x01),
    (OV5640_JPG_MODE_SELECT, 0x03),
    (OV5640_JPEG_CTRL07, 0x04),
    (0x440e, 0x00),
    (0x460b, 0x35),
    (0x460c, 0x23),
    (OV5640_PCLK_PERIOD, 0x22),
    (0x3824, 0x02),
    (OV5640_ISP_CONTROL00, 0xa7),
    (OV5640_ISP_CONTROL01, 0xa3),
    (OV5640_AWB_CTRL00, 0xff),
    (OV5640_AWB_CTRL01, 0xf2),
    (OV5640_AWB_CTRL02, 0x00),
    (OV5640_AWB_CTRL03, 0x14),
    (OV5640_AWB_CTRL04, 0x25),
    (OV5640_AWB_CTRL05, 0x24),
    (OV5640_AWB_CTRL06, 0x09),
    (OV5640_AWB_CTRL07, 0x09),
    (OV5640_AWB_CTRL08, 0x09),
    (OV5640_AWB_CTRL09, 0x75),
    (OV5640_AWB_CTRL10, 0x54),
    (OV5640_AWB_CTRL11, 0xe0),
    (OV5640_AWB_CTRL12, 0xb2),
    (OV5640_AWB_CTRL13, 0x42),
    (OV5640_AWB_CTRL14, 0x3d),
    (OV5640_AWB_CTRL15, 0x56),
    (OV5640_AWB_CTRL16, 0x46),
    (OV5640_AWB_CTRL17, 0xf8),
    (OV5640_AWB_CTRL18, 0x04),
    (OV5640_AWB_CTRL19, 0x70),
    (OV5640_AWB_CTRL20, 0xf0),
    (OV5640_AWB_CTRL21, 0xf0),
    (OV5640_AWB_CTRL22, 0x03),
    (OV5640_AWB_CTRL23, 0x01),
    (OV5640_AWB_CTRL24, 0x04),
    (OV5640_AWB_CTRL25, 0x12),
    (OV5640_AWB_CTRL26, 0x04),
    (OV5640_AWB_CTRL27, 0x00),
    (OV5640_AWB_CTRL28, 0x06),
    (OV5640_AWB_CTRL29, 0x82),
    (OV5640_AWB_CTRL30, 0x38),
    (OV5640_CMX1, 0x1e),
    (OV5640_CMX2, 0x5b),
    (OV5640_CMX3, 0x08),
    (OV5640_CMX4, 0x0a),
    (OV5640_CMX5, 0x7e),
    (OV5640_CMX6, 0x88),
    (OV5640_CMX7, 0x7c),
    (OV5640_CMX8, 0x6c),
    (OV5640_CMX9, 0x10),
    (OV5640_CMXSIGN_HIGH, 0x01),
    (OV5640_CMXSIGN_LOW, 0x98),
    (OV5640_CIP_SHARPENMT_TH1, 0x08),
    (OV5640_CIP_SHARPENMT_TH2, 0x30),
    (OV5640_CIP_SHARPENMT_OFFSET1, 0x10),
    (OV5640_CIP_SHARPENMT_OFFSET2, 0x00),
    (OV5640_CIP_DNS_TH1, 0x08),
    (OV5640_CIP_DNS_TH2, 0x30),
    (OV5640_CIP_DNS_OFFSET1, 0x08),
    (OV5640_CIP_DNS_OFFSET2, 0x16),
    (OV5640_CIP_CTRL, 0x08),
    (OV5640_CIP_SHARPENTH_TH1, 0x30),
    (OV5640_CIP_SHARPENTH_TH2, 0x04),
    (OV5640_CIP_SHARPENTH_OFFSET1, 0x06),
    (OV5640_GAMMA_CTRL00, 0x01),
    (OV5640_GAMMA_YST00, 0x08),
    (OV5640_GAMMA_YST01, 0x14),
    (OV5640_GAMMA_YST02, 0x28),
    (OV5640_GAMMA_YST03, 0x51),
    (OV5640_GAMMA_YST04, 0x65),
    (OV5640_GAMMA_YST05, 0x71),
    (OV5640_GAMMA_YST06, 0x7d),
    (OV5640_GAMMA_YST07, 0x87),
    (OV5640_GAMMA_YST08, 0x91),
    (OV5640_GAMMA_YST09, 0x9a),
    (OV5640_GAMMA_YST0A, 0xaa),
    (OV5640_GAMMA_YST0B, 0xb8),
    (OV5640_GAMMA_YST0C, 0xcd),
    (OV5640_GAMMA_YST0D, 0xdd),
    (OV5640_GAMMA_YST0E, 0xea),
    (OV5640_GAMMA_YST0F, 0x1d),
    (OV5640_SDE_CTRL0, 0x02),
    (OV5640_SDE_CTRL3, 0x40),
    (OV5640_SDE_CTRL4, 0x10),
    (OV5640_SDE_CTRL9, 0x10),
    (OV5640_SDE_CTRL10, 0x00),
    (OV5640_SDE_CTRL11, 0xf8),
    (OV5640_GMTRX00, 0x23),
    (OV5640_GMTRX01, 0x14),
    (OV5640_GMTRX02, 0x0f),
    (OV5640_GMTRX03, 0x0f),
    (OV5640_GMTRX04, 0x12),
    (OV5640_GMTRX05, 0x26),
    (OV5640_GMTRX10, 0x0c),
    (OV5640_GMTRX11, 0x08),
    (OV5640_GMTRX12, 0x05),
    (OV5640_GMTRX13, 0x05),
    (OV5640_GMTRX14, 0x08),
    (OV5640_GMTRX15, 0x0d),
    (OV5640_GMTRX20, 0x08),
    (OV5640_GMTRX21, 0x03),
    (OV5640_GMTRX22, 0x00),
    (OV5640_GMTRX23, 0x00),
    (OV5640_GMTRX24, 0x03),
    (OV5640_GMTRX25, 0x09),
    (OV5640_GMTRX30, 0x07),
    (OV5640_GMTRX31, 0x03),
    (OV5640_GMTRX32, 0x00),
    (OV5640_GMTRX33, 0x01),
    (OV5640_GMTRX34, 0x03),
    (OV5640_GMTRX35, 0x08),
    (OV5640_GMTRX40, 0x0d),
    (OV5640_GMTRX41, 0x08),
    (OV5640_GMTRX42, 0x05),
    (OV5640_GMTRX43, 0x06),
    (OV5640_GMTRX44, 0x08),
    (OV5640_GMTRX45, 0x0e),
    (OV5640_GMTRX50, 0x29),
    (OV5640_GMTRX51, 0x17),
    (OV5640_GMTRX52, 0x11),
    (OV5640_GMTRX53, 0x11),
    (OV5640_GMTRX54, 0x15),
    (OV5640_GMTRX55, 0x28),
    (OV5640_BRMATRX00, 0x46),
    (OV5640_BRMATRX01, 0x26),
    (OV5640_BRMATRX02, 0x08),
    (OV5640_BRMATRX03, 0x26),
    (OV5640_BRMATRX04, 0x64),
    (OV5640_BRMATRX05, 0x26),
    (OV5640_BRMATRX06, 0x24),
    (OV5640_BRMATRX07, 0x22),
    (OV5640_BRMATRX08, 0x24),
    (OV5640_BRMATRX09, 0x24),
    (OV5640_BRMATRX20, 0x06),
    (OV5640_BRMATRX21, 0x22),
    (OV5640_BRMATRX22, 0x40),
    (OV5640_BRMATRX23, 0x42),
    (OV5640_BRMATRX24, 0x24),
    (OV5640_BRMATRX30, 0x26),
    (OV5640_BRMATRX31, 0x24),
    (OV5640_BRMATRX32, 0x22),
    (OV5640_BRMATRX33, 0x22),
    (OV5640_BRMATRX34, 0x26),
    (OV5640_BRMATRX40, 0x44),
    (OV5640_BRMATRX41, 0x24),
    (OV5640_BRMATRX42, 0x26),
    (OV5640_BRMATRX43, 0x28),
    (OV5640_BRMATRX44, 0x42),
    (OV5640_LENC_BR_OFFSET, 0xce),
    (0x5025, 0x00),
    (OV5640_AEC_CTRL0F, 0x30),
    (OV5640_AEC_CTRL10, 0x28),
    (OV5640_AEC_CTRL1B, 0x30),
    (OV5640_AEC_CTRL1E, 0x26),
    (OV5640_AEC_CTRL11, 0x60),
    (OV5640_AEC_CTRL1F, 0x14),
    (OV5640_SYSTEM_CTROL0, 0x02),
];
pub const OV5640_JPEG_MODE: &[(u16, u16)] = &[
    // (OV5640_TIMING_TC_REG21,  0x20),
    // (OV5640_SYSREM_RESET02, 0),
    // (OV5640_CLOCK_ENABLE02, 0x28 ),
];
pub const OV5640_WVGA: &[(u16, u16)] = &[
    (OV5640_TIMING_DVPHO_HIGH, 0x03),
    (OV5640_TIMING_DVPHO_LOW, 0x20),
    (OV5640_TIMING_DVPVO_HIGH, 0x01),
    (OV5640_TIMING_DVPVO_LOW, 0xE0),
];
