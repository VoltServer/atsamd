#[doc = "Register `CHANNEL` reader"]
pub type R = crate::R<ChannelSpec>;
#[doc = "Register `CHANNEL` writer"]
pub type W = crate::W<ChannelSpec>;
#[doc = "Event Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evgenselect {
    #[doc = "0: No event generator selected"]
    None = 0,
    #[doc = "1: XOSC0 fail detection"]
    OscctrlXoscFail0 = 1,
    #[doc = "2: XOSC1 fail detection"]
    OscctrlXoscFail1 = 2,
    #[doc = "3: XOSC32K fail detection"]
    Osc32kctrlXosc32kFail = 3,
    #[doc = "4: RTC period 0"]
    RtcPer0 = 4,
    #[doc = "5: RTC period 1"]
    RtcPer1 = 5,
    #[doc = "6: RTC period 2"]
    RtcPer2 = 6,
    #[doc = "7: RTC period 3"]
    RtcPer3 = 7,
    #[doc = "8: RTC period 4"]
    RtcPer4 = 8,
    #[doc = "9: RTC period 5"]
    RtcPer5 = 9,
    #[doc = "10: RTC period 6"]
    RtcPer6 = 10,
    #[doc = "11: RTC period 7"]
    RtcPer7 = 11,
    #[doc = "12: RTC comparison 0"]
    RtcCmp0 = 12,
    #[doc = "13: RTC comparison 1"]
    RtcCmp1 = 13,
    #[doc = "14: RTC comparison 2"]
    RtcCmp2 = 14,
    #[doc = "15: RTC comparison 3"]
    RtcCmp3 = 15,
    #[doc = "16: RTC tamper detection"]
    RtcTamper = 16,
    #[doc = "17: RTC overflow"]
    RtcOvf = 17,
    #[doc = "18: EIC external interrupt 0"]
    EicExtint0 = 18,
    #[doc = "19: EIC external interrupt 1"]
    EicExtint1 = 19,
    #[doc = "20: EIC external interrupt 2"]
    EicExtint2 = 20,
    #[doc = "21: EIC external interrupt 3"]
    EicExtint3 = 21,
    #[doc = "22: EIC external interrupt 4"]
    EicExtint4 = 22,
    #[doc = "23: EIC external interrupt 5"]
    EicExtint5 = 23,
    #[doc = "24: EIC external interrupt 6"]
    EicExtint6 = 24,
    #[doc = "25: EIC external interrupt 7"]
    EicExtint7 = 25,
    #[doc = "26: EIC external interrupt 8"]
    EicExtint8 = 26,
    #[doc = "27: EIC external interrupt 9"]
    EicExtint9 = 27,
    #[doc = "28: EIC external interrupt 10"]
    EicExtint10 = 28,
    #[doc = "29: EIC external interrupt 11"]
    EicExtint11 = 29,
    #[doc = "30: EIC external interrupt 12"]
    EicExtint12 = 30,
    #[doc = "31: EIC external interrupt 13"]
    EicExtint13 = 31,
    #[doc = "32: EIC external interrupt 14"]
    EicExtint14 = 32,
    #[doc = "33: EIC external interrupt 15"]
    EicExtint15 = 33,
    #[doc = "34: DMA channel 0"]
    DmacCh0 = 34,
    #[doc = "35: DMA channel 1"]
    DmacCh1 = 35,
    #[doc = "36: DMA channel 2"]
    DmacCh2 = 36,
    #[doc = "37: DMA channel 3"]
    DmacCh3 = 37,
    #[doc = "38: PAC Acc. error"]
    PacAccerr = 38,
    #[doc = "41: TCC0 Overflow"]
    Tcc0Ovf = 41,
    #[doc = "42: TCC0 Trigger Event"]
    Tcc0Trg = 42,
    #[doc = "43: TCC0 Counter"]
    Tcc0Cnt = 43,
    #[doc = "44: TCC0 Match/Compare 0"]
    Tcc0Mc0 = 44,
    #[doc = "45: TCC0 Match/Compare 1"]
    Tcc0Mc1 = 45,
    #[doc = "46: TCC0 Match/Compare 2"]
    Tcc0Mc2 = 46,
    #[doc = "47: TCC0 Match/Compare 3"]
    Tcc0Mc3 = 47,
    #[doc = "48: TCC0 Match/Compare 4"]
    Tcc0Mc4 = 48,
    #[doc = "49: TCC0 Match/Compare 5"]
    Tcc0Mc5 = 49,
    #[doc = "50: TCC1 Overflow"]
    Tcc1Ovf = 50,
    #[doc = "51: TCC1 Trigger Event"]
    Tcc1Trg = 51,
    #[doc = "52: TCC1 Counter"]
    Tcc1Cnt = 52,
    #[doc = "53: TCC1 Match/Compare 0"]
    Tcc1Mc0 = 53,
    #[doc = "54: TCC1 Match/Compare 1"]
    Tcc1Mc1 = 54,
    #[doc = "55: TCC1 Match/Compare 2"]
    Tcc1Mc2 = 55,
    #[doc = "56: TCC1 Match/Compare 3"]
    Tcc1Mc3 = 56,
    #[doc = "57: TCC2 Overflow"]
    Tcc2Ovf = 57,
    #[doc = "58: TCC2 Trigger Event"]
    Tcc2Trg = 58,
    #[doc = "59: TCC2 Counter"]
    Tcc2Cnt = 59,
    #[doc = "60: TCC2 Match/Compare 0"]
    Tcc2Mc0 = 60,
    #[doc = "61: TCC2 Match/Compare 1"]
    Tcc2Mc1 = 61,
    #[doc = "62: TCC2 Match/Compare 2"]
    Tcc2Mc2 = 62,
    #[doc = "63: TCC3 Overflow"]
    Tcc3Ovf = 63,
    #[doc = "64: TCC3 Trigger Event"]
    Tcc3Trg = 64,
    #[doc = "65: TCC3 Counter"]
    Tcc3Cnt = 65,
    #[doc = "66: TCC3 Match/Compare 0"]
    Tcc3Mc0 = 66,
    #[doc = "67: TCC3 Match/Compare 1"]
    Tcc3Mc1 = 67,
    #[doc = "68: TCC4 Overflow"]
    Tcc4Ovf = 68,
    #[doc = "69: TCC4 Trigger Event"]
    Tcc4Trg = 69,
    #[doc = "70: TCC4 Counter"]
    Tcc4Cnt = 70,
    #[doc = "71: TCC4 Match/Compare 0"]
    Tcc4Mc0 = 71,
    #[doc = "72: TCC4 Match/Compare 1"]
    Tcc4Mc1 = 72,
    #[doc = "73: TC0 Overflow"]
    Tc0Ovf = 73,
    #[doc = "74: TC0 Match/Compare 0"]
    Tc0Mc0 = 74,
    #[doc = "75: TC0 Match/Compare 1"]
    Tc0Mc1 = 75,
    #[doc = "76: TC1 Overflow"]
    Tc1Ovf = 76,
    #[doc = "77: TC1 Match/Compare 0"]
    Tc1Mc0 = 77,
    #[doc = "78: TC1 Match/Compare 1"]
    Tc1Mc1 = 78,
    #[doc = "79: TC2 Overflow"]
    Tc2Ovf = 79,
    #[doc = "80: TC2 Match/Compare 0"]
    Tc2Mc0 = 80,
    #[doc = "81: TC2 Match/Compare 1"]
    Tc2Mc1 = 81,
    #[doc = "82: TC3 Overflow"]
    Tc3Ovf = 82,
    #[doc = "83: TC3 Match/Compare 0"]
    Tc3Mc0 = 83,
    #[doc = "84: TC3 Match/Compare 1"]
    Tc3Mc1 = 84,
    #[doc = "85: TC4 Overflow"]
    Tc4Ovf = 85,
    #[doc = "86: TC4 Match/Compare 0"]
    Tc4Mc0 = 86,
    #[doc = "87: TC4 Match/Compare 1"]
    Tc4Mc1 = 87,
    #[doc = "88: TC5 Overflow"]
    Tc5Ovf = 88,
    #[doc = "89: TC5 Match/Compare 0"]
    Tc5Mc0 = 89,
    #[doc = "90: TC5 Match/Compare 1"]
    Tc5Mc1 = 90,
    #[doc = "91: TC6 Overflow"]
    Tc6Ovf = 91,
    #[doc = "92: TC6 Match/Compare 0"]
    Tc6Mc0 = 92,
    #[doc = "93: TC6 Match/Compare 1"]
    Tc6Mc1 = 93,
    #[doc = "94: TC7 Overflow"]
    Tc7Ovf = 94,
    #[doc = "95: TC7 Match/Compare 0"]
    Tc7Mc0 = 95,
    #[doc = "96: TC7 Match/Compare 1"]
    Tc7Mc1 = 96,
    #[doc = "97: PDEC Overflow"]
    PedcOvf = 97,
    #[doc = "98: PDEC Error"]
    PedcErr = 98,
    #[doc = "99: PDEC Direction"]
    PedcDir = 99,
    #[doc = "100: PDEC VLC"]
    PedcVlc = 100,
    #[doc = "101: PDEC MC0"]
    PedcMc0 = 101,
    #[doc = "102: PDEC MC1"]
    PedcMc1 = 102,
    #[doc = "103: ADC0 RESRDY"]
    Adc0Resrdy = 103,
    #[doc = "104: ADC0 Window Monitor"]
    Adc0Winmon = 104,
    #[doc = "105: ADC1 RESRDY"]
    Adc1Resrdy = 105,
    #[doc = "106: ADC1 Window Monitor"]
    Adc1Winmon = 106,
    #[doc = "107: AC Comparator 0"]
    AcComp0 = 107,
    #[doc = "108: AC Comparator 1"]
    AcComp1 = 108,
    #[doc = "109: AC0 Window"]
    AcWin = 109,
    #[doc = "110: DAC empty 0"]
    DacEmpty0 = 110,
    #[doc = "111: DAC empty 1"]
    DacEmpty1 = 111,
    #[doc = "112: DAC RESRDY 0"]
    DacResrdy0 = 112,
    #[doc = "113: DAC RESRDY 1"]
    DacResrdy1 = 113,
    #[doc = "114: GMAC Timestamp CMP"]
    GmacTsuCmp = 114,
    #[doc = "115: TRNG Ready"]
    TrngReady = 115,
    #[doc = "116: CCL LUTOUT 0"]
    CclLutout0 = 116,
    #[doc = "117: CCL LUTOUT 1"]
    CclLutout1 = 117,
    #[doc = "118: CCL LUTOUT 2"]
    CclLutout2 = 118,
    #[doc = "119: CCL LUTOUT 3"]
    CclLutout3 = 119,
}
impl From<Evgenselect> for u8 {
    #[inline(always)]
    fn from(variant: Evgenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evgenselect {
    type Ux = u8;
}
impl crate::IsEnum for Evgenselect {}
#[doc = "Field `EVGEN` reader - Event Generator Selection"]
pub type EvgenR = crate::FieldReader<Evgenselect>;
impl EvgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evgenselect> {
        match self.bits {
            0 => Some(Evgenselect::None),
            1 => Some(Evgenselect::OscctrlXoscFail0),
            2 => Some(Evgenselect::OscctrlXoscFail1),
            3 => Some(Evgenselect::Osc32kctrlXosc32kFail),
            4 => Some(Evgenselect::RtcPer0),
            5 => Some(Evgenselect::RtcPer1),
            6 => Some(Evgenselect::RtcPer2),
            7 => Some(Evgenselect::RtcPer3),
            8 => Some(Evgenselect::RtcPer4),
            9 => Some(Evgenselect::RtcPer5),
            10 => Some(Evgenselect::RtcPer6),
            11 => Some(Evgenselect::RtcPer7),
            12 => Some(Evgenselect::RtcCmp0),
            13 => Some(Evgenselect::RtcCmp1),
            14 => Some(Evgenselect::RtcCmp2),
            15 => Some(Evgenselect::RtcCmp3),
            16 => Some(Evgenselect::RtcTamper),
            17 => Some(Evgenselect::RtcOvf),
            18 => Some(Evgenselect::EicExtint0),
            19 => Some(Evgenselect::EicExtint1),
            20 => Some(Evgenselect::EicExtint2),
            21 => Some(Evgenselect::EicExtint3),
            22 => Some(Evgenselect::EicExtint4),
            23 => Some(Evgenselect::EicExtint5),
            24 => Some(Evgenselect::EicExtint6),
            25 => Some(Evgenselect::EicExtint7),
            26 => Some(Evgenselect::EicExtint8),
            27 => Some(Evgenselect::EicExtint9),
            28 => Some(Evgenselect::EicExtint10),
            29 => Some(Evgenselect::EicExtint11),
            30 => Some(Evgenselect::EicExtint12),
            31 => Some(Evgenselect::EicExtint13),
            32 => Some(Evgenselect::EicExtint14),
            33 => Some(Evgenselect::EicExtint15),
            34 => Some(Evgenselect::DmacCh0),
            35 => Some(Evgenselect::DmacCh1),
            36 => Some(Evgenselect::DmacCh2),
            37 => Some(Evgenselect::DmacCh3),
            38 => Some(Evgenselect::PacAccerr),
            41 => Some(Evgenselect::Tcc0Ovf),
            42 => Some(Evgenselect::Tcc0Trg),
            43 => Some(Evgenselect::Tcc0Cnt),
            44 => Some(Evgenselect::Tcc0Mc0),
            45 => Some(Evgenselect::Tcc0Mc1),
            46 => Some(Evgenselect::Tcc0Mc2),
            47 => Some(Evgenselect::Tcc0Mc3),
            48 => Some(Evgenselect::Tcc0Mc4),
            49 => Some(Evgenselect::Tcc0Mc5),
            50 => Some(Evgenselect::Tcc1Ovf),
            51 => Some(Evgenselect::Tcc1Trg),
            52 => Some(Evgenselect::Tcc1Cnt),
            53 => Some(Evgenselect::Tcc1Mc0),
            54 => Some(Evgenselect::Tcc1Mc1),
            55 => Some(Evgenselect::Tcc1Mc2),
            56 => Some(Evgenselect::Tcc1Mc3),
            57 => Some(Evgenselect::Tcc2Ovf),
            58 => Some(Evgenselect::Tcc2Trg),
            59 => Some(Evgenselect::Tcc2Cnt),
            60 => Some(Evgenselect::Tcc2Mc0),
            61 => Some(Evgenselect::Tcc2Mc1),
            62 => Some(Evgenselect::Tcc2Mc2),
            63 => Some(Evgenselect::Tcc3Ovf),
            64 => Some(Evgenselect::Tcc3Trg),
            65 => Some(Evgenselect::Tcc3Cnt),
            66 => Some(Evgenselect::Tcc3Mc0),
            67 => Some(Evgenselect::Tcc3Mc1),
            68 => Some(Evgenselect::Tcc4Ovf),
            69 => Some(Evgenselect::Tcc4Trg),
            70 => Some(Evgenselect::Tcc4Cnt),
            71 => Some(Evgenselect::Tcc4Mc0),
            72 => Some(Evgenselect::Tcc4Mc1),
            73 => Some(Evgenselect::Tc0Ovf),
            74 => Some(Evgenselect::Tc0Mc0),
            75 => Some(Evgenselect::Tc0Mc1),
            76 => Some(Evgenselect::Tc1Ovf),
            77 => Some(Evgenselect::Tc1Mc0),
            78 => Some(Evgenselect::Tc1Mc1),
            79 => Some(Evgenselect::Tc2Ovf),
            80 => Some(Evgenselect::Tc2Mc0),
            81 => Some(Evgenselect::Tc2Mc1),
            82 => Some(Evgenselect::Tc3Ovf),
            83 => Some(Evgenselect::Tc3Mc0),
            84 => Some(Evgenselect::Tc3Mc1),
            85 => Some(Evgenselect::Tc4Ovf),
            86 => Some(Evgenselect::Tc4Mc0),
            87 => Some(Evgenselect::Tc4Mc1),
            88 => Some(Evgenselect::Tc5Ovf),
            89 => Some(Evgenselect::Tc5Mc0),
            90 => Some(Evgenselect::Tc5Mc1),
            91 => Some(Evgenselect::Tc6Ovf),
            92 => Some(Evgenselect::Tc6Mc0),
            93 => Some(Evgenselect::Tc6Mc1),
            94 => Some(Evgenselect::Tc7Ovf),
            95 => Some(Evgenselect::Tc7Mc0),
            96 => Some(Evgenselect::Tc7Mc1),
            97 => Some(Evgenselect::PedcOvf),
            98 => Some(Evgenselect::PedcErr),
            99 => Some(Evgenselect::PedcDir),
            100 => Some(Evgenselect::PedcVlc),
            101 => Some(Evgenselect::PedcMc0),
            102 => Some(Evgenselect::PedcMc1),
            103 => Some(Evgenselect::Adc0Resrdy),
            104 => Some(Evgenselect::Adc0Winmon),
            105 => Some(Evgenselect::Adc1Resrdy),
            106 => Some(Evgenselect::Adc1Winmon),
            107 => Some(Evgenselect::AcComp0),
            108 => Some(Evgenselect::AcComp1),
            109 => Some(Evgenselect::AcWin),
            110 => Some(Evgenselect::DacEmpty0),
            111 => Some(Evgenselect::DacEmpty1),
            112 => Some(Evgenselect::DacResrdy0),
            113 => Some(Evgenselect::DacResrdy1),
            114 => Some(Evgenselect::GmacTsuCmp),
            115 => Some(Evgenselect::TrngReady),
            116 => Some(Evgenselect::CclLutout0),
            117 => Some(Evgenselect::CclLutout1),
            118 => Some(Evgenselect::CclLutout2),
            119 => Some(Evgenselect::CclLutout3),
            _ => None,
        }
    }
    #[doc = "No event generator selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Evgenselect::None
    }
    #[doc = "XOSC0 fail detection"]
    #[inline(always)]
    pub fn is_oscctrl_xosc_fail0(&self) -> bool {
        *self == Evgenselect::OscctrlXoscFail0
    }
    #[doc = "XOSC1 fail detection"]
    #[inline(always)]
    pub fn is_oscctrl_xosc_fail1(&self) -> bool {
        *self == Evgenselect::OscctrlXoscFail1
    }
    #[doc = "XOSC32K fail detection"]
    #[inline(always)]
    pub fn is_osc32kctrl_xosc32k_fail(&self) -> bool {
        *self == Evgenselect::Osc32kctrlXosc32kFail
    }
    #[doc = "RTC period 0"]
    #[inline(always)]
    pub fn is_rtc_per0(&self) -> bool {
        *self == Evgenselect::RtcPer0
    }
    #[doc = "RTC period 1"]
    #[inline(always)]
    pub fn is_rtc_per1(&self) -> bool {
        *self == Evgenselect::RtcPer1
    }
    #[doc = "RTC period 2"]
    #[inline(always)]
    pub fn is_rtc_per2(&self) -> bool {
        *self == Evgenselect::RtcPer2
    }
    #[doc = "RTC period 3"]
    #[inline(always)]
    pub fn is_rtc_per3(&self) -> bool {
        *self == Evgenselect::RtcPer3
    }
    #[doc = "RTC period 4"]
    #[inline(always)]
    pub fn is_rtc_per4(&self) -> bool {
        *self == Evgenselect::RtcPer4
    }
    #[doc = "RTC period 5"]
    #[inline(always)]
    pub fn is_rtc_per5(&self) -> bool {
        *self == Evgenselect::RtcPer5
    }
    #[doc = "RTC period 6"]
    #[inline(always)]
    pub fn is_rtc_per6(&self) -> bool {
        *self == Evgenselect::RtcPer6
    }
    #[doc = "RTC period 7"]
    #[inline(always)]
    pub fn is_rtc_per7(&self) -> bool {
        *self == Evgenselect::RtcPer7
    }
    #[doc = "RTC comparison 0"]
    #[inline(always)]
    pub fn is_rtc_cmp0(&self) -> bool {
        *self == Evgenselect::RtcCmp0
    }
    #[doc = "RTC comparison 1"]
    #[inline(always)]
    pub fn is_rtc_cmp1(&self) -> bool {
        *self == Evgenselect::RtcCmp1
    }
    #[doc = "RTC comparison 2"]
    #[inline(always)]
    pub fn is_rtc_cmp2(&self) -> bool {
        *self == Evgenselect::RtcCmp2
    }
    #[doc = "RTC comparison 3"]
    #[inline(always)]
    pub fn is_rtc_cmp3(&self) -> bool {
        *self == Evgenselect::RtcCmp3
    }
    #[doc = "RTC tamper detection"]
    #[inline(always)]
    pub fn is_rtc_tamper(&self) -> bool {
        *self == Evgenselect::RtcTamper
    }
    #[doc = "RTC overflow"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == Evgenselect::RtcOvf
    }
    #[doc = "EIC external interrupt 0"]
    #[inline(always)]
    pub fn is_eic_extint0(&self) -> bool {
        *self == Evgenselect::EicExtint0
    }
    #[doc = "EIC external interrupt 1"]
    #[inline(always)]
    pub fn is_eic_extint1(&self) -> bool {
        *self == Evgenselect::EicExtint1
    }
    #[doc = "EIC external interrupt 2"]
    #[inline(always)]
    pub fn is_eic_extint2(&self) -> bool {
        *self == Evgenselect::EicExtint2
    }
    #[doc = "EIC external interrupt 3"]
    #[inline(always)]
    pub fn is_eic_extint3(&self) -> bool {
        *self == Evgenselect::EicExtint3
    }
    #[doc = "EIC external interrupt 4"]
    #[inline(always)]
    pub fn is_eic_extint4(&self) -> bool {
        *self == Evgenselect::EicExtint4
    }
    #[doc = "EIC external interrupt 5"]
    #[inline(always)]
    pub fn is_eic_extint5(&self) -> bool {
        *self == Evgenselect::EicExtint5
    }
    #[doc = "EIC external interrupt 6"]
    #[inline(always)]
    pub fn is_eic_extint6(&self) -> bool {
        *self == Evgenselect::EicExtint6
    }
    #[doc = "EIC external interrupt 7"]
    #[inline(always)]
    pub fn is_eic_extint7(&self) -> bool {
        *self == Evgenselect::EicExtint7
    }
    #[doc = "EIC external interrupt 8"]
    #[inline(always)]
    pub fn is_eic_extint8(&self) -> bool {
        *self == Evgenselect::EicExtint8
    }
    #[doc = "EIC external interrupt 9"]
    #[inline(always)]
    pub fn is_eic_extint9(&self) -> bool {
        *self == Evgenselect::EicExtint9
    }
    #[doc = "EIC external interrupt 10"]
    #[inline(always)]
    pub fn is_eic_extint10(&self) -> bool {
        *self == Evgenselect::EicExtint10
    }
    #[doc = "EIC external interrupt 11"]
    #[inline(always)]
    pub fn is_eic_extint11(&self) -> bool {
        *self == Evgenselect::EicExtint11
    }
    #[doc = "EIC external interrupt 12"]
    #[inline(always)]
    pub fn is_eic_extint12(&self) -> bool {
        *self == Evgenselect::EicExtint12
    }
    #[doc = "EIC external interrupt 13"]
    #[inline(always)]
    pub fn is_eic_extint13(&self) -> bool {
        *self == Evgenselect::EicExtint13
    }
    #[doc = "EIC external interrupt 14"]
    #[inline(always)]
    pub fn is_eic_extint14(&self) -> bool {
        *self == Evgenselect::EicExtint14
    }
    #[doc = "EIC external interrupt 15"]
    #[inline(always)]
    pub fn is_eic_extint15(&self) -> bool {
        *self == Evgenselect::EicExtint15
    }
    #[doc = "DMA channel 0"]
    #[inline(always)]
    pub fn is_dmac_ch0(&self) -> bool {
        *self == Evgenselect::DmacCh0
    }
    #[doc = "DMA channel 1"]
    #[inline(always)]
    pub fn is_dmac_ch1(&self) -> bool {
        *self == Evgenselect::DmacCh1
    }
    #[doc = "DMA channel 2"]
    #[inline(always)]
    pub fn is_dmac_ch2(&self) -> bool {
        *self == Evgenselect::DmacCh2
    }
    #[doc = "DMA channel 3"]
    #[inline(always)]
    pub fn is_dmac_ch3(&self) -> bool {
        *self == Evgenselect::DmacCh3
    }
    #[doc = "PAC Acc. error"]
    #[inline(always)]
    pub fn is_pac_accerr(&self) -> bool {
        *self == Evgenselect::PacAccerr
    }
    #[doc = "TCC0 Overflow"]
    #[inline(always)]
    pub fn is_tcc0_ovf(&self) -> bool {
        *self == Evgenselect::Tcc0Ovf
    }
    #[doc = "TCC0 Trigger Event"]
    #[inline(always)]
    pub fn is_tcc0_trg(&self) -> bool {
        *self == Evgenselect::Tcc0Trg
    }
    #[doc = "TCC0 Counter"]
    #[inline(always)]
    pub fn is_tcc0_cnt(&self) -> bool {
        *self == Evgenselect::Tcc0Cnt
    }
    #[doc = "TCC0 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tcc0_mc0(&self) -> bool {
        *self == Evgenselect::Tcc0Mc0
    }
    #[doc = "TCC0 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tcc0_mc1(&self) -> bool {
        *self == Evgenselect::Tcc0Mc1
    }
    #[doc = "TCC0 Match/Compare 2"]
    #[inline(always)]
    pub fn is_tcc0_mc2(&self) -> bool {
        *self == Evgenselect::Tcc0Mc2
    }
    #[doc = "TCC0 Match/Compare 3"]
    #[inline(always)]
    pub fn is_tcc0_mc3(&self) -> bool {
        *self == Evgenselect::Tcc0Mc3
    }
    #[doc = "TCC0 Match/Compare 4"]
    #[inline(always)]
    pub fn is_tcc0_mc4(&self) -> bool {
        *self == Evgenselect::Tcc0Mc4
    }
    #[doc = "TCC0 Match/Compare 5"]
    #[inline(always)]
    pub fn is_tcc0_mc5(&self) -> bool {
        *self == Evgenselect::Tcc0Mc5
    }
    #[doc = "TCC1 Overflow"]
    #[inline(always)]
    pub fn is_tcc1_ovf(&self) -> bool {
        *self == Evgenselect::Tcc1Ovf
    }
    #[doc = "TCC1 Trigger Event"]
    #[inline(always)]
    pub fn is_tcc1_trg(&self) -> bool {
        *self == Evgenselect::Tcc1Trg
    }
    #[doc = "TCC1 Counter"]
    #[inline(always)]
    pub fn is_tcc1_cnt(&self) -> bool {
        *self == Evgenselect::Tcc1Cnt
    }
    #[doc = "TCC1 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tcc1_mc0(&self) -> bool {
        *self == Evgenselect::Tcc1Mc0
    }
    #[doc = "TCC1 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tcc1_mc1(&self) -> bool {
        *self == Evgenselect::Tcc1Mc1
    }
    #[doc = "TCC1 Match/Compare 2"]
    #[inline(always)]
    pub fn is_tcc1_mc2(&self) -> bool {
        *self == Evgenselect::Tcc1Mc2
    }
    #[doc = "TCC1 Match/Compare 3"]
    #[inline(always)]
    pub fn is_tcc1_mc3(&self) -> bool {
        *self == Evgenselect::Tcc1Mc3
    }
    #[doc = "TCC2 Overflow"]
    #[inline(always)]
    pub fn is_tcc2_ovf(&self) -> bool {
        *self == Evgenselect::Tcc2Ovf
    }
    #[doc = "TCC2 Trigger Event"]
    #[inline(always)]
    pub fn is_tcc2_trg(&self) -> bool {
        *self == Evgenselect::Tcc2Trg
    }
    #[doc = "TCC2 Counter"]
    #[inline(always)]
    pub fn is_tcc2_cnt(&self) -> bool {
        *self == Evgenselect::Tcc2Cnt
    }
    #[doc = "TCC2 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tcc2_mc0(&self) -> bool {
        *self == Evgenselect::Tcc2Mc0
    }
    #[doc = "TCC2 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tcc2_mc1(&self) -> bool {
        *self == Evgenselect::Tcc2Mc1
    }
    #[doc = "TCC2 Match/Compare 2"]
    #[inline(always)]
    pub fn is_tcc2_mc2(&self) -> bool {
        *self == Evgenselect::Tcc2Mc2
    }
    #[doc = "TCC3 Overflow"]
    #[inline(always)]
    pub fn is_tcc3_ovf(&self) -> bool {
        *self == Evgenselect::Tcc3Ovf
    }
    #[doc = "TCC3 Trigger Event"]
    #[inline(always)]
    pub fn is_tcc3_trg(&self) -> bool {
        *self == Evgenselect::Tcc3Trg
    }
    #[doc = "TCC3 Counter"]
    #[inline(always)]
    pub fn is_tcc3_cnt(&self) -> bool {
        *self == Evgenselect::Tcc3Cnt
    }
    #[doc = "TCC3 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tcc3_mc0(&self) -> bool {
        *self == Evgenselect::Tcc3Mc0
    }
    #[doc = "TCC3 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tcc3_mc1(&self) -> bool {
        *self == Evgenselect::Tcc3Mc1
    }
    #[doc = "TCC4 Overflow"]
    #[inline(always)]
    pub fn is_tcc4_ovf(&self) -> bool {
        *self == Evgenselect::Tcc4Ovf
    }
    #[doc = "TCC4 Trigger Event"]
    #[inline(always)]
    pub fn is_tcc4_trg(&self) -> bool {
        *self == Evgenselect::Tcc4Trg
    }
    #[doc = "TCC4 Counter"]
    #[inline(always)]
    pub fn is_tcc4_cnt(&self) -> bool {
        *self == Evgenselect::Tcc4Cnt
    }
    #[doc = "TCC4 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tcc4_mc0(&self) -> bool {
        *self == Evgenselect::Tcc4Mc0
    }
    #[doc = "TCC4 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tcc4_mc1(&self) -> bool {
        *self == Evgenselect::Tcc4Mc1
    }
    #[doc = "TC0 Overflow"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == Evgenselect::Tc0Ovf
    }
    #[doc = "TC0 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc0_mc0(&self) -> bool {
        *self == Evgenselect::Tc0Mc0
    }
    #[doc = "TC0 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc0_mc1(&self) -> bool {
        *self == Evgenselect::Tc0Mc1
    }
    #[doc = "TC1 Overflow"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == Evgenselect::Tc1Ovf
    }
    #[doc = "TC1 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc1_mc0(&self) -> bool {
        *self == Evgenselect::Tc1Mc0
    }
    #[doc = "TC1 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc1_mc1(&self) -> bool {
        *self == Evgenselect::Tc1Mc1
    }
    #[doc = "TC2 Overflow"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == Evgenselect::Tc2Ovf
    }
    #[doc = "TC2 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc2_mc0(&self) -> bool {
        *self == Evgenselect::Tc2Mc0
    }
    #[doc = "TC2 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc2_mc1(&self) -> bool {
        *self == Evgenselect::Tc2Mc1
    }
    #[doc = "TC3 Overflow"]
    #[inline(always)]
    pub fn is_tc3_ovf(&self) -> bool {
        *self == Evgenselect::Tc3Ovf
    }
    #[doc = "TC3 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc3_mc0(&self) -> bool {
        *self == Evgenselect::Tc3Mc0
    }
    #[doc = "TC3 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc3_mc1(&self) -> bool {
        *self == Evgenselect::Tc3Mc1
    }
    #[doc = "TC4 Overflow"]
    #[inline(always)]
    pub fn is_tc4_ovf(&self) -> bool {
        *self == Evgenselect::Tc4Ovf
    }
    #[doc = "TC4 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc4_mc0(&self) -> bool {
        *self == Evgenselect::Tc4Mc0
    }
    #[doc = "TC4 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc4_mc1(&self) -> bool {
        *self == Evgenselect::Tc4Mc1
    }
    #[doc = "TC5 Overflow"]
    #[inline(always)]
    pub fn is_tc5_ovf(&self) -> bool {
        *self == Evgenselect::Tc5Ovf
    }
    #[doc = "TC5 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc5_mc0(&self) -> bool {
        *self == Evgenselect::Tc5Mc0
    }
    #[doc = "TC5 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc5_mc1(&self) -> bool {
        *self == Evgenselect::Tc5Mc1
    }
    #[doc = "TC6 Overflow"]
    #[inline(always)]
    pub fn is_tc6_ovf(&self) -> bool {
        *self == Evgenselect::Tc6Ovf
    }
    #[doc = "TC6 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc6_mc0(&self) -> bool {
        *self == Evgenselect::Tc6Mc0
    }
    #[doc = "TC6 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc6_mc1(&self) -> bool {
        *self == Evgenselect::Tc6Mc1
    }
    #[doc = "TC7 Overflow"]
    #[inline(always)]
    pub fn is_tc7_ovf(&self) -> bool {
        *self == Evgenselect::Tc7Ovf
    }
    #[doc = "TC7 Match/Compare 0"]
    #[inline(always)]
    pub fn is_tc7_mc0(&self) -> bool {
        *self == Evgenselect::Tc7Mc0
    }
    #[doc = "TC7 Match/Compare 1"]
    #[inline(always)]
    pub fn is_tc7_mc1(&self) -> bool {
        *self == Evgenselect::Tc7Mc1
    }
    #[doc = "PDEC Overflow"]
    #[inline(always)]
    pub fn is_pedc_ovf(&self) -> bool {
        *self == Evgenselect::PedcOvf
    }
    #[doc = "PDEC Error"]
    #[inline(always)]
    pub fn is_pedc_err(&self) -> bool {
        *self == Evgenselect::PedcErr
    }
    #[doc = "PDEC Direction"]
    #[inline(always)]
    pub fn is_pedc_dir(&self) -> bool {
        *self == Evgenselect::PedcDir
    }
    #[doc = "PDEC VLC"]
    #[inline(always)]
    pub fn is_pedc_vlc(&self) -> bool {
        *self == Evgenselect::PedcVlc
    }
    #[doc = "PDEC MC0"]
    #[inline(always)]
    pub fn is_pedc_mc0(&self) -> bool {
        *self == Evgenselect::PedcMc0
    }
    #[doc = "PDEC MC1"]
    #[inline(always)]
    pub fn is_pedc_mc1(&self) -> bool {
        *self == Evgenselect::PedcMc1
    }
    #[doc = "ADC0 RESRDY"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == Evgenselect::Adc0Resrdy
    }
    #[doc = "ADC0 Window Monitor"]
    #[inline(always)]
    pub fn is_adc0_winmon(&self) -> bool {
        *self == Evgenselect::Adc0Winmon
    }
    #[doc = "ADC1 RESRDY"]
    #[inline(always)]
    pub fn is_adc1_resrdy(&self) -> bool {
        *self == Evgenselect::Adc1Resrdy
    }
    #[doc = "ADC1 Window Monitor"]
    #[inline(always)]
    pub fn is_adc1_winmon(&self) -> bool {
        *self == Evgenselect::Adc1Winmon
    }
    #[doc = "AC Comparator 0"]
    #[inline(always)]
    pub fn is_ac_comp0(&self) -> bool {
        *self == Evgenselect::AcComp0
    }
    #[doc = "AC Comparator 1"]
    #[inline(always)]
    pub fn is_ac_comp1(&self) -> bool {
        *self == Evgenselect::AcComp1
    }
    #[doc = "AC0 Window"]
    #[inline(always)]
    pub fn is_ac_win(&self) -> bool {
        *self == Evgenselect::AcWin
    }
    #[doc = "DAC empty 0"]
    #[inline(always)]
    pub fn is_dac_empty0(&self) -> bool {
        *self == Evgenselect::DacEmpty0
    }
    #[doc = "DAC empty 1"]
    #[inline(always)]
    pub fn is_dac_empty1(&self) -> bool {
        *self == Evgenselect::DacEmpty1
    }
    #[doc = "DAC RESRDY 0"]
    #[inline(always)]
    pub fn is_dac_resrdy0(&self) -> bool {
        *self == Evgenselect::DacResrdy0
    }
    #[doc = "DAC RESRDY 1"]
    #[inline(always)]
    pub fn is_dac_resrdy1(&self) -> bool {
        *self == Evgenselect::DacResrdy1
    }
    #[doc = "GMAC Timestamp CMP"]
    #[inline(always)]
    pub fn is_gmac_tsu_cmp(&self) -> bool {
        *self == Evgenselect::GmacTsuCmp
    }
    #[doc = "TRNG Ready"]
    #[inline(always)]
    pub fn is_trng_ready(&self) -> bool {
        *self == Evgenselect::TrngReady
    }
    #[doc = "CCL LUTOUT 0"]
    #[inline(always)]
    pub fn is_ccl_lutout0(&self) -> bool {
        *self == Evgenselect::CclLutout0
    }
    #[doc = "CCL LUTOUT 1"]
    #[inline(always)]
    pub fn is_ccl_lutout1(&self) -> bool {
        *self == Evgenselect::CclLutout1
    }
    #[doc = "CCL LUTOUT 2"]
    #[inline(always)]
    pub fn is_ccl_lutout2(&self) -> bool {
        *self == Evgenselect::CclLutout2
    }
    #[doc = "CCL LUTOUT 3"]
    #[inline(always)]
    pub fn is_ccl_lutout3(&self) -> bool {
        *self == Evgenselect::CclLutout3
    }
}
#[doc = "Field `EVGEN` writer - Event Generator Selection"]
pub type EvgenW<'a, REG> = crate::FieldWriter<'a, REG, 7, Evgenselect>;
impl<'a, REG> EvgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event generator selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::None)
    }
    #[doc = "XOSC0 fail detection"]
    #[inline(always)]
    pub fn oscctrl_xosc_fail0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::OscctrlXoscFail0)
    }
    #[doc = "XOSC1 fail detection"]
    #[inline(always)]
    pub fn oscctrl_xosc_fail1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::OscctrlXoscFail1)
    }
    #[doc = "XOSC32K fail detection"]
    #[inline(always)]
    pub fn osc32kctrl_xosc32k_fail(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Osc32kctrlXosc32kFail)
    }
    #[doc = "RTC period 0"]
    #[inline(always)]
    pub fn rtc_per0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer0)
    }
    #[doc = "RTC period 1"]
    #[inline(always)]
    pub fn rtc_per1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer1)
    }
    #[doc = "RTC period 2"]
    #[inline(always)]
    pub fn rtc_per2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer2)
    }
    #[doc = "RTC period 3"]
    #[inline(always)]
    pub fn rtc_per3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer3)
    }
    #[doc = "RTC period 4"]
    #[inline(always)]
    pub fn rtc_per4(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer4)
    }
    #[doc = "RTC period 5"]
    #[inline(always)]
    pub fn rtc_per5(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer5)
    }
    #[doc = "RTC period 6"]
    #[inline(always)]
    pub fn rtc_per6(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer6)
    }
    #[doc = "RTC period 7"]
    #[inline(always)]
    pub fn rtc_per7(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcPer7)
    }
    #[doc = "RTC comparison 0"]
    #[inline(always)]
    pub fn rtc_cmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcCmp0)
    }
    #[doc = "RTC comparison 1"]
    #[inline(always)]
    pub fn rtc_cmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcCmp1)
    }
    #[doc = "RTC comparison 2"]
    #[inline(always)]
    pub fn rtc_cmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcCmp2)
    }
    #[doc = "RTC comparison 3"]
    #[inline(always)]
    pub fn rtc_cmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcCmp3)
    }
    #[doc = "RTC tamper detection"]
    #[inline(always)]
    pub fn rtc_tamper(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcTamper)
    }
    #[doc = "RTC overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::RtcOvf)
    }
    #[doc = "EIC external interrupt 0"]
    #[inline(always)]
    pub fn eic_extint0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint0)
    }
    #[doc = "EIC external interrupt 1"]
    #[inline(always)]
    pub fn eic_extint1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint1)
    }
    #[doc = "EIC external interrupt 2"]
    #[inline(always)]
    pub fn eic_extint2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint2)
    }
    #[doc = "EIC external interrupt 3"]
    #[inline(always)]
    pub fn eic_extint3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint3)
    }
    #[doc = "EIC external interrupt 4"]
    #[inline(always)]
    pub fn eic_extint4(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint4)
    }
    #[doc = "EIC external interrupt 5"]
    #[inline(always)]
    pub fn eic_extint5(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint5)
    }
    #[doc = "EIC external interrupt 6"]
    #[inline(always)]
    pub fn eic_extint6(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint6)
    }
    #[doc = "EIC external interrupt 7"]
    #[inline(always)]
    pub fn eic_extint7(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint7)
    }
    #[doc = "EIC external interrupt 8"]
    #[inline(always)]
    pub fn eic_extint8(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint8)
    }
    #[doc = "EIC external interrupt 9"]
    #[inline(always)]
    pub fn eic_extint9(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint9)
    }
    #[doc = "EIC external interrupt 10"]
    #[inline(always)]
    pub fn eic_extint10(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint10)
    }
    #[doc = "EIC external interrupt 11"]
    #[inline(always)]
    pub fn eic_extint11(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint11)
    }
    #[doc = "EIC external interrupt 12"]
    #[inline(always)]
    pub fn eic_extint12(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint12)
    }
    #[doc = "EIC external interrupt 13"]
    #[inline(always)]
    pub fn eic_extint13(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint13)
    }
    #[doc = "EIC external interrupt 14"]
    #[inline(always)]
    pub fn eic_extint14(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint14)
    }
    #[doc = "EIC external interrupt 15"]
    #[inline(always)]
    pub fn eic_extint15(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::EicExtint15)
    }
    #[doc = "DMA channel 0"]
    #[inline(always)]
    pub fn dmac_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DmacCh0)
    }
    #[doc = "DMA channel 1"]
    #[inline(always)]
    pub fn dmac_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DmacCh1)
    }
    #[doc = "DMA channel 2"]
    #[inline(always)]
    pub fn dmac_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DmacCh2)
    }
    #[doc = "DMA channel 3"]
    #[inline(always)]
    pub fn dmac_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DmacCh3)
    }
    #[doc = "PAC Acc. error"]
    #[inline(always)]
    pub fn pac_accerr(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PacAccerr)
    }
    #[doc = "TCC0 Overflow"]
    #[inline(always)]
    pub fn tcc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Ovf)
    }
    #[doc = "TCC0 Trigger Event"]
    #[inline(always)]
    pub fn tcc0_trg(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Trg)
    }
    #[doc = "TCC0 Counter"]
    #[inline(always)]
    pub fn tcc0_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Cnt)
    }
    #[doc = "TCC0 Match/Compare 0"]
    #[inline(always)]
    pub fn tcc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc0)
    }
    #[doc = "TCC0 Match/Compare 1"]
    #[inline(always)]
    pub fn tcc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc1)
    }
    #[doc = "TCC0 Match/Compare 2"]
    #[inline(always)]
    pub fn tcc0_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc2)
    }
    #[doc = "TCC0 Match/Compare 3"]
    #[inline(always)]
    pub fn tcc0_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc3)
    }
    #[doc = "TCC0 Match/Compare 4"]
    #[inline(always)]
    pub fn tcc0_mc4(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc4)
    }
    #[doc = "TCC0 Match/Compare 5"]
    #[inline(always)]
    pub fn tcc0_mc5(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc0Mc5)
    }
    #[doc = "TCC1 Overflow"]
    #[inline(always)]
    pub fn tcc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Ovf)
    }
    #[doc = "TCC1 Trigger Event"]
    #[inline(always)]
    pub fn tcc1_trg(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Trg)
    }
    #[doc = "TCC1 Counter"]
    #[inline(always)]
    pub fn tcc1_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Cnt)
    }
    #[doc = "TCC1 Match/Compare 0"]
    #[inline(always)]
    pub fn tcc1_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Mc0)
    }
    #[doc = "TCC1 Match/Compare 1"]
    #[inline(always)]
    pub fn tcc1_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Mc1)
    }
    #[doc = "TCC1 Match/Compare 2"]
    #[inline(always)]
    pub fn tcc1_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Mc2)
    }
    #[doc = "TCC1 Match/Compare 3"]
    #[inline(always)]
    pub fn tcc1_mc3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc1Mc3)
    }
    #[doc = "TCC2 Overflow"]
    #[inline(always)]
    pub fn tcc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Ovf)
    }
    #[doc = "TCC2 Trigger Event"]
    #[inline(always)]
    pub fn tcc2_trg(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Trg)
    }
    #[doc = "TCC2 Counter"]
    #[inline(always)]
    pub fn tcc2_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Cnt)
    }
    #[doc = "TCC2 Match/Compare 0"]
    #[inline(always)]
    pub fn tcc2_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Mc0)
    }
    #[doc = "TCC2 Match/Compare 1"]
    #[inline(always)]
    pub fn tcc2_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Mc1)
    }
    #[doc = "TCC2 Match/Compare 2"]
    #[inline(always)]
    pub fn tcc2_mc2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc2Mc2)
    }
    #[doc = "TCC3 Overflow"]
    #[inline(always)]
    pub fn tcc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc3Ovf)
    }
    #[doc = "TCC3 Trigger Event"]
    #[inline(always)]
    pub fn tcc3_trg(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc3Trg)
    }
    #[doc = "TCC3 Counter"]
    #[inline(always)]
    pub fn tcc3_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc3Cnt)
    }
    #[doc = "TCC3 Match/Compare 0"]
    #[inline(always)]
    pub fn tcc3_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc3Mc0)
    }
    #[doc = "TCC3 Match/Compare 1"]
    #[inline(always)]
    pub fn tcc3_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc3Mc1)
    }
    #[doc = "TCC4 Overflow"]
    #[inline(always)]
    pub fn tcc4_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc4Ovf)
    }
    #[doc = "TCC4 Trigger Event"]
    #[inline(always)]
    pub fn tcc4_trg(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc4Trg)
    }
    #[doc = "TCC4 Counter"]
    #[inline(always)]
    pub fn tcc4_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc4Cnt)
    }
    #[doc = "TCC4 Match/Compare 0"]
    #[inline(always)]
    pub fn tcc4_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc4Mc0)
    }
    #[doc = "TCC4 Match/Compare 1"]
    #[inline(always)]
    pub fn tcc4_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tcc4Mc1)
    }
    #[doc = "TC0 Overflow"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc0Ovf)
    }
    #[doc = "TC0 Match/Compare 0"]
    #[inline(always)]
    pub fn tc0_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc0Mc0)
    }
    #[doc = "TC0 Match/Compare 1"]
    #[inline(always)]
    pub fn tc0_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc0Mc1)
    }
    #[doc = "TC1 Overflow"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc1Ovf)
    }
    #[doc = "TC1 Match/Compare 0"]
    #[inline(always)]
    pub fn tc1_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc1Mc0)
    }
    #[doc = "TC1 Match/Compare 1"]
    #[inline(always)]
    pub fn tc1_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc1Mc1)
    }
    #[doc = "TC2 Overflow"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc2Ovf)
    }
    #[doc = "TC2 Match/Compare 0"]
    #[inline(always)]
    pub fn tc2_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc2Mc0)
    }
    #[doc = "TC2 Match/Compare 1"]
    #[inline(always)]
    pub fn tc2_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc2Mc1)
    }
    #[doc = "TC3 Overflow"]
    #[inline(always)]
    pub fn tc3_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc3Ovf)
    }
    #[doc = "TC3 Match/Compare 0"]
    #[inline(always)]
    pub fn tc3_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc3Mc0)
    }
    #[doc = "TC3 Match/Compare 1"]
    #[inline(always)]
    pub fn tc3_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc3Mc1)
    }
    #[doc = "TC4 Overflow"]
    #[inline(always)]
    pub fn tc4_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc4Ovf)
    }
    #[doc = "TC4 Match/Compare 0"]
    #[inline(always)]
    pub fn tc4_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc4Mc0)
    }
    #[doc = "TC4 Match/Compare 1"]
    #[inline(always)]
    pub fn tc4_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc4Mc1)
    }
    #[doc = "TC5 Overflow"]
    #[inline(always)]
    pub fn tc5_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc5Ovf)
    }
    #[doc = "TC5 Match/Compare 0"]
    #[inline(always)]
    pub fn tc5_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc5Mc0)
    }
    #[doc = "TC5 Match/Compare 1"]
    #[inline(always)]
    pub fn tc5_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc5Mc1)
    }
    #[doc = "TC6 Overflow"]
    #[inline(always)]
    pub fn tc6_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc6Ovf)
    }
    #[doc = "TC6 Match/Compare 0"]
    #[inline(always)]
    pub fn tc6_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc6Mc0)
    }
    #[doc = "TC6 Match/Compare 1"]
    #[inline(always)]
    pub fn tc6_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc6Mc1)
    }
    #[doc = "TC7 Overflow"]
    #[inline(always)]
    pub fn tc7_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc7Ovf)
    }
    #[doc = "TC7 Match/Compare 0"]
    #[inline(always)]
    pub fn tc7_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc7Mc0)
    }
    #[doc = "TC7 Match/Compare 1"]
    #[inline(always)]
    pub fn tc7_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Tc7Mc1)
    }
    #[doc = "PDEC Overflow"]
    #[inline(always)]
    pub fn pedc_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcOvf)
    }
    #[doc = "PDEC Error"]
    #[inline(always)]
    pub fn pedc_err(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcErr)
    }
    #[doc = "PDEC Direction"]
    #[inline(always)]
    pub fn pedc_dir(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcDir)
    }
    #[doc = "PDEC VLC"]
    #[inline(always)]
    pub fn pedc_vlc(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcVlc)
    }
    #[doc = "PDEC MC0"]
    #[inline(always)]
    pub fn pedc_mc0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcMc0)
    }
    #[doc = "PDEC MC1"]
    #[inline(always)]
    pub fn pedc_mc1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::PedcMc1)
    }
    #[doc = "ADC0 RESRDY"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Adc0Resrdy)
    }
    #[doc = "ADC0 Window Monitor"]
    #[inline(always)]
    pub fn adc0_winmon(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Adc0Winmon)
    }
    #[doc = "ADC1 RESRDY"]
    #[inline(always)]
    pub fn adc1_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Adc1Resrdy)
    }
    #[doc = "ADC1 Window Monitor"]
    #[inline(always)]
    pub fn adc1_winmon(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::Adc1Winmon)
    }
    #[doc = "AC Comparator 0"]
    #[inline(always)]
    pub fn ac_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::AcComp0)
    }
    #[doc = "AC Comparator 1"]
    #[inline(always)]
    pub fn ac_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::AcComp1)
    }
    #[doc = "AC0 Window"]
    #[inline(always)]
    pub fn ac_win(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::AcWin)
    }
    #[doc = "DAC empty 0"]
    #[inline(always)]
    pub fn dac_empty0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DacEmpty0)
    }
    #[doc = "DAC empty 1"]
    #[inline(always)]
    pub fn dac_empty1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DacEmpty1)
    }
    #[doc = "DAC RESRDY 0"]
    #[inline(always)]
    pub fn dac_resrdy0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DacResrdy0)
    }
    #[doc = "DAC RESRDY 1"]
    #[inline(always)]
    pub fn dac_resrdy1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::DacResrdy1)
    }
    #[doc = "GMAC Timestamp CMP"]
    #[inline(always)]
    pub fn gmac_tsu_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::GmacTsuCmp)
    }
    #[doc = "TRNG Ready"]
    #[inline(always)]
    pub fn trng_ready(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::TrngReady)
    }
    #[doc = "CCL LUTOUT 0"]
    #[inline(always)]
    pub fn ccl_lutout0(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::CclLutout0)
    }
    #[doc = "CCL LUTOUT 1"]
    #[inline(always)]
    pub fn ccl_lutout1(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::CclLutout1)
    }
    #[doc = "CCL LUTOUT 2"]
    #[inline(always)]
    pub fn ccl_lutout2(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::CclLutout2)
    }
    #[doc = "CCL LUTOUT 3"]
    #[inline(always)]
    pub fn ccl_lutout3(self) -> &'a mut crate::W<REG> {
        self.variant(Evgenselect::CclLutout3)
    }
}
#[doc = "Path Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pathselect {
    #[doc = "0: Synchronous path"]
    Synchronous = 0,
    #[doc = "1: Resynchronized path"]
    Resynchronized = 1,
    #[doc = "2: Asynchronous path"]
    Asynchronous = 2,
}
impl From<Pathselect> for u8 {
    #[inline(always)]
    fn from(variant: Pathselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pathselect {
    type Ux = u8;
}
impl crate::IsEnum for Pathselect {}
#[doc = "Field `PATH` reader - Path Selection"]
pub type PathR = crate::FieldReader<Pathselect>;
impl PathR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pathselect> {
        match self.bits {
            0 => Some(Pathselect::Synchronous),
            1 => Some(Pathselect::Resynchronized),
            2 => Some(Pathselect::Asynchronous),
            _ => None,
        }
    }
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Pathselect::Synchronous
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == Pathselect::Resynchronized
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Pathselect::Asynchronous
    }
}
#[doc = "Field `PATH` writer - Path Selection"]
pub type PathW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pathselect>;
impl<'a, REG> PathW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Synchronous)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Resynchronized)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Pathselect::Asynchronous)
    }
}
#[doc = "Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgselselect {
    #[doc = "0: No event output when using the resynchronized or synchronous path"]
    NoEvtOutput = 0,
    #[doc = "1: Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RisingEdge = 1,
    #[doc = "2: Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FallingEdge = 2,
    #[doc = "3: Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BothEdges = 3,
}
impl From<Edgselselect> for u8 {
    #[inline(always)]
    fn from(variant: Edgselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgselselect {
    type Ux = u8;
}
impl crate::IsEnum for Edgselselect {}
#[doc = "Field `EDGSEL` reader - Edge Detection Selection"]
pub type EdgselR = crate::FieldReader<Edgselselect>;
impl EdgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edgselselect {
        match self.bits {
            0 => Edgselselect::NoEvtOutput,
            1 => Edgselselect::RisingEdge,
            2 => Edgselselect::FallingEdge,
            3 => Edgselselect::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == Edgselselect::NoEvtOutput
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Edgselselect::RisingEdge
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Edgselselect::FallingEdge
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == Edgselselect::BothEdges
    }
}
#[doc = "Field `EDGSEL` writer - Edge Detection Selection"]
pub type EdgselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgselselect, crate::Safe>;
impl<'a, REG> EdgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::NoEvtOutput)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::RisingEdge)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::FallingEdge)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Edgselselect::BothEdges)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - Generic Clock On Demand"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Generic Clock On Demand"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EvgenR {
        EvgenR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PathR {
        PathR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EdgselR {
        EdgselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Event Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn evgen(&mut self) -> EvgenW<ChannelSpec> {
        EvgenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    #[must_use]
    pub fn path(&mut self) -> PathW<ChannelSpec> {
        PathW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgsel(&mut self) -> EdgselW<ChannelSpec> {
        EdgselW::new(self, 10)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<ChannelSpec> {
        RunstdbyW::new(self, 14)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<ChannelSpec> {
        OndemandW::new(self, 15)
    }
}
#[doc = "Channel n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelSpec;
impl crate::RegisterSpec for ChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for ChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for ChannelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHANNEL to value 0x8000"]
impl crate::Resettable for ChannelSpec {
    const RESET_VALUE: u32 = 0x8000;
}
