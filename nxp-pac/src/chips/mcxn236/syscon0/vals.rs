#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Adc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc0Rst {
        Adc0Rst::from_bits(val)
    }
}
impl From<Adc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc0Rst) -> u8 {
        Adc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Adc0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivHalt {
        Adc0clkdivHalt::from_bits(val)
    }
}
impl From<Adc0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivHalt) -> u8 {
        Adc0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Adc0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivReset {
        Adc0clkdivReset::from_bits(val)
    }
}
impl From<Adc0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivReset) -> u8 {
        Adc0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Adc0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivUnstab {
        Adc0clkdivUnstab::from_bits(val)
    }
}
impl From<Adc0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivUnstab) -> u8 {
        Adc0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO 12 MHz clock."]
    Enum0x3 = 0x03,
    #[doc = "Clk_in."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Adc0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkselSel {
        Adc0clkselSel::from_bits(val)
    }
}
impl From<Adc0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkselSel) -> u8 {
        Adc0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Adc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc1Rst {
        Adc1Rst::from_bits(val)
    }
}
impl From<Adc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc1Rst) -> u8 {
        Adc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Adc1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivHalt {
        Adc1clkdivHalt::from_bits(val)
    }
}
impl From<Adc1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivHalt) -> u8 {
        Adc1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Adc1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivReset {
        Adc1clkdivReset::from_bits(val)
    }
}
impl From<Adc1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivReset) -> u8 {
        Adc1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Adc1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivUnstab {
        Adc1clkdivUnstab::from_bits(val)
    }
}
impl From<Adc1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivUnstab) -> u8 {
        Adc1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO 12 MHz clock."]
    Enum0x3 = 0x03,
    #[doc = "Clk_in clock."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Adc1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkselSel {
        Adc1clkselSel::from_bits(val)
    }
}
impl From<Adc1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkselSel) -> u8 {
        Adc1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Aoi0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0Rst {
    #[inline(always)]
    fn from(val: u8) -> Aoi0Rst {
        Aoi0Rst::from_bits(val)
    }
}
impl From<Aoi0Rst> for u8 {
    #[inline(always)]
    fn from(val: Aoi0Rst) -> u8 {
        Aoi0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AssetProtection {
    #[doc = "ELS asset is protected."]
    Value0 = 0x0,
    #[doc = "ELS asset is not protected."]
    Value1 = 0x01,
    #[doc = "ELS asset is protected."]
    Value2 = 0x02,
    #[doc = "ELS asset is protected."]
    Value3 = 0x03,
}
impl AssetProtection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AssetProtection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AssetProtection {
    #[inline(always)]
    fn from(val: u8) -> AssetProtection {
        AssetProtection::from_bits(val)
    }
}
impl From<AssetProtection> for u8 {
    #[inline(always)]
    fn from(val: AssetProtection) -> u8 {
        AssetProtection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootImage {
    #[doc = "Internal flash image 0."]
    Enum0 = 0x0,
    #[doc = "Internal flash image 1."]
    Enum1 = 0x01,
    #[doc = "FlexSPI flash image 0."]
    Enum2 = 0x02,
    #[doc = "FlexSPI flash image 1."]
    Enum3 = 0x03,
    #[doc = "Recovery SPI flash image."]
    Enum4 = 0x04,
    #[doc = "Serial boot image (write-memory and execute ISP command used)."]
    Enum5 = 0x05,
    #[doc = "Receive SB3 containing SB_JUMP command is used."]
    Enum6 = 0x06,
    #[doc = "Customer SBL/recovery image (Bank1 IFR0)."]
    Enum7 = 0x07,
    #[doc = "NXP MAD recovery image (Bank1 IFR0)."]
    Enum8 = 0x08,
    #[doc = "NXP ROM extension (NMPA - Bank0 IFR0)."]
    Enum9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl BootImage {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootImage {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootImage {
    #[inline(always)]
    fn from(val: u8) -> BootImage {
        BootImage::from_bits(val)
    }
}
impl From<BootImage> for u8 {
    #[inline(always)]
    fn from(val: BootImage) -> u8 {
        BootImage::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl ClkoutdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivHalt {
        ClkoutdivHalt::from_bits(val)
    }
}
impl From<ClkoutdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivHalt) -> u8 {
        ClkoutdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl ClkoutdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReset {
        ClkoutdivReset::from_bits(val)
    }
}
impl From<ClkoutdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReset) -> u8 {
        ClkoutdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl ClkoutdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivUnstab {
        ClkoutdivUnstab::from_bits(val)
    }
}
impl From<ClkoutdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivUnstab) -> u8 {
        ClkoutdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock (main_clk)."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock (pll0_clk)."]
    Enum0x1 = 0x01,
    #[doc = "CLKIN clock (clk_in)."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock (fro_hf)."]
    Enum0x3 = 0x03,
    #[doc = "FRO 12 MHz clock (fro_12m)."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock (pll1_clk)."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock (lp_osc)."]
    Enum0x6 = 0x06,
    #[doc = "USB PLL clock (usb_pll_clk)."]
    Enum0x7 = 0x07,
    #[doc = "No clock."]
    Enum0x8 = 0x08,
    #[doc = "No clock."]
    Enum0x9 = 0x09,
    #[doc = "No clock."]
    Enum0xA = 0x0a,
    #[doc = "No clock."]
    Enum0xB = 0x0b,
    #[doc = "No clock."]
    Enum0xC = 0x0c,
    #[doc = "No clock."]
    Enum0xD = 0x0d,
    #[doc = "No clock."]
    Enum0xE = 0x0e,
    #[doc = "No clock."]
    Enum0xF = 0x0f,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrFlashCache {
    #[doc = "No clear flash cache."]
    Enable = 0x0,
    #[doc = "Clears flash cache."]
    Disable = 0x01,
}
impl ClrFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrFlashCache {
    #[inline(always)]
    fn from(val: u8) -> ClrFlashCache {
        ClrFlashCache::from_bits(val)
    }
}
impl From<ClrFlashCache> for u8 {
    #[inline(always)]
    fn from(val: ClrFlashCache) -> u8 {
        ClrFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache."]
    Enable = 0x0,
    #[doc = "Clears the cache."]
    Disable = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Cmp0fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivHalt {
        Cmp0fclkdivHalt::from_bits(val)
    }
}
impl From<Cmp0fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivHalt) -> u8 {
        Cmp0fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Cmp0fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivReset {
        Cmp0fclkdivReset::from_bits(val)
    }
}
impl From<Cmp0fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivReset) -> u8 {
        Cmp0fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Cmp0fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkdivUnstab {
        Cmp0fclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp0fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkdivUnstab) -> u8 {
        Cmp0fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkselSel {
        Cmp0fclkselSel::from_bits(val)
    }
}
impl From<Cmp0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkselSel) -> u8 {
        Cmp0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Cmp0rrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivHalt {
        Cmp0rrclkdivHalt::from_bits(val)
    }
}
impl From<Cmp0rrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivHalt) -> u8 {
        Cmp0rrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Cmp0rrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivReset {
        Cmp0rrclkdivReset::from_bits(val)
    }
}
impl From<Cmp0rrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivReset) -> u8 {
        Cmp0rrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Cmp0rrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkdivUnstab {
        Cmp0rrclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp0rrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkdivUnstab) -> u8 {
        Cmp0rrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp0rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkselSel {
        Cmp0rrclkselSel::from_bits(val)
    }
}
impl From<Cmp0rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkselSel) -> u8 {
        Cmp0rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Cmp1fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivHalt {
        Cmp1fclkdivHalt::from_bits(val)
    }
}
impl From<Cmp1fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivHalt) -> u8 {
        Cmp1fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Cmp1fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivReset {
        Cmp1fclkdivReset::from_bits(val)
    }
}
impl From<Cmp1fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivReset) -> u8 {
        Cmp1fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Cmp1fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkdivUnstab {
        Cmp1fclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp1fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkdivUnstab) -> u8 {
        Cmp1fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkselSel {
        Cmp1fclkselSel::from_bits(val)
    }
}
impl From<Cmp1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkselSel) -> u8 {
        Cmp1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Cmp1rrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivHalt {
        Cmp1rrclkdivHalt::from_bits(val)
    }
}
impl From<Cmp1rrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivHalt) -> u8 {
        Cmp1rrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Cmp1rrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivReset {
        Cmp1rrclkdivReset::from_bits(val)
    }
}
impl From<Cmp1rrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivReset) -> u8 {
        Cmp1rrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Cmp1rrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkdivUnstab {
        Cmp1rrclkdivUnstab::from_bits(val)
    }
}
impl From<Cmp1rrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkdivUnstab) -> u8 {
        Cmp1rrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp1rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkselSel {
        Cmp1rrclkselSel::from_bits(val)
    }
}
impl From<Cmp1rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkselSel) -> u8 {
        Cmp1rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup."]
    Awake = 0x0,
    #[doc = "CPU is in lockup."]
    Sleeping = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalNoref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Cpu0nstckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalNoref {
        Cpu0nstckcalNoref::from_bits(val)
    }
}
impl From<Cpu0nstckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalNoref) -> u8 {
        Cpu0nstckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalSkew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Cpu0nstckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalSkew {
        Cpu0nstckcalSkew::from_bits(val)
    }
}
impl From<Cpu0nstckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalSkew) -> u8 {
        Cpu0nstckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping."]
    Awake = 0x0,
    #[doc = "CPU is sleeping."]
    Sleeping = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalNoref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Cpu0stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalNoref {
        Cpu0stckcalNoref::from_bits(val)
    }
}
impl From<Cpu0stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalNoref) -> u8 {
        Cpu0stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalSkew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Cpu0stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalSkew {
        Cpu0stckcalSkew::from_bits(val)
    }
}
impl From<Cpu0stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalSkew) -> u8 {
        Cpu0stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl CrcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRst {
    #[inline(always)]
    fn from(val: u8) -> CrcRst {
        CrcRst::from_bits(val)
    }
}
impl From<CrcRst> for u8 {
    #[inline(always)]
    fn from(val: CrcRst) -> u8 {
        CrcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivHalt {
    #[doc = "Divider clock is running."]
    Enable = 0x0,
    #[doc = "Divider clock has stopped."]
    Disable = 0x01,
}
impl CtimerclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivHalt {
        CtimerclkdivHalt::from_bits(val)
    }
}
impl From<CtimerclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivHalt) -> u8 {
        CtimerclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivUnstab {
    #[doc = "Stable divider clock."]
    Enable = 0x0,
    #[doc = "Unstable clock frequency."]
    Disable = 0x01,
}
impl CtimerclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivUnstab {
        CtimerclkdivUnstab::from_bits(val)
    }
}
impl From<CtimerclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivUnstab) -> u8 {
        CtimerclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkselSel {
    #[doc = "FRO_1M clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "PLL1_clk0 clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum0x3 = 0x03,
    #[doc = "FRO 12MHz clock."]
    Enum0x4 = 0x04,
    #[doc = "SAI0 MCLK IN clock."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
    #[doc = "SAI1 MCLK IN clock."]
    Enum0x8 = 0x08,
    #[doc = "SAI0 TX_BCLK clock."]
    Enum0x9 = 0x09,
    #[doc = "SAI0 RX_BCLK clock."]
    Enum0xA = 0x0a,
    #[doc = "SAI1 TX_BCLK clock."]
    Enum0xB = 0x0b,
    #[doc = "SAI1 RX_BCLK clock."]
    Enum0xC = 0x0c,
    #[doc = "No clock."]
    Enum0xD = 0x0d,
    #[doc = "No clock."]
    Enum0xE = 0x0e,
    #[doc = "No clock."]
    Enum0xF = 0x0f,
}
impl CtimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkselSel {
        CtimerclkselSel::from_bits(val)
    }
}
impl From<CtimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkselSel) -> u8 {
        CtimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation."]
    Enable = 0x0,
    #[doc = "Disables data speculation."]
    Disable = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashCache {
    #[doc = "Enables flash cache."]
    Enable = 0x0,
    #[doc = "Disables flash cache."]
    Disable = 0x01,
}
impl DisFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashCache {
    #[inline(always)]
    fn from(val: u8) -> DisFlashCache {
        DisFlashCache::from_bits(val)
    }
}
impl From<DisFlashCache> for u8 {
    #[inline(always)]
    fn from(val: DisFlashCache) -> u8 {
        DisFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashData {
    #[doc = "Enables flash data cache when DIS_FLASH_CACHE=0."]
    Enable = 0x0,
    #[doc = "Disables flash data cache."]
    Disable = 0x01,
}
impl DisFlashData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashData {
    #[inline(always)]
    fn from(val: u8) -> DisFlashData {
        DisFlashData::from_bits(val)
    }
}
impl From<DisFlashData> for u8 {
    #[inline(always)]
    fn from(val: DisFlashData) -> u8 {
        DisFlashData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashInst {
    #[doc = "Enables flash instruction cache when DIS_FLASH_CACHE=0."]
    Enable = 0x0,
    #[doc = "Disables flash instruction cache."]
    Disable = 0x01,
}
impl DisFlashInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashInst {
    #[inline(always)]
    fn from(val: u8) -> DisFlashInst {
        DisFlashInst::from_bits(val)
    }
}
impl From<DisFlashInst> for u8 {
    #[inline(always)]
    fn from(val: DisFlashInst) -> u8 {
        DisFlashInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation."]
    Enable = 0x0,
    #[doc = "Disables flash speculation."]
    Disable = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data."]
    Disable = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction."]
    Disable = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dma0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rst {
        Dma0Rst::from_bits(val)
    }
}
impl From<Dma0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rst) -> u8 {
        Dma0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dma1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rst {
        Dma1Rst::from_bits(val)
    }
}
impl From<Dma1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rst) -> u8 {
        Dma1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EimRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl EimRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EimRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EimRst {
    #[inline(always)]
    fn from(val: u8) -> EimRst {
        EimRst::from_bits(val)
    }
}
impl From<EimRst> for u8 {
    #[inline(always)]
    fn from(val: EimRst) -> u8 {
        EimRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0clkselSel {
    #[doc = "clk_16k\\[2\\]."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
}
impl Ewm0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0clkselSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Ewm0clkselSel {
        Ewm0clkselSel::from_bits(val)
    }
}
impl From<Ewm0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Ewm0clkselSel) -> u8 {
        Ewm0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EwmRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl EwmRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EwmRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EwmRst {
    #[inline(always)]
    fn from(val: u8) -> EwmRst {
        EwmRst::from_bits(val)
    }
}
impl From<EwmRst> for u8 {
    #[inline(always)]
    fn from(val: EwmRst) -> u8 {
        EwmRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc0Rst {
        Fc0Rst::from_bits(val)
    }
}
impl From<Fc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc0Rst) -> u8 {
        Fc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc1Rst {
        Fc1Rst::from_bits(val)
    }
}
impl From<Fc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc1Rst) -> u8 {
        Fc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc2Rst {
        Fc2Rst::from_bits(val)
    }
}
impl From<Fc2Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc2Rst) -> u8 {
        Fc2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc3Rst {
        Fc3Rst::from_bits(val)
    }
}
impl From<Fc3Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc3Rst) -> u8 {
        Fc3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc4Rst {
        Fc4Rst::from_bits(val)
    }
}
impl From<Fc4Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc4Rst) -> u8 {
        Fc4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc5Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc5Rst {
        Fc5Rst::from_bits(val)
    }
}
impl From<Fc5Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc5Rst) -> u8 {
        Fc5Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc6Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc6Rst {
        Fc6Rst::from_bits(val)
    }
}
impl From<Fc6Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc6Rst) -> u8 {
        Fc6Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Fc7Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7Rst {
    #[inline(always)]
    fn from(val: u8) -> Fc7Rst {
        Fc7Rst::from_bits(val)
    }
}
impl From<Fc7Rst> for u8 {
    #[inline(always)]
    fn from(val: Fc7Rst) -> u8 {
        Fc7Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL divided clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    Enum0x2 = 0x02,
    #[doc = "fro_hf_div clock."]
    Enum0x3 = 0x03,
    #[doc = "clk_1m clock."]
    Enum0x4 = 0x04,
    #[doc = "USB PLL clock."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy."]
    Enable = 0x0,
    #[doc = "Stall on FLASH busy."]
    Disable = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Flexcan0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0Rst {
        Flexcan0Rst::from_bits(val)
    }
}
impl From<Flexcan0Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0Rst) -> u8 {
        Flexcan0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Flexcan0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivHalt {
        Flexcan0clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivHalt) -> u8 {
        Flexcan0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Flexcan0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivReset {
        Flexcan0clkdivReset::from_bits(val)
    }
}
impl From<Flexcan0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivReset) -> u8 {
        Flexcan0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Flexcan0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivUnstab {
        Flexcan0clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivUnstab) -> u8 {
        Flexcan0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Flexcan0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkselSel {
        Flexcan0clkselSel::from_bits(val)
    }
}
impl From<Flexcan0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkselSel) -> u8 {
        Flexcan0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Flexcan1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1Rst {
        Flexcan1Rst::from_bits(val)
    }
}
impl From<Flexcan1Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1Rst) -> u8 {
        Flexcan1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Flexcan1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivHalt {
        Flexcan1clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivHalt) -> u8 {
        Flexcan1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Flexcan1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivReset {
        Flexcan1clkdivReset::from_bits(val)
    }
}
impl From<Flexcan1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivReset) -> u8 {
        Flexcan1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Flexcan1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivUnstab {
        Flexcan1clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivUnstab) -> u8 {
        Flexcan1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Flexcan1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkselSel {
        Flexcan1clkselSel::from_bits(val)
    }
}
impl From<Flexcan1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkselSel) -> u8 {
        Flexcan1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FlexcommclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivHalt {
        FlexcommclkdivHalt::from_bits(val)
    }
}
impl From<FlexcommclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivHalt) -> u8 {
        FlexcommclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FlexcommclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivReset {
        FlexcommclkdivReset::from_bits(val)
    }
}
impl From<FlexcommclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivReset) -> u8 {
        FlexcommclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FlexcommclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivUnstab {
        FlexcommclkdivUnstab::from_bits(val)
    }
}
impl From<FlexcommclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivUnstab) -> u8 {
        FlexcommclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FlexioRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioRst {
    #[inline(always)]
    fn from(val: u8) -> FlexioRst {
        FlexioRst::from_bits(val)
    }
}
impl From<FlexioRst> for u8 {
    #[inline(always)]
    fn from(val: FlexioRst) -> u8 {
        FlexioRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FlexioclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivHalt {
        FlexioclkdivHalt::from_bits(val)
    }
}
impl From<FlexioclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivHalt) -> u8 {
        FlexioclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FlexioclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivReset {
        FlexioclkdivReset::from_bits(val)
    }
}
impl From<FlexioclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivReset) -> u8 {
        FlexioclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FlexioclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivUnstab {
        FlexioclkdivUnstab::from_bits(val)
    }
}
impl From<FlexioclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivUnstab) -> u8 {
        FlexioclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "FRO_12M clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl FlexioclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkselSel {
        FlexioclkselSel::from_bits(val)
    }
}
impl From<FlexioclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkselSel) -> u8 {
        FlexioclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FmuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuRst {
    #[inline(always)]
    fn from(val: u8) -> FmuRst {
        FmuRst::from_bits(val)
    }
}
impl From<FmuRst> for u8 {
    #[inline(always)]
    fn from(val: FmuRst) -> u8 {
        FmuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation."]
    Enable = 0x0,
    #[doc = "Forces no allocation."]
    Disable = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running, this bit is set to 0 when the register is written."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetIsoSw {
    #[doc = "Isolation is disabled."]
    Disable0 = 0x0,
    #[doc = "Isolation is disabled."]
    Disable1 = 0x01,
    #[doc = "Isolation is enabled. When both GDET0_CTRL/GDET1_CTRL GDET_ISO_SW are \"10\", isolation_on is asserted."]
    Enable = 0x02,
    #[doc = "Isolation is disabled."]
    Disable3 = 0x03,
}
impl GdetIsoSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetIsoSw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetIsoSw {
    #[inline(always)]
    fn from(val: u8) -> GdetIsoSw {
        GdetIsoSw::from_bits(val)
    }
}
impl From<GdetIsoSw> for u8 {
    #[inline(always)]
    fn from(val: GdetIsoSw) -> u8 {
        GdetIsoSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Gpio0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Rst {
        Gpio0Rst::from_bits(val)
    }
}
impl From<Gpio0Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Rst) -> u8 {
        Gpio0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Gpio1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Rst {
        Gpio1Rst::from_bits(val)
    }
}
impl From<Gpio1Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Rst) -> u8 {
        Gpio1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Gpio2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Rst {
        Gpio2Rst::from_bits(val)
    }
}
impl From<Gpio2Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Rst) -> u8 {
        Gpio2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Gpio3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Rst {
        Gpio3Rst::from_bits(val)
    }
}
impl From<Gpio3Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Rst) -> u8 {
        Gpio3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Gpio4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Rst {
        Gpio4Rst::from_bits(val)
    }
}
impl From<Gpio4Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Rst) -> u8 {
        Gpio4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl I3c0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c0Rst {
        I3c0Rst::from_bits(val)
    }
}
impl From<I3c0Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c0Rst) -> u8 {
        I3c0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c0fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivHalt {
        I3c0fclkdivHalt::from_bits(val)
    }
}
impl From<I3c0fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivHalt) -> u8 {
        I3c0fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c0fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivReset {
        I3c0fclkdivReset::from_bits(val)
    }
}
impl From<I3c0fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivReset) -> u8 {
        I3c0fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c0fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivUnstab {
        I3c0fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivUnstab) -> u8 {
        I3c0fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "clk_1m clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkselSel {
        I3c0fclkselSel::from_bits(val)
    }
}
impl From<I3c0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkselSel) -> u8 {
        I3c0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl I3c1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c1Rst {
        I3c1Rst::from_bits(val)
    }
}
impl From<I3c1Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c1Rst) -> u8 {
        I3c1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c1fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivHalt {
        I3c1fclkdivHalt::from_bits(val)
    }
}
impl From<I3c1fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivHalt) -> u8 {
        I3c1fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c1fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivReset {
        I3c1fclkdivReset::from_bits(val)
    }
}
impl From<I3c1fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivReset) -> u8 {
        I3c1fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c1fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivUnstab {
        I3c1fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivUnstab) -> u8 {
        I3c1fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "clk_1m clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkselSel {
        I3c1fclkselSel::from_bits(val)
    }
}
impl From<I3c1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkselSel) -> u8 {
        I3c1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is consecutive."]
    Normal = 0x0,
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is interleaved. This setting is need for PKC L0 memory access."]
    Interleave = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers."]
    Disable = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers."]
    Enable = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MicfilRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilRst {
    #[inline(always)]
    fn from(val: u8) -> MicfilRst {
        MicfilRst::from_bits(val)
    }
}
impl From<MicfilRst> for u8 {
    #[inline(always)]
    fn from(val: MicfilRst) -> u8 {
        MicfilRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl MicfilfclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivHalt {
        MicfilfclkdivHalt::from_bits(val)
    }
}
impl From<MicfilfclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivHalt) -> u8 {
        MicfilfclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl MicfilfclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivReset {
        MicfilfclkdivReset::from_bits(val)
    }
}
impl From<MicfilfclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivReset) -> u8 {
        MicfilfclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl MicfilfclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivUnstab {
        MicfilfclkdivUnstab::from_bits(val)
    }
}
impl From<MicfilfclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivUnstab) -> u8 {
        MicfilfclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkselSel {
    #[doc = "FRO_12M clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_clk0 clock."]
    Enum4 = 0x04,
    #[doc = "SAI0_MCLK clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
    #[doc = "SAI1_MCLK clock."]
    Enum8 = 0x08,
    #[doc = "No clock."]
    Enum9 = 0x09,
    #[doc = "No clock."]
    Enum10 = 0x0a,
    #[doc = "No clock."]
    Enum11 = 0x0b,
    #[doc = "No clock."]
    Enum12 = 0x0c,
    #[doc = "No clock."]
    Enum13 = 0x0d,
    #[doc = "No clock."]
    Enum14 = 0x0e,
    #[doc = "No clock."]
    Enum15 = 0x0f,
}
impl MicfilfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkselSel {
        MicfilfclkselSel::from_bits(val)
    }
}
impl From<MicfilfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkselSel) -> u8 {
        MicfilfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MrtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRst {
    #[inline(always)]
    fn from(val: u8) -> MrtRst {
        MrtRst::from_bits(val)
    }
}
impl From<MrtRst> for u8 {
    #[inline(always)]
    fn from(val: MrtRst) -> u8 {
        MrtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MuxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRst {
    #[inline(always)]
    fn from(val: u8) -> MuxRst {
        MuxRst::from_bits(val)
    }
}
impl From<MuxRst> for u8 {
    #[inline(always)]
    fn from(val: MuxRst) -> u8 {
        MuxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl OstimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerRst {
    #[inline(always)]
    fn from(val: u8) -> OstimerRst {
        OstimerRst::from_bits(val)
    }
}
impl From<OstimerRst> for u8 {
    #[inline(always)]
    fn from(val: OstimerRst) -> u8 {
        OstimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerclkselSel {
    #[doc = "clk_16k\\[2\\]."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
}
impl OstimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> OstimerclkselSel {
        OstimerclkselSel::from_bits(val)
    }
}
impl From<OstimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: OstimerclkselSel) -> u8 {
        OstimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRst {
    #[inline(always)]
    fn from(val: u8) -> PintRst {
        PintRst::from_bits(val)
    }
}
impl From<PintRst> for u8 {
    #[inline(always)]
    fn from(val: PintRst) -> u8 {
        PintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PkcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRst {
    #[inline(always)]
    fn from(val: u8) -> PkcRst {
        PkcRst::from_bits(val)
    }
}
impl From<PkcRst> for u8 {
    #[inline(always)]
    fn from(val: PkcRst) -> u8 {
        PkcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Pll1clk0divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divHalt {
        Pll1clk0divHalt::from_bits(val)
    }
}
impl From<Pll1clk0divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divHalt) -> u8 {
        Pll1clk0divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Pll1clk0divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divReset {
        Pll1clk0divReset::from_bits(val)
    }
}
impl From<Pll1clk0divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divReset) -> u8 {
        Pll1clk0divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Pll1clk0divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divUnstab {
        Pll1clk0divUnstab::from_bits(val)
    }
}
impl From<Pll1clk0divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divUnstab) -> u8 {
        Pll1clk0divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Pll1clk1divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divHalt {
        Pll1clk1divHalt::from_bits(val)
    }
}
impl From<Pll1clk1divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divHalt) -> u8 {
        Pll1clk1divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Pll1clk1divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divReset {
        Pll1clk1divReset::from_bits(val)
    }
}
impl From<Pll1clk1divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divReset) -> u8 {
        Pll1clk1divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Pll1clk1divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divUnstab {
        Pll1clk1divUnstab::from_bits(val)
    }
}
impl From<Pll1clk1divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divUnstab) -> u8 {
        Pll1clk1divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl PllclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivHalt {
        PllclkdivHalt::from_bits(val)
    }
}
impl From<PllclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivHalt) -> u8 {
        PllclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl PllclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivReset {
        PllclkdivReset::from_bits(val)
    }
}
impl From<PllclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivReset) -> u8 {
        PllclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl PllclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivUnstab {
        PllclkdivUnstab::from_bits(val)
    }
}
impl From<PllclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivUnstab) -> u8 {
        PllclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivselSel {
    #[doc = "PLL0 clock."]
    Enum0 = 0x0,
    #[doc = "pll1_clk0."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl PllclkdivselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivselSel {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivselSel {
        PllclkdivselSel::from_bits(val)
    }
}
impl From<PllclkdivselSel> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivselSel) -> u8 {
        PllclkdivselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Port0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0Rst {
    #[inline(always)]
    fn from(val: u8) -> Port0Rst {
        Port0Rst::from_bits(val)
    }
}
impl From<Port0Rst> for u8 {
    #[inline(always)]
    fn from(val: Port0Rst) -> u8 {
        Port0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Port1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1Rst {
    #[inline(always)]
    fn from(val: u8) -> Port1Rst {
        Port1Rst::from_bits(val)
    }
}
impl From<Port1Rst> for u8 {
    #[inline(always)]
    fn from(val: Port1Rst) -> u8 {
        Port1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Port2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2Rst {
    #[inline(always)]
    fn from(val: u8) -> Port2Rst {
        Port2Rst::from_bits(val)
    }
}
impl From<Port2Rst> for u8 {
    #[inline(always)]
    fn from(val: Port2Rst) -> u8 {
        Port2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Port3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3Rst {
    #[inline(always)]
    fn from(val: u8) -> Port3Rst {
        Port3Rst::from_bits(val)
    }
}
impl From<Port3Rst> for u8 {
    #[inline(always)]
    fn from(val: Port3Rst) -> u8 {
        Port3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Port4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4Rst {
    #[inline(always)]
    fn from(val: u8) -> Port4Rst {
        Port4Rst::from_bits(val)
    }
}
impl From<Port4Rst> for u8 {
    #[inline(always)]
    fn from(val: Port4Rst) -> u8 {
        Port4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Cbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Cbus {
        PriCpu0Cbus::from_bits(val)
    }
}
impl From<PriCpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Cbus) -> u8 {
        PriCpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Sbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Sbus {
        PriCpu0Sbus::from_bits(val)
    }
}
impl From<PriCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Sbus) -> u8 {
        PriCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriPkcEls {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriPkcEls {
    #[inline(always)]
    fn from(val: u8) -> PriPkcEls {
        PriPkcEls::from_bits(val)
    }
}
impl From<PriPkcEls> for u8 {
    #[inline(always)]
    fn from(val: PriPkcEls) -> u8 {
        PriPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbHs {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbHs {
    #[inline(always)]
    fn from(val: u8) -> PriUsbHs {
        PriUsbHs::from_bits(val)
    }
}
impl From<PriUsbHs> for u8 {
    #[inline(always)]
    fn from(val: PriUsbHs) -> u8 {
        PriUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PufRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRst {
    #[inline(always)]
    fn from(val: u8) -> PufRst {
        PufRst::from_bits(val)
    }
}
impl From<PufRst> for u8 {
    #[inline(always)]
    fn from(val: PufRst) -> u8 {
        PufRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Pwm0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm0Rst {
        Pwm0Rst::from_bits(val)
    }
}
impl From<Pwm0Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm0Rst) -> u8 {
        Pwm0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Pwm1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm1Rst {
        Pwm1Rst::from_bits(val)
    }
}
impl From<Pwm1Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm1Rst) -> u8 {
        Pwm1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Qdc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Rst {
        Qdc0Rst::from_bits(val)
    }
}
impl From<Qdc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Rst) -> u8 {
        Qdc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Qdc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Rst {
        Qdc1Rst::from_bits(val)
    }
}
impl From<Qdc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Rst) -> u8 {
        Qdc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl RtcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRst {
    #[inline(always)]
    fn from(val: u8) -> RtcRst {
        RtcRst::from_bits(val)
    }
}
impl From<RtcRst> for u8 {
    #[inline(always)]
    fn from(val: RtcRst) -> u8 {
        RtcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sai0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai0Rst {
        Sai0Rst::from_bits(val)
    }
}
impl From<Sai0Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai0Rst) -> u8 {
        Sai0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Sai0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivHalt {
        Sai0clkdivHalt::from_bits(val)
    }
}
impl From<Sai0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivHalt) -> u8 {
        Sai0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Sai0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivReset {
        Sai0clkdivReset::from_bits(val)
    }
}
impl From<Sai0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivReset) -> u8 {
        Sai0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Sai0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivUnstab {
        Sai0clkdivUnstab::from_bits(val)
    }
}
impl From<Sai0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivUnstab) -> u8 {
        Sai0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_CLK0 clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Sai0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkselSel {
        Sai0clkselSel::from_bits(val)
    }
}
impl From<Sai0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkselSel) -> u8 {
        Sai0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sai1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai1Rst {
        Sai1Rst::from_bits(val)
    }
}
impl From<Sai1Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai1Rst) -> u8 {
        Sai1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Sai1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivHalt {
        Sai1clkdivHalt::from_bits(val)
    }
}
impl From<Sai1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivHalt) -> u8 {
        Sai1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Sai1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivReset {
        Sai1clkdivReset::from_bits(val)
    }
}
impl From<Sai1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivReset) -> u8 {
        Sai1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Sai1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivUnstab {
        Sai1clkdivUnstab::from_bits(val)
    }
}
impl From<Sai1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivUnstab) -> u8 {
        Sai1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_CLK0 clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Sai1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkselSel {
        Sai1clkselSel::from_bits(val)
    }
}
impl From<Sai1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkselSel) -> u8 {
        Sai1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sb3 {
    #[doc = "customer fw load/update file."]
    Customer = 0x0,
    #[doc = "NXP Provisioning FW."]
    Nxp = 0x01,
    #[doc = "ELS signed OEM Provisioning FW."]
    Oem = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sb3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sb3 {
    #[inline(always)]
    fn from(val: u8) -> Sb3 {
        Sb3::from_bits(val)
    }
}
impl From<Sb3> for u8 {
    #[inline(always)]
    fn from(val: Sb3) -> u8 {
        Sb3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecCode(u32);
impl SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SecCode {
    pub const fn from_bits(val: u32) -> SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SecCode {
    #[inline(always)]
    fn from(val: u32) -> SecCode {
        SecCode::from_bits(val)
    }
}
impl From<SecCode> for u32 {
    #[inline(always)]
    fn from(val: SecCode) -> u32 {
        SecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl SmartDmaRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaRst {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaRst {
        SmartDmaRst::from_bits(val)
    }
}
impl From<SmartDmaRst> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaRst) -> u8 {
        SmartDmaRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Halt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Systickclkdiv0Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Halt {
        Systickclkdiv0Halt::from_bits(val)
    }
}
impl From<Systickclkdiv0Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Halt) -> u8 {
        Systickclkdiv0Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Systickclkdiv0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reset {
        Systickclkdiv0Reset::from_bits(val)
    }
}
impl From<Systickclkdiv0Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reset) -> u8 {
        Systickclkdiv0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Unstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Systickclkdiv0Unstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Unstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Unstab {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Unstab {
        Systickclkdiv0Unstab::from_bits(val)
    }
}
impl From<Systickclkdiv0Unstab> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Unstab) -> u8 {
        Systickclkdiv0Unstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "SYSTICKCLKDIV0 output."]
    Enum0x0 = 0x0,
    #[doc = "Clk 1 MHz clock."]
    Enum0x1 = 0x01,
    #[doc = "LP Oscillator clock."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "No clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer0Rst {
        Timer0Rst::from_bits(val)
    }
}
impl From<Timer0Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer0Rst) -> u8 {
        Timer0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer1Rst {
        Timer1Rst::from_bits(val)
    }
}
impl From<Timer1Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer1Rst) -> u8 {
        Timer1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer2Rst {
        Timer2Rst::from_bits(val)
    }
}
impl From<Timer2Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer2Rst) -> u8 {
        Timer2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer3Rst {
        Timer3Rst::from_bits(val)
    }
}
impl From<Timer3Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer3Rst) -> u8 {
        Timer3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer4Rst {
        Timer4Rst::from_bits(val)
    }
}
impl From<Timer4Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer4Rst) -> u8 {
        Timer4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl TraceclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivHalt {
        TraceclkdivHalt::from_bits(val)
    }
}
impl From<TraceclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivHalt) -> u8 {
        TraceclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl TraceclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReset {
        TraceclkdivReset::from_bits(val)
    }
}
impl From<TraceclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReset) -> u8 {
        TraceclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl TraceclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivUnstab {
        TraceclkdivUnstab::from_bits(val)
    }
}
impl From<TraceclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivUnstab) -> u8 {
        TraceclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "TRACECLKDIV output."]
    Enum0x0 = 0x0,
    #[doc = "Clk 1 MHz clock."]
    Enum0x1 = 0x01,
    #[doc = "LP Oscillator clock."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "No clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TroRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl TroRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TroRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TroRst {
    #[inline(always)]
    fn from(val: u8) -> TroRst {
        TroRst::from_bits(val)
    }
}
impl From<TroRst> for u8 {
    #[inline(always)]
    fn from(val: TroRst) -> u8 {
        TroRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers."]
    Enable = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    Freeze = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsPhyRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UsbHsPhyRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsPhyRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsPhyRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsPhyRst {
        UsbHsPhyRst::from_bits(val)
    }
}
impl From<UsbHsPhyRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsPhyRst) -> u8 {
        UsbHsPhyRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UsbHsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsRst {
        UsbHsRst::from_bits(val)
    }
}
impl From<UsbHsRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsRst) -> u8 {
        UsbHsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UtickRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRst {
    #[inline(always)]
    fn from(val: u8) -> UtickRst {
        UtickRst::from_bits(val)
    }
}
impl From<UtickRst> for u8 {
    #[inline(always)]
    fn from(val: UtickRst) -> u8 {
        UtickRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl UtickclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivHalt {
        UtickclkdivHalt::from_bits(val)
    }
}
impl From<UtickclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivHalt) -> u8 {
        UtickclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl UtickclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivReset {
        UtickclkdivReset::from_bits(val)
    }
}
impl From<UtickclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivReset) -> u8 {
        UtickclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl UtickclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivUnstab {
        UtickclkdivUnstab::from_bits(val)
    }
}
impl From<UtickclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivUnstab) -> u8 {
        UtickclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkselSel {
    #[doc = "clk_in."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
}
impl UtickclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickclkselSel {
        UtickclkselSel::from_bits(val)
    }
}
impl From<UtickclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickclkselSel) -> u8 {
        UtickclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VrefRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl VrefRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefRst {
    #[inline(always)]
    fn from(val: u8) -> VrefRst {
        VrefRst::from_bits(val)
    }
}
impl From<VrefRst> for u8 {
    #[inline(always)]
    fn from(val: VrefRst) -> u8 {
        VrefRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Wdt0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivHalt {
        Wdt0clkdivHalt::from_bits(val)
    }
}
impl From<Wdt0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivHalt) -> u8 {
        Wdt0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Wdt0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivReset {
        Wdt0clkdivReset::from_bits(val)
    }
}
impl From<Wdt0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivReset) -> u8 {
        Wdt0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Wdt0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivUnstab {
        Wdt0clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivUnstab) -> u8 {
        Wdt0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Wdt1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivHalt {
        Wdt1clkdivHalt::from_bits(val)
    }
}
impl From<Wdt1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivHalt) -> u8 {
        Wdt1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Wdt1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivReset {
        Wdt1clkdivReset::from_bits(val)
    }
}
impl From<Wdt1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivReset) -> u8 {
        Wdt1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Wdt1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivUnstab {
        Wdt1clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivUnstab) -> u8 {
        Wdt1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkselSel {
    #[doc = "FRO16K clock 2."]
    Enum0 = 0x0,
    #[doc = "fro_hf_div clock."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "clk_1m clock."]
    Enum3 = 0x03,
}
impl Wdt1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkselSel {
        Wdt1clkselSel::from_bits(val)
    }
}
impl From<Wdt1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkselSel) -> u8 {
        Wdt1clkselSel::to_bits(val)
    }
}
