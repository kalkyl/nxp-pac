#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule0(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule0 {
    #[doc = "eDMA0_CH15."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch15(&self) -> super::vals::EDma0Ch15 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EDma0Ch15::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH15."]
    #[inline(always)]
    pub const fn set_e_dma0_ch15(&mut self, val: super::vals::EDma0Ch15) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SCT0."]
    #[must_use]
    #[inline(always)]
    pub const fn sct0(&self) -> super::vals::Sct0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sct0::from_bits(val as u8)
    }
    #[doc = "SCT0."]
    #[inline(always)]
    pub const fn set_sct0(&mut self, val: super::vals::Sct0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LP_FLEXCOMM0."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_flexcomm0(&self) -> super::vals::LpFlexcomm0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::LpFlexcomm0::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM0."]
    #[inline(always)]
    pub const fn set_lp_flexcomm0(&mut self, val: super::vals::LpFlexcomm0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LP_FLEXCOMM1."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_flexcomm1(&self) -> super::vals::LpFlexcomm1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::LpFlexcomm1::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM1."]
    #[inline(always)]
    pub const fn set_lp_flexcomm1(&mut self, val: super::vals::LpFlexcomm1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LP_FLEXCOMM2."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_flexcomm2(&self) -> super::vals::LpFlexcomm2 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LpFlexcomm2::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM2."]
    #[inline(always)]
    pub const fn set_lp_flexcomm2(&mut self, val: super::vals::LpFlexcomm2) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LP_FLEXCOMM3."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_flexcomm3(&self) -> super::vals::LpFlexcomm3 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::LpFlexcomm3::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM3."]
    #[inline(always)]
    pub const fn set_lp_flexcomm3(&mut self, val: super::vals::LpFlexcomm3) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO0_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias0(&self) -> super::vals::Gpio0Alias0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Gpio0Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO0_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio0_alias0(&mut self, val: super::vals::Gpio0Alias0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule0 {
        AhbPeripheral0SlavePortP12SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule0")
            .field("e_dma0_ch15", &self.e_dma0_ch15())
            .field("sct0", &self.sct0())
            .field("lp_flexcomm0", &self.lp_flexcomm0())
            .field("lp_flexcomm1", &self.lp_flexcomm1())
            .field("lp_flexcomm2", &self.lp_flexcomm2())
            .field("lp_flexcomm3", &self.lp_flexcomm3())
            .field("gpio0_alias0", &self.gpio0_alias0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule0 {{ e_dma0_ch15: {:?}, sct0: {:?}, lp_flexcomm0: {:?}, lp_flexcomm1: {:?}, lp_flexcomm2: {:?}, lp_flexcomm3: {:?}, gpio0_alias0: {:?} }}",
            self.e_dma0_ch15(),
            self.sct0(),
            self.lp_flexcomm0(),
            self.lp_flexcomm1(),
            self.lp_flexcomm2(),
            self.lp_flexcomm3(),
            self.gpio0_alias0()
        )
    }
}
#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule1(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule1 {
    #[doc = "GPIO0_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias1(&self) -> super::vals::Gpio0Alias1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio0Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO0_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio0_alias1(&mut self, val: super::vals::Gpio0Alias1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO1_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias0(&self) -> super::vals::Gpio1Alias0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio1Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO1_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio1_alias0(&mut self, val: super::vals::Gpio1Alias0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "GPIO1_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias1(&self) -> super::vals::Gpio1Alias1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Gpio1Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO1_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio1_alias1(&mut self, val: super::vals::Gpio1Alias1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GPIO2_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias0(&self) -> super::vals::Gpio2Alias0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Gpio2Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO2_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio2_alias0(&mut self, val: super::vals::Gpio2Alias0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "GPIO2_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias1(&self) -> super::vals::Gpio2Alias1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Gpio2Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO2_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio2_alias1(&mut self, val: super::vals::Gpio2Alias1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "GPIO3_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias0(&self) -> super::vals::Gpio3Alias0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Gpio3Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO3_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio3_alias0(&mut self, val: super::vals::Gpio3Alias0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "GPIO3_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias1(&self) -> super::vals::Gpio3Alias1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Gpio3Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO3_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio3_alias1(&mut self, val: super::vals::Gpio3Alias1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO4_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias0(&self) -> super::vals::Gpio4Alias0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Gpio4Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO4_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio4_alias0(&mut self, val: super::vals::Gpio4Alias0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule1 {
        AhbPeripheral0SlavePortP12SlaveRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule1")
            .field("gpio0_alias1", &self.gpio0_alias1())
            .field("gpio1_alias0", &self.gpio1_alias0())
            .field("gpio1_alias1", &self.gpio1_alias1())
            .field("gpio2_alias0", &self.gpio2_alias0())
            .field("gpio2_alias1", &self.gpio2_alias1())
            .field("gpio3_alias0", &self.gpio3_alias0())
            .field("gpio3_alias1", &self.gpio3_alias1())
            .field("gpio4_alias0", &self.gpio4_alias0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule1 {{ gpio0_alias1: {:?}, gpio1_alias0: {:?}, gpio1_alias1: {:?}, gpio2_alias0: {:?}, gpio2_alias1: {:?}, gpio3_alias0: {:?}, gpio3_alias1: {:?}, gpio4_alias0: {:?} }}",
            self.gpio0_alias1(),
            self.gpio1_alias0(),
            self.gpio1_alias1(),
            self.gpio2_alias0(),
            self.gpio2_alias1(),
            self.gpio3_alias0(),
            self.gpio3_alias1(),
            self.gpio4_alias0()
        )
    }
}
#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule2(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule2 {
    #[doc = "GPIO4_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias1(&self) -> super::vals::Gpio4Alias1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio4Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO4_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio4_alias1(&mut self, val: super::vals::Gpio4Alias1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule2 {
        AhbPeripheral0SlavePortP12SlaveRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule2")
            .field("gpio4_alias1", &self.gpio4_alias1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule2 {{ gpio4_alias1: {:?} }}",
            self.gpio4_alias1()
        )
    }
}
#[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral1SlavePortP13SlaveRule0(pub u32);
impl AhbPeripheral1SlavePortP13SlaveRule0 {
    #[doc = "eDMA1_CH15."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch15(&self) -> super::vals::EDma1Ch15 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EDma1Ch15::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH15."]
    #[inline(always)]
    pub const fn set_e_dma1_ch15(&mut self, val: super::vals::EDma1Ch15) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SEMA42."]
    #[must_use]
    #[inline(always)]
    pub const fn sema42(&self) -> super::vals::Sema42 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sema42::from_bits(val as u8)
    }
    #[doc = "SEMA42."]
    #[inline(always)]
    pub const fn set_sema42(&mut self, val: super::vals::Sema42) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox(&self) -> super::vals::Mailbox {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Mailbox::from_bits(val as u8)
    }
    #[doc = "MAILBOX."]
    #[inline(always)]
    pub const fn set_mailbox(&mut self, val: super::vals::Mailbox) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PKC_RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_ram(&self) -> super::vals::PkcRam {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PkcRam::from_bits(val as u8)
    }
    #[doc = "PKC_RAM."]
    #[inline(always)]
    pub const fn set_pkc_ram(&mut self, val: super::vals::PkcRam) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FLEXCOMM4."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4(&self) -> super::vals::Flexcomm4 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flexcomm4::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4."]
    #[inline(always)]
    pub const fn set_flexcomm4(&mut self, val: super::vals::Flexcomm4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "FLEXCOMM5."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5(&self) -> super::vals::Flexcomm5 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flexcomm5::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5."]
    #[inline(always)]
    pub const fn set_flexcomm5(&mut self, val: super::vals::Flexcomm5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FLEXCOMM6."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6(&self) -> super::vals::Flexcomm6 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flexcomm6::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6."]
    #[inline(always)]
    pub const fn set_flexcomm6(&mut self, val: super::vals::Flexcomm6) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral1SlavePortP13SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeripheral1SlavePortP13SlaveRule0 {
        AhbPeripheral1SlavePortP13SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeripheral1SlavePortP13SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral1SlavePortP13SlaveRule0")
            .field("e_dma1_ch15", &self.e_dma1_ch15())
            .field("sema42", &self.sema42())
            .field("mailbox", &self.mailbox())
            .field("pkc_ram", &self.pkc_ram())
            .field("flexcomm4", &self.flexcomm4())
            .field("flexcomm5", &self.flexcomm5())
            .field("flexcomm6", &self.flexcomm6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral1SlavePortP13SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral1SlavePortP13SlaveRule0 {{ e_dma1_ch15: {:?}, sema42: {:?}, mailbox: {:?}, pkc_ram: {:?}, flexcomm4: {:?}, flexcomm5: {:?}, flexcomm6: {:?} }}",
            self.e_dma1_ch15(),
            self.sema42(),
            self.mailbox(),
            self.pkc_ram(),
            self.flexcomm4(),
            self.flexcomm5(),
            self.flexcomm6()
        )
    }
}
#[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral1SlavePortP13SlaveRule1(pub u32);
impl AhbPeripheral1SlavePortP13SlaveRule1 {
    #[doc = "FLEXCOMM7."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7(&self) -> super::vals::Flexcomm7 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexcomm7::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7."]
    #[inline(always)]
    pub const fn set_flexcomm7(&mut self, val: super::vals::Flexcomm7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FLEXCOMM8."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm8(&self) -> super::vals::Flexcomm8 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexcomm8::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM8."]
    #[inline(always)]
    pub const fn set_flexcomm8(&mut self, val: super::vals::Flexcomm8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FLEXCOMM9."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm9(&self) -> super::vals::Flexcomm9 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexcomm9::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM9."]
    #[inline(always)]
    pub const fn set_flexcomm9(&mut self, val: super::vals::Flexcomm9) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USB FS OTG RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_otg_ram(&self) -> super::vals::UsbFsOtgRam {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::UsbFsOtgRam::from_bits(val as u8)
    }
    #[doc = "USB FS OTG RAM."]
    #[inline(always)]
    pub const fn set_usb_fs_otg_ram(&mut self, val: super::vals::UsbFsOtgRam) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CDOG0."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> super::vals::Cdog0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Cdog0::from_bits(val as u8)
    }
    #[doc = "CDOG0."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: super::vals::Cdog0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CDOG1."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> super::vals::Cdog1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Cdog1::from_bits(val as u8)
    }
    #[doc = "CDOG1."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: super::vals::Cdog1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DEBUG_MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> super::vals::DebugMailbox {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::DebugMailbox::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX."]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: super::vals::DebugMailbox) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "NPU."]
    #[must_use]
    #[inline(always)]
    pub const fn npu(&self) -> super::vals::Npu {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Npu::from_bits(val as u8)
    }
    #[doc = "NPU."]
    #[inline(always)]
    pub const fn set_npu(&mut self, val: super::vals::Npu) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral1SlavePortP13SlaveRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral1SlavePortP13SlaveRule1 {
        AhbPeripheral1SlavePortP13SlaveRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral1SlavePortP13SlaveRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral1SlavePortP13SlaveRule1")
            .field("flexcomm7", &self.flexcomm7())
            .field("flexcomm8", &self.flexcomm8())
            .field("flexcomm9", &self.flexcomm9())
            .field("usb_fs_otg_ram", &self.usb_fs_otg_ram())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("npu", &self.npu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral1SlavePortP13SlaveRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral1SlavePortP13SlaveRule1 {{ flexcomm7: {:?}, flexcomm8: {:?}, flexcomm9: {:?}, usb_fs_otg_ram: {:?}, cdog0: {:?}, cdog1: {:?}, debug_mailbox: {:?}, npu: {:?} }}",
            self.flexcomm7(),
            self.flexcomm8(),
            self.flexcomm9(),
            self.usb_fs_otg_ram(),
            self.cdog0(),
            self.cdog1(),
            self.debug_mailbox(),
            self.npu()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrlPeripheralRule0(pub u32);
impl AhbSecureCtrlPeripheralRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::AhbSecureCtrlPeripheralRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AhbSecureCtrlPeripheralRule0Rule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::AhbSecureCtrlPeripheralRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::AhbSecureCtrlPeripheralRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AhbSecureCtrlPeripheralRule0Rule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::AhbSecureCtrlPeripheralRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::AhbSecureCtrlPeripheralRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbSecureCtrlPeripheralRule0Rule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::AhbSecureCtrlPeripheralRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::AhbSecureCtrlPeripheralRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AhbSecureCtrlPeripheralRule0Rule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::AhbSecureCtrlPeripheralRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AhbSecureCtrlPeripheralRule0 {
    #[inline(always)]
    fn default() -> AhbSecureCtrlPeripheralRule0 {
        AhbSecureCtrlPeripheralRule0(0)
    }
}
impl core::fmt::Debug for AhbSecureCtrlPeripheralRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSecureCtrlPeripheralRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSecureCtrlPeripheralRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSecureCtrlPeripheralRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule0(pub u32);
impl AipsBridgeGroup0MemRule0 {
    #[doc = "GPIO5_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias0(&self) -> super::vals::Gpio5Alias0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio5Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio5_alias0(&mut self, val: super::vals::Gpio5Alias0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO5_ALIAS2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias1(&self) -> super::vals::Gpio5Alias1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio5Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS2."]
    #[inline(always)]
    pub const fn set_gpio5_alias1(&mut self, val: super::vals::Gpio5Alias1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> super::vals::Port5 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Port5::from_bits(val as u8)
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: super::vals::Port5) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "FMU0."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu0(&self) -> super::vals::Fmu0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Fmu0::from_bits(val as u8)
    }
    #[doc = "FMU0."]
    #[inline(always)]
    pub const fn set_fmu0(&mut self, val: super::vals::Fmu0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SCG0."]
    #[must_use]
    #[inline(always)]
    pub const fn scg0(&self) -> super::vals::Scg0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Scg0::from_bits(val as u8)
    }
    #[doc = "SCG0."]
    #[inline(always)]
    pub const fn set_scg0(&mut self, val: super::vals::Scg0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "SPC0."]
    #[must_use]
    #[inline(always)]
    pub const fn spc0(&self) -> super::vals::Spc0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Spc0::from_bits(val as u8)
    }
    #[doc = "SPC0."]
    #[inline(always)]
    pub const fn set_spc0(&mut self, val: super::vals::Spc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WUU0."]
    #[must_use]
    #[inline(always)]
    pub const fn wuu0(&self) -> super::vals::Wuu0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wuu0::from_bits(val as u8)
    }
    #[doc = "WUU0."]
    #[inline(always)]
    pub const fn set_wuu0(&mut self, val: super::vals::Wuu0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup0MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule0 {
        AipsBridgeGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule0")
            .field("gpio5_alias0", &self.gpio5_alias0())
            .field("gpio5_alias1", &self.gpio5_alias1())
            .field("port5", &self.port5())
            .field("fmu0", &self.fmu0())
            .field("scg0", &self.scg0())
            .field("spc0", &self.spc0())
            .field("wuu0", &self.wuu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule0 {{ gpio5_alias0: {:?}, gpio5_alias1: {:?}, port5: {:?}, fmu0: {:?}, scg0: {:?}, spc0: {:?}, wuu0: {:?} }}",
            self.gpio5_alias0(),
            self.gpio5_alias1(),
            self.port5(),
            self.fmu0(),
            self.scg0(),
            self.spc0(),
            self.wuu0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule1(pub u32);
impl AipsBridgeGroup0MemRule1 {
    #[doc = "LPTMR0."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr0(&self) -> super::vals::Lptmr0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lptmr0::from_bits(val as u8)
    }
    #[doc = "LPTMR0."]
    #[inline(always)]
    pub const fn set_lptmr0(&mut self, val: super::vals::Lptmr0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPTMR1."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr1(&self) -> super::vals::Lptmr1 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lptmr1::from_bits(val as u8)
    }
    #[doc = "LPTMR1."]
    #[inline(always)]
    pub const fn set_lptmr1(&mut self, val: super::vals::Lptmr1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RTC."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> super::vals::Rtc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Rtc::from_bits(val as u8)
    }
    #[doc = "RTC."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: super::vals::Rtc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU_TEST."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_test(&self) -> super::vals::FmuTest {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::FmuTest::from_bits(val as u8)
    }
    #[doc = "FMU_TEST."]
    #[inline(always)]
    pub const fn set_fmu_test(&mut self, val: super::vals::FmuTest) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup0MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule1 {
        AipsBridgeGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule1")
            .field("lptmr0", &self.lptmr0())
            .field("lptmr1", &self.lptmr1())
            .field("rtc", &self.rtc())
            .field("fmu_test", &self.fmu_test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule1 {{ lptmr0: {:?}, lptmr1: {:?}, rtc: {:?}, fmu_test: {:?} }}",
            self.lptmr0(),
            self.lptmr1(),
            self.rtc(),
            self.fmu_test()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule2(pub u32);
impl AipsBridgeGroup0MemRule2 {
    #[doc = "TSI."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi(&self) -> super::vals::Tsi {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Tsi::from_bits(val as u8)
    }
    #[doc = "TSI."]
    #[inline(always)]
    pub const fn set_tsi(&mut self, val: super::vals::Tsi) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> super::vals::Cmp0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cmp0::from_bits(val as u8)
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: super::vals::Cmp0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> super::vals::Cmp1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Cmp1::from_bits(val as u8)
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: super::vals::Cmp1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> super::vals::Cmp2 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Cmp2::from_bits(val as u8)
    }
    #[doc = "CMP2."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: super::vals::Cmp2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ELS."]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Els {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Els::from_bits(val as u8)
    }
    #[doc = "ELS."]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Els) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ELS_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn els_alias1(&self) -> super::vals::ElsAlias1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::ElsAlias1::from_bits(val as u8)
    }
    #[doc = "ELS_ALIAS1."]
    #[inline(always)]
    pub const fn set_els_alias1(&mut self, val: super::vals::ElsAlias1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ELS_ALIAS2."]
    #[must_use]
    #[inline(always)]
    pub const fn els_alias2(&self) -> super::vals::ElsAlias2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::ElsAlias2::from_bits(val as u8)
    }
    #[doc = "ELS_ALIAS2."]
    #[inline(always)]
    pub const fn set_els_alias2(&mut self, val: super::vals::ElsAlias2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ELS_ALIAS3."]
    #[must_use]
    #[inline(always)]
    pub const fn els_alias3(&self) -> super::vals::ElsAlias3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::ElsAlias3::from_bits(val as u8)
    }
    #[doc = "ELS_ALIAS3."]
    #[inline(always)]
    pub const fn set_els_alias3(&mut self, val: super::vals::ElsAlias3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule2 {
        AipsBridgeGroup0MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule2")
            .field("tsi", &self.tsi())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("els", &self.els())
            .field("els_alias1", &self.els_alias1())
            .field("els_alias2", &self.els_alias2())
            .field("els_alias3", &self.els_alias3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule2 {{ tsi: {:?}, cmp0: {:?}, cmp1: {:?}, cmp2: {:?}, els: {:?}, els_alias1: {:?}, els_alias2: {:?}, els_alias3: {:?} }}",
            self.tsi(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.els(),
            self.els_alias1(),
            self.els_alias2(),
            self.els_alias3()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule3(pub u32);
impl AipsBridgeGroup0MemRule3 {
    #[doc = "DIGTMP."]
    #[must_use]
    #[inline(always)]
    pub const fn digtmp(&self) -> super::vals::Digtmp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Digtmp::from_bits(val as u8)
    }
    #[doc = "DIGTMP."]
    #[inline(always)]
    pub const fn set_digtmp(&mut self, val: super::vals::Digtmp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "VBAT."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> super::vals::Vbat {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Vbat::from_bits(val as u8)
    }
    #[doc = "VBAT."]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: super::vals::Vbat) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn trng(&self) -> super::vals::Trng {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Trng::from_bits(val as u8)
    }
    #[doc = "TRNG."]
    #[inline(always)]
    pub const fn set_trng(&mut self, val: super::vals::Trng) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> super::vals::Eim0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Eim0::from_bits(val as u8)
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: super::vals::Eim0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> super::vals::Erm0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Erm0::from_bits(val as u8)
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: super::vals::Erm0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "INTM0."]
    #[must_use]
    #[inline(always)]
    pub const fn intm0(&self) -> super::vals::Intm0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Intm0::from_bits(val as u8)
    }
    #[doc = "INTM0."]
    #[inline(always)]
    pub const fn set_intm0(&mut self, val: super::vals::Intm0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup0MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule3 {
        AipsBridgeGroup0MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule3")
            .field("digtmp", &self.digtmp())
            .field("vbat", &self.vbat())
            .field("trng", &self.trng())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("intm0", &self.intm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule3 {{ digtmp: {:?}, vbat: {:?}, trng: {:?}, eim0: {:?}, erm0: {:?}, intm0: {:?} }}",
            self.digtmp(),
            self.vbat(),
            self.trng(),
            self.eim0(),
            self.erm0(),
            self.intm0()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule0(pub u32);
impl AipsBridgeGroup1MemRule0 {
    #[doc = "eDMA0_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_mp(&self) -> super::vals::EDma0Mp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EDma0Mp::from_bits(val as u8)
    }
    #[doc = "eDMA0_MP."]
    #[inline(always)]
    pub const fn set_e_dma0_mp(&mut self, val: super::vals::EDma0Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA0_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch0(&self) -> super::vals::EDma0Ch0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EDma0Ch0::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH0."]
    #[inline(always)]
    pub const fn set_e_dma0_ch0(&mut self, val: super::vals::EDma0Ch0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch1(&self) -> super::vals::EDma0Ch1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EDma0Ch1::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH1."]
    #[inline(always)]
    pub const fn set_e_dma0_ch1(&mut self, val: super::vals::EDma0Ch1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA0_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch2(&self) -> super::vals::EDma0Ch2 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EDma0Ch2::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH2."]
    #[inline(always)]
    pub const fn set_e_dma0_ch2(&mut self, val: super::vals::EDma0Ch2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FLEXSPI0 Registers."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch3(&self) -> super::vals::EDma0Ch3 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::EDma0Ch3::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0 Registers."]
    #[inline(always)]
    pub const fn set_e_dma0_ch3(&mut self, val: super::vals::EDma0Ch3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA0_CH4."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch4(&self) -> super::vals::EDma0Ch4 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::EDma0Ch4::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH4."]
    #[inline(always)]
    pub const fn set_e_dma0_ch4(&mut self, val: super::vals::EDma0Ch4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA0_CH5."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch5(&self) -> super::vals::EDma0Ch5 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::EDma0Ch5::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH5."]
    #[inline(always)]
    pub const fn set_e_dma0_ch5(&mut self, val: super::vals::EDma0Ch5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA0_CH6."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch6(&self) -> super::vals::EDma0Ch6 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::EDma0Ch6::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH6."]
    #[inline(always)]
    pub const fn set_e_dma0_ch6(&mut self, val: super::vals::EDma0Ch6) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule0 {
        AipsBridgeGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule0")
            .field("e_dma0_mp", &self.e_dma0_mp())
            .field("e_dma0_ch0", &self.e_dma0_ch0())
            .field("e_dma0_ch1", &self.e_dma0_ch1())
            .field("e_dma0_ch2", &self.e_dma0_ch2())
            .field("e_dma0_ch3", &self.e_dma0_ch3())
            .field("e_dma0_ch4", &self.e_dma0_ch4())
            .field("e_dma0_ch5", &self.e_dma0_ch5())
            .field("e_dma0_ch6", &self.e_dma0_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule0 {{ e_dma0_mp: {:?}, e_dma0_ch0: {:?}, e_dma0_ch1: {:?}, e_dma0_ch2: {:?}, e_dma0_ch3: {:?}, e_dma0_ch4: {:?}, e_dma0_ch5: {:?}, e_dma0_ch6: {:?} }}",
            self.e_dma0_mp(),
            self.e_dma0_ch0(),
            self.e_dma0_ch1(),
            self.e_dma0_ch2(),
            self.e_dma0_ch3(),
            self.e_dma0_ch4(),
            self.e_dma0_ch5(),
            self.e_dma0_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule1(pub u32);
impl AipsBridgeGroup1MemRule1 {
    #[doc = "eDMA0_CH7."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch7(&self) -> super::vals::EDma0Ch7 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EDma0Ch7::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH7."]
    #[inline(always)]
    pub const fn set_e_dma0_ch7(&mut self, val: super::vals::EDma0Ch7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA0_CH8."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch8(&self) -> super::vals::EDma0Ch8 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EDma0Ch8::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH8."]
    #[inline(always)]
    pub const fn set_e_dma0_ch8(&mut self, val: super::vals::EDma0Ch8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0_CH9."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch9(&self) -> super::vals::EDma0Ch9 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EDma0Ch9::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH9."]
    #[inline(always)]
    pub const fn set_e_dma0_ch9(&mut self, val: super::vals::EDma0Ch9) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA0_CH10."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch10(&self) -> super::vals::EDma0Ch10 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EDma0Ch10::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH10."]
    #[inline(always)]
    pub const fn set_e_dma0_ch10(&mut self, val: super::vals::EDma0Ch10) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch11(&self) -> super::vals::EDma0Ch11 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::EDma0Ch11::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_e_dma0_ch11(&mut self, val: super::vals::EDma0Ch11) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA0_CH12."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch12(&self) -> super::vals::EDma0Ch12 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::EDma0Ch12::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH12."]
    #[inline(always)]
    pub const fn set_e_dma0_ch12(&mut self, val: super::vals::EDma0Ch12) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA0_CH13."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch13(&self) -> super::vals::EDma0Ch13 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::EDma0Ch13::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH13."]
    #[inline(always)]
    pub const fn set_e_dma0_ch13(&mut self, val: super::vals::EDma0Ch13) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA0_CH14."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch14(&self) -> super::vals::EDma0Ch14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::EDma0Ch14::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH14."]
    #[inline(always)]
    pub const fn set_e_dma0_ch14(&mut self, val: super::vals::EDma0Ch14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule1 {
        AipsBridgeGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule1")
            .field("e_dma0_ch7", &self.e_dma0_ch7())
            .field("e_dma0_ch8", &self.e_dma0_ch8())
            .field("e_dma0_ch9", &self.e_dma0_ch9())
            .field("e_dma0_ch10", &self.e_dma0_ch10())
            .field("e_dma0_ch11", &self.e_dma0_ch11())
            .field("e_dma0_ch12", &self.e_dma0_ch12())
            .field("e_dma0_ch13", &self.e_dma0_ch13())
            .field("e_dma0_ch14", &self.e_dma0_ch14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule1 {{ e_dma0_ch7: {:?}, e_dma0_ch8: {:?}, e_dma0_ch9: {:?}, e_dma0_ch10: {:?}, e_dma0_ch11: {:?}, e_dma0_ch12: {:?}, e_dma0_ch13: {:?}, e_dma0_ch14: {:?} }}",
            self.e_dma0_ch7(),
            self.e_dma0_ch8(),
            self.e_dma0_ch9(),
            self.e_dma0_ch10(),
            self.e_dma0_ch11(),
            self.e_dma0_ch12(),
            self.e_dma0_ch13(),
            self.e_dma0_ch14()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule0(pub u32);
impl AipsBridgeGroup2MemRule0 {
    #[doc = "eDMA1_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_mp(&self) -> super::vals::EDma1Mp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EDma1Mp::from_bits(val as u8)
    }
    #[doc = "eDMA1_MP."]
    #[inline(always)]
    pub const fn set_e_dma1_mp(&mut self, val: super::vals::EDma1Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA1_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch0(&self) -> super::vals::EDma1Ch0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EDma1Ch0::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH0."]
    #[inline(always)]
    pub const fn set_e_dma1_ch0(&mut self, val: super::vals::EDma1Ch0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA1_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch1(&self) -> super::vals::EDma1Ch1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EDma1Ch1::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH1."]
    #[inline(always)]
    pub const fn set_e_dma1_ch1(&mut self, val: super::vals::EDma1Ch1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA1_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch2(&self) -> super::vals::EDma1Ch2 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EDma1Ch2::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH2."]
    #[inline(always)]
    pub const fn set_e_dma1_ch2(&mut self, val: super::vals::EDma1Ch2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "eDMA1_CH3."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch3(&self) -> super::vals::EDma1Ch3 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::EDma1Ch3::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH3."]
    #[inline(always)]
    pub const fn set_e_dma1_ch3(&mut self, val: super::vals::EDma1Ch3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA1_CH4."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch4(&self) -> super::vals::EDma1Ch4 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::EDma1Ch4::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH4."]
    #[inline(always)]
    pub const fn set_e_dma1_ch4(&mut self, val: super::vals::EDma1Ch4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA1_CH5."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch5(&self) -> super::vals::EDma1Ch5 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::EDma1Ch5::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH5."]
    #[inline(always)]
    pub const fn set_e_dma1_ch5(&mut self, val: super::vals::EDma1Ch5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA1_CH6."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch6(&self) -> super::vals::EDma1Ch6 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::EDma1Ch6::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH6."]
    #[inline(always)]
    pub const fn set_e_dma1_ch6(&mut self, val: super::vals::EDma1Ch6) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule0 {
        AipsBridgeGroup2MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule0")
            .field("e_dma1_mp", &self.e_dma1_mp())
            .field("e_dma1_ch0", &self.e_dma1_ch0())
            .field("e_dma1_ch1", &self.e_dma1_ch1())
            .field("e_dma1_ch2", &self.e_dma1_ch2())
            .field("e_dma1_ch3", &self.e_dma1_ch3())
            .field("e_dma1_ch4", &self.e_dma1_ch4())
            .field("e_dma1_ch5", &self.e_dma1_ch5())
            .field("e_dma1_ch6", &self.e_dma1_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule0 {{ e_dma1_mp: {:?}, e_dma1_ch0: {:?}, e_dma1_ch1: {:?}, e_dma1_ch2: {:?}, e_dma1_ch3: {:?}, e_dma1_ch4: {:?}, e_dma1_ch5: {:?}, e_dma1_ch6: {:?} }}",
            self.e_dma1_mp(),
            self.e_dma1_ch0(),
            self.e_dma1_ch1(),
            self.e_dma1_ch2(),
            self.e_dma1_ch3(),
            self.e_dma1_ch4(),
            self.e_dma1_ch5(),
            self.e_dma1_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule1(pub u32);
impl AipsBridgeGroup2MemRule1 {
    #[doc = "eDMA1_CH7."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch7(&self) -> super::vals::EDma1Ch7 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EDma1Ch7::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH7."]
    #[inline(always)]
    pub const fn set_e_dma1_ch7(&mut self, val: super::vals::EDma1Ch7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for AipsBridgeGroup2MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule1 {
        AipsBridgeGroup2MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule1")
            .field("e_dma1_ch7", &self.e_dma1_ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule1 {{ e_dma1_ch7: {:?} }}",
            self.e_dma1_ch7()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule0(pub u32);
impl AipsBridgeGroup3MemRule0 {
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> super::vals::Ewm0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ewm0::from_bits(val as u8)
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: super::vals::Ewm0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPCAC."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac(&self) -> super::vals::Lpcac {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Lpcac::from_bits(val as u8)
    }
    #[doc = "LPCAC."]
    #[inline(always)]
    pub const fn set_lpcac(&mut self, val: super::vals::Lpcac) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FLEXSPI_CMX."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_cmx(&self) -> super::vals::FlexspiCmx {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::FlexspiCmx::from_bits(val as u8)
    }
    #[doc = "FLEXSPI_CMX."]
    #[inline(always)]
    pub const fn set_flexspi_cmx(&mut self, val: super::vals::FlexspiCmx) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SFA."]
    #[must_use]
    #[inline(always)]
    pub const fn sfa(&self) -> super::vals::Sfa {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Sfa::from_bits(val as u8)
    }
    #[doc = "SFA."]
    #[inline(always)]
    pub const fn set_sfa(&mut self, val: super::vals::Sfa) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "MBC."]
    #[must_use]
    #[inline(always)]
    pub const fn mbc(&self) -> super::vals::Mbc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Mbc::from_bits(val as u8)
    }
    #[doc = "MBC."]
    #[inline(always)]
    pub const fn set_mbc(&mut self, val: super::vals::Mbc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule0 {
        AipsBridgeGroup3MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule0")
            .field("ewm0", &self.ewm0())
            .field("lpcac", &self.lpcac())
            .field("flexspi_cmx", &self.flexspi_cmx())
            .field("sfa", &self.sfa())
            .field("mbc", &self.mbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule0 {{ ewm0: {:?}, lpcac: {:?}, flexspi_cmx: {:?}, sfa: {:?}, mbc: {:?} }}",
            self.ewm0(),
            self.lpcac(),
            self.flexspi_cmx(),
            self.sfa(),
            self.mbc()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule1(pub u32);
impl AipsBridgeGroup3MemRule1 {
    #[doc = "FLEXSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> super::vals::Flexspi {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi::from_bits(val as u8)
    }
    #[doc = "FLEXSPI."]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: super::vals::Flexspi) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "OTPC."]
    #[must_use]
    #[inline(always)]
    pub const fn otpc(&self) -> super::vals::Otpc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Otpc::from_bits(val as u8)
    }
    #[doc = "OTPC."]
    #[inline(always)]
    pub const fn set_otpc(&mut self, val: super::vals::Otpc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> super::vals::Crc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Crc::from_bits(val as u8)
    }
    #[doc = "CRC."]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: super::vals::Crc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "NPX."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> super::vals::Npx {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Npx::from_bits(val as u8)
    }
    #[doc = "NPX."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: super::vals::Npx) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm(&self) -> super::vals::Pwm {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Pwm::from_bits(val as u8)
    }
    #[doc = "PWM."]
    #[inline(always)]
    pub const fn set_pwm(&mut self, val: super::vals::Pwm) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "QDC."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc(&self) -> super::vals::Qdc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Qdc::from_bits(val as u8)
    }
    #[doc = "QDC."]
    #[inline(always)]
    pub const fn set_qdc(&mut self, val: super::vals::Qdc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule1 {
        AipsBridgeGroup3MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule1")
            .field("flexspi", &self.flexspi())
            .field("otpc", &self.otpc())
            .field("crc", &self.crc())
            .field("npx", &self.npx())
            .field("pwm", &self.pwm())
            .field("qdc", &self.qdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule1 {{ flexspi: {:?}, otpc: {:?}, crc: {:?}, npx: {:?}, pwm: {:?}, qdc: {:?} }}",
            self.flexspi(),
            self.otpc(),
            self.crc(),
            self.npx(),
            self.pwm(),
            self.qdc()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule2(pub u32);
impl AipsBridgeGroup3MemRule2 {
    #[doc = "PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1(&self) -> super::vals::Pwm1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pwm1::from_bits(val as u8)
    }
    #[doc = "PWM1."]
    #[inline(always)]
    pub const fn set_pwm1(&mut self, val: super::vals::Pwm1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> super::vals::Qdc1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Qdc1::from_bits(val as u8)
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: super::vals::Qdc1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "EVTG."]
    #[must_use]
    #[inline(always)]
    pub const fn evtg(&self) -> super::vals::Evtg {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Evtg::from_bits(val as u8)
    }
    #[doc = "EVTG."]
    #[inline(always)]
    pub const fn set_evtg(&mut self, val: super::vals::Evtg) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN0 RULE0."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_rule0(&self) -> super::vals::Can0Rule0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Can0Rule0::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE0."]
    #[inline(always)]
    pub const fn set_can0_rule0(&mut self, val: super::vals::Can0Rule0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CAN0 RULE1."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_rule1(&self) -> super::vals::Can0Rule1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Can0Rule1::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE1."]
    #[inline(always)]
    pub const fn set_can0_rule1(&mut self, val: super::vals::Can0Rule1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CAN0 RULE2."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_rule2(&self) -> super::vals::Can0Rule2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Can0Rule2::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE2."]
    #[inline(always)]
    pub const fn set_can0_rule2(&mut self, val: super::vals::Can0Rule2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CAN0 RULE3."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_rule3(&self) -> super::vals::Can0Rule3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Can0Rule3::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE3."]
    #[inline(always)]
    pub const fn set_can0_rule3(&mut self, val: super::vals::Can0Rule3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule2 {
        AipsBridgeGroup3MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule2")
            .field("pwm1", &self.pwm1())
            .field("qdc1", &self.qdc1())
            .field("evtg", &self.evtg())
            .field("can0_rule0", &self.can0_rule0())
            .field("can0_rule1", &self.can0_rule1())
            .field("can0_rule2", &self.can0_rule2())
            .field("can0_rule3", &self.can0_rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule2 {{ pwm1: {:?}, qdc1: {:?}, evtg: {:?}, can0_rule0: {:?}, can0_rule1: {:?}, can0_rule2: {:?}, can0_rule3: {:?} }}",
            self.pwm1(),
            self.qdc1(),
            self.evtg(),
            self.can0_rule0(),
            self.can0_rule1(),
            self.can0_rule2(),
            self.can0_rule3()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule3(pub u32);
impl AipsBridgeGroup3MemRule3 {
    #[doc = "CAN1 RULE0."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_rule0(&self) -> super::vals::Can1Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Can1Rule0::from_bits(val as u8)
    }
    #[doc = "CAN1 RULE0."]
    #[inline(always)]
    pub const fn set_can1_rule0(&mut self, val: super::vals::Can1Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAN1 RULE1."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_rule1(&self) -> super::vals::Can1Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Can1Rule1::from_bits(val as u8)
    }
    #[doc = "CAN1 RULE1."]
    #[inline(always)]
    pub const fn set_can1_rule1(&mut self, val: super::vals::Can1Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CAN1 RULE2."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_rule2(&self) -> super::vals::Can1Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Can1Rule2::from_bits(val as u8)
    }
    #[doc = "CAN1 RULE2."]
    #[inline(always)]
    pub const fn set_can1_rule2(&mut self, val: super::vals::Can1Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN1 RULE3."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_rule3(&self) -> super::vals::Can1Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Can1Rule3::from_bits(val as u8)
    }
    #[doc = "CAN1 RULE3."]
    #[inline(always)]
    pub const fn set_can1_rule3(&mut self, val: super::vals::Can1Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "USBDCD."]
    #[must_use]
    #[inline(always)]
    pub const fn usbdcd(&self) -> super::vals::Usbdcd {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Usbdcd::from_bits(val as u8)
    }
    #[doc = "USBDCD."]
    #[inline(always)]
    pub const fn set_usbdcd(&mut self, val: super::vals::Usbdcd) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "USBFS."]
    #[must_use]
    #[inline(always)]
    pub const fn usbfs(&self) -> super::vals::Usbfs {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Usbfs::from_bits(val as u8)
    }
    #[doc = "USBFS."]
    #[inline(always)]
    pub const fn set_usbfs(&mut self, val: super::vals::Usbfs) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup3MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule3 {
        AipsBridgeGroup3MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule3")
            .field("can1_rule0", &self.can1_rule0())
            .field("can1_rule1", &self.can1_rule1())
            .field("can1_rule2", &self.can1_rule2())
            .field("can1_rule3", &self.can1_rule3())
            .field("usbdcd", &self.usbdcd())
            .field("usbfs", &self.usbfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule3 {{ can1_rule0: {:?}, can1_rule1: {:?}, can1_rule2: {:?}, can1_rule3: {:?}, usbdcd: {:?}, usbfs: {:?} }}",
            self.can1_rule0(),
            self.can1_rule1(),
            self.can1_rule2(),
            self.can1_rule3(),
            self.usbdcd(),
            self.usbfs()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule0(pub u32);
impl AipsBridgeGroup4MemRule0 {
    #[doc = "ENET."]
    #[must_use]
    #[inline(always)]
    pub const fn enet(&self) -> super::vals::Enet {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enet::from_bits(val as u8)
    }
    #[doc = "ENET."]
    #[inline(always)]
    pub const fn set_enet(&mut self, val: super::vals::Enet) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "EMVSIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn emvsim0(&self) -> super::vals::Emvsim0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Emvsim0::from_bits(val as u8)
    }
    #[doc = "EMVSIM0."]
    #[inline(always)]
    pub const fn set_emvsim0(&mut self, val: super::vals::Emvsim0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "EMVSIM1."]
    #[must_use]
    #[inline(always)]
    pub const fn emvsim1(&self) -> super::vals::Emvsim1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Emvsim1::from_bits(val as u8)
    }
    #[doc = "EMVSIM1."]
    #[inline(always)]
    pub const fn set_emvsim1(&mut self, val: super::vals::Emvsim1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FLEXIO."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> super::vals::Flexio {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flexio::from_bits(val as u8)
    }
    #[doc = "FLEXIO."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: super::vals::Flexio) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "SAI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sai0(&self) -> super::vals::Sai0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Sai0::from_bits(val as u8)
    }
    #[doc = "SAI0."]
    #[inline(always)]
    pub const fn set_sai0(&mut self, val: super::vals::Sai0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "SAI1."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1(&self) -> super::vals::Sai1 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Sai1::from_bits(val as u8)
    }
    #[doc = "SAI1."]
    #[inline(always)]
    pub const fn set_sai1(&mut self, val: super::vals::Sai1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule0 {
        AipsBridgeGroup4MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule0")
            .field("enet", &self.enet())
            .field("emvsim0", &self.emvsim0())
            .field("emvsim1", &self.emvsim1())
            .field("flexio", &self.flexio())
            .field("sai0", &self.sai0())
            .field("sai1", &self.sai1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule0 {{ enet: {:?}, emvsim0: {:?}, emvsim1: {:?}, flexio: {:?}, sai0: {:?}, sai1: {:?} }}",
            self.enet(),
            self.emvsim0(),
            self.emvsim1(),
            self.flexio(),
            self.sai0(),
            self.sai1()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule1(pub u32);
impl AipsBridgeGroup4MemRule1 {
    #[doc = "SINC0."]
    #[must_use]
    #[inline(always)]
    pub const fn sinc0(&self) -> super::vals::Sinc0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sinc0::from_bits(val as u8)
    }
    #[doc = "SINC0."]
    #[inline(always)]
    pub const fn set_sinc0(&mut self, val: super::vals::Sinc0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "uSDHC0."]
    #[must_use]
    #[inline(always)]
    pub const fn u_sdhc0(&self) -> super::vals::USdhc0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::USdhc0::from_bits(val as u8)
    }
    #[doc = "uSDHC0."]
    #[inline(always)]
    pub const fn set_u_sdhc0(&mut self, val: super::vals::USdhc0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "USBHSPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbhsphy(&self) -> super::vals::Usbhsphy {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Usbhsphy::from_bits(val as u8)
    }
    #[doc = "USBHSPHY."]
    #[inline(always)]
    pub const fn set_usbhsphy(&mut self, val: super::vals::Usbhsphy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USBHS."]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs(&self) -> super::vals::Usbhs {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Usbhs::from_bits(val as u8)
    }
    #[doc = "USBHS."]
    #[inline(always)]
    pub const fn set_usbhs(&mut self, val: super::vals::Usbhs) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "MICD."]
    #[must_use]
    #[inline(always)]
    pub const fn micd(&self) -> super::vals::Micd {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Micd::from_bits(val as u8)
    }
    #[doc = "MICD."]
    #[inline(always)]
    pub const fn set_micd(&mut self, val: super::vals::Micd) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Adc0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Adc0::from_bits(val as u8)
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Adc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> super::vals::Adc1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Adc1::from_bits(val as u8)
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: super::vals::Adc1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> super::vals::Dac0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dac0::from_bits(val as u8)
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: super::vals::Dac0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule1 {
        AipsBridgeGroup4MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule1")
            .field("sinc0", &self.sinc0())
            .field("u_sdhc0", &self.u_sdhc0())
            .field("usbhsphy", &self.usbhsphy())
            .field("usbhs", &self.usbhs())
            .field("micd", &self.micd())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("dac0", &self.dac0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule1 {{ sinc0: {:?}, u_sdhc0: {:?}, usbhsphy: {:?}, usbhs: {:?}, micd: {:?}, adc0: {:?}, adc1: {:?}, dac0: {:?} }}",
            self.sinc0(),
            self.u_sdhc0(),
            self.usbhsphy(),
            self.usbhs(),
            self.micd(),
            self.adc0(),
            self.adc1(),
            self.dac0()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule2(pub u32);
impl AipsBridgeGroup4MemRule2 {
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> super::vals::Opamp0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Opamp0::from_bits(val as u8)
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: super::vals::Opamp0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "VREF."]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> super::vals::Vref {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Vref::from_bits(val as u8)
    }
    #[doc = "VREF."]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: super::vals::Vref) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DAC."]
    #[must_use]
    #[inline(always)]
    pub const fn dac(&self) -> super::vals::Dac {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dac::from_bits(val as u8)
    }
    #[doc = "DAC."]
    #[inline(always)]
    pub const fn set_dac(&mut self, val: super::vals::Dac) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "OPAMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> super::vals::Opamp1 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Opamp1::from_bits(val as u8)
    }
    #[doc = "OPAMP1."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: super::vals::Opamp1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "HPDAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn hpdac0(&self) -> super::vals::Hpdac0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Hpdac0::from_bits(val as u8)
    }
    #[doc = "HPDAC0."]
    #[inline(always)]
    pub const fn set_hpdac0(&mut self, val: super::vals::Hpdac0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "OPAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> super::vals::Opamp2 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Opamp2::from_bits(val as u8)
    }
    #[doc = "OPAMP2."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: super::vals::Opamp2) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> super::vals::Port0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Port0::from_bits(val as u8)
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: super::vals::Port0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> super::vals::Port1 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Port1::from_bits(val as u8)
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: super::vals::Port1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule2 {
        AipsBridgeGroup4MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule2")
            .field("opamp0", &self.opamp0())
            .field("vref", &self.vref())
            .field("dac", &self.dac())
            .field("opamp1", &self.opamp1())
            .field("hpdac0", &self.hpdac0())
            .field("opamp2", &self.opamp2())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule2 {{ opamp0: {:?}, vref: {:?}, dac: {:?}, opamp1: {:?}, hpdac0: {:?}, opamp2: {:?}, port0: {:?}, port1: {:?} }}",
            self.opamp0(),
            self.vref(),
            self.dac(),
            self.opamp1(),
            self.hpdac0(),
            self.opamp2(),
            self.port0(),
            self.port1()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule3(pub u32);
impl AipsBridgeGroup4MemRule3 {
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> super::vals::Port2 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Port2::from_bits(val as u8)
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: super::vals::Port2) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> super::vals::Port3 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Port3::from_bits(val as u8)
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: super::vals::Port3) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> super::vals::Port4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Port4::from_bits(val as u8)
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: super::vals::Port4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "MTR0."]
    #[must_use]
    #[inline(always)]
    pub const fn mtr0(&self) -> super::vals::Mtr0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Mtr0::from_bits(val as u8)
    }
    #[doc = "MTR0."]
    #[inline(always)]
    pub const fn set_mtr0(&mut self, val: super::vals::Mtr0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> super::vals::Atx0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Atx0::from_bits(val as u8)
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: super::vals::Atx0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule3 {
        AipsBridgeGroup4MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule3")
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("mtr0", &self.mtr0())
            .field("atx0", &self.atx0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule3 {{ port2: {:?}, port3: {:?}, port4: {:?}, mtr0: {:?}, atx0: {:?} }}",
            self.port2(),
            self.port3(),
            self.port4(),
            self.mtr0(),
            self.atx0()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule0(pub u32);
impl ApbPeripheralGroup0MemRule0 {
    #[doc = "SYSCON."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> super::vals::Syscon {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Syscon::from_bits(val as u8)
    }
    #[doc = "SYSCON."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: super::vals::Syscon) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PINT0."]
    #[must_use]
    #[inline(always)]
    pub const fn pint0(&self) -> super::vals::Pint0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Pint0::from_bits(val as u8)
    }
    #[doc = "PINT0."]
    #[inline(always)]
    pub const fn set_pint0(&mut self, val: super::vals::Pint0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> super::vals::Inputmux {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Inputmux::from_bits(val as u8)
    }
    #[doc = "INPUTMUX."]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: super::vals::Inputmux) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule0 {
        ApbPeripheralGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule0")
            .field("syscon", &self.syscon())
            .field("pint0", &self.pint0())
            .field("inputmux", &self.inputmux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule0 {{ syscon: {:?}, pint0: {:?}, inputmux: {:?} }}",
            self.syscon(),
            self.pint0(),
            self.inputmux()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule1(pub u32);
impl ApbPeripheralGroup0MemRule1 {
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> super::vals::Ctimer0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ctimer0::from_bits(val as u8)
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: super::vals::Ctimer0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> super::vals::Ctimer1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ctimer1::from_bits(val as u8)
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: super::vals::Ctimer1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> super::vals::Ctimer2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ctimer2::from_bits(val as u8)
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: super::vals::Ctimer2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> super::vals::Ctimer3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ctimer3::from_bits(val as u8)
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: super::vals::Ctimer3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule1 {
        ApbPeripheralGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule1")
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule1 {{ ctimer0: {:?}, ctimer1: {:?}, ctimer2: {:?}, ctimer3: {:?} }}",
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3()
        )
    }
}
#[doc = "APB Bridge Group 0 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule2(pub u32);
impl ApbPeripheralGroup0MemRule2 {
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> super::vals::Ctimer4 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctimer4::from_bits(val as u8)
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: super::vals::Ctimer4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FREQME0."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme0(&self) -> super::vals::Freqme0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Freqme0::from_bits(val as u8)
    }
    #[doc = "FREQME0."]
    #[inline(always)]
    pub const fn set_freqme0(&mut self, val: super::vals::Freqme0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTCIK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utcik0(&self) -> super::vals::Utcik0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Utcik0::from_bits(val as u8)
    }
    #[doc = "UTCIK0."]
    #[inline(always)]
    pub const fn set_utcik0(&mut self, val: super::vals::Utcik0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "MRT0."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0(&self) -> super::vals::Mrt0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Mrt0::from_bits(val as u8)
    }
    #[doc = "MRT0."]
    #[inline(always)]
    pub const fn set_mrt0(&mut self, val: super::vals::Mrt0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> super::vals::Ostimer0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ostimer0::from_bits(val as u8)
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: super::vals::Ostimer0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> super::vals::Wwdt0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wwdt0::from_bits(val as u8)
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: super::vals::Wwdt0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> super::vals::Wwdt1 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Wwdt1::from_bits(val as u8)
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: super::vals::Wwdt1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule2 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule2 {
        ApbPeripheralGroup0MemRule2(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule2")
            .field("ctimer4", &self.ctimer4())
            .field("freqme0", &self.freqme0())
            .field("utcik0", &self.utcik0())
            .field("mrt0", &self.mrt0())
            .field("ostimer0", &self.ostimer0())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule2 {{ ctimer4: {:?}, freqme0: {:?}, utcik0: {:?}, mrt0: {:?}, ostimer0: {:?}, wwdt0: {:?}, wwdt1: {:?} }}",
            self.ctimer4(),
            self.freqme0(),
            self.utcik0(),
            self.mrt0(),
            self.ostimer0(),
            self.wwdt0(),
            self.wwdt1()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule3(pub u32);
impl ApbPeripheralGroup0MemRule3 {
    #[doc = "CACHE64_POLSEL0."]
    #[must_use]
    #[inline(always)]
    pub const fn cache64_polsel0(&self) -> super::vals::Cache64Polsel0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Cache64Polsel0::from_bits(val as u8)
    }
    #[doc = "CACHE64_POLSEL0."]
    #[inline(always)]
    pub const fn set_cache64_polsel0(&mut self, val: super::vals::Cache64Polsel0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule3 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule3 {
        ApbPeripheralGroup0MemRule3(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule3")
            .field("cache64_polsel0", &self.cache64_polsel0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule3 {{ cache64_polsel0: {:?} }}",
            self.cache64_polsel0()
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule0(pub u32);
impl ApbPeripheralGroup1MemRule0 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> super::vals::I3c0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::I3c0::from_bits(val as u8)
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: super::vals::I3c0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> super::vals::I3c1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::I3c1::from_bits(val as u8)
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: super::vals::I3c1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GDET."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet(&self) -> super::vals::Gdet {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Gdet::from_bits(val as u8)
    }
    #[doc = "GDET."]
    #[inline(always)]
    pub const fn set_gdet(&mut self, val: super::vals::Gdet) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ITRC."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc(&self) -> super::vals::Itrc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Itrc::from_bits(val as u8)
    }
    #[doc = "ITRC."]
    #[inline(always)]
    pub const fn set_itrc(&mut self, val: super::vals::Itrc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup1MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule0 {
        ApbPeripheralGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule0")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("gdet", &self.gdet())
            .field("itrc", &self.itrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule0 {{ i3c0: {:?}, i3c1: {:?}, gdet: {:?}, itrc: {:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.gdet(),
            self.itrc()
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule1(pub u32);
impl ApbPeripheralGroup1MemRule1 {
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::ApbPeripheralGroup1MemRule1Pkc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::ApbPeripheralGroup1MemRule1Pkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::ApbPeripheralGroup1MemRule1Pkc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PUF_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_alias0(&self) -> super::vals::PufAlias0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PufAlias0::from_bits(val as u8)
    }
    #[doc = "PUF_ALIAS0."]
    #[inline(always)]
    pub const fn set_puf_alias0(&mut self, val: super::vals::PufAlias0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PUF_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_alias1(&self) -> super::vals::PufAlias1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PufAlias1::from_bits(val as u8)
    }
    #[doc = "PUF_ALIAS1."]
    #[inline(always)]
    pub const fn set_puf_alias1(&mut self, val: super::vals::PufAlias1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PUF_ALIAS2."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_alias2(&self) -> super::vals::PufAlias2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::PufAlias2::from_bits(val as u8)
    }
    #[doc = "PUF_ALIAS2."]
    #[inline(always)]
    pub const fn set_puf_alias2(&mut self, val: super::vals::PufAlias2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PUF_ALIAS3."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_alias3(&self) -> super::vals::PufAlias3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::PufAlias3::from_bits(val as u8)
    }
    #[doc = "PUF_ALIAS3."]
    #[inline(always)]
    pub const fn set_puf_alias3(&mut self, val: super::vals::PufAlias3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup1MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule1 {
        ApbPeripheralGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule1")
            .field("pkc", &self.pkc())
            .field("puf_alias0", &self.puf_alias0())
            .field("puf_alias1", &self.puf_alias1())
            .field("puf_alias2", &self.puf_alias2())
            .field("puf_alias3", &self.puf_alias3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule1 {{ pkc: {:?}, puf_alias0: {:?}, puf_alias1: {:?}, puf_alias2: {:?}, puf_alias3: {:?} }}",
            self.pkc(),
            self.puf_alias0(),
            self.puf_alias1(),
            self.puf_alias2(),
            self.puf_alias3()
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule2(pub u32);
impl ApbPeripheralGroup1MemRule2 {
    #[doc = "COOLFLUX."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux(&self) -> super::vals::Coolflux {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Coolflux::from_bits(val as u8)
    }
    #[doc = "COOLFLUX."]
    #[inline(always)]
    pub const fn set_coolflux(&mut self, val: super::vals::Coolflux) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::ApbPeripheralGroup1MemRule2Smartdma {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::ApbPeripheralGroup1MemRule2Smartdma::from_bits(val as u8)
    }
    #[doc = "SmartDMA."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::ApbPeripheralGroup1MemRule2Smartdma) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PLU."]
    #[must_use]
    #[inline(always)]
    pub const fn plu(&self) -> super::vals::Plu {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Plu::from_bits(val as u8)
    }
    #[doc = "PLU."]
    #[inline(always)]
    pub const fn set_plu(&mut self, val: super::vals::Plu) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for ApbPeripheralGroup1MemRule2 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule2 {
        ApbPeripheralGroup1MemRule2(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule2")
            .field("coolflux", &self.coolflux())
            .field("smartdma", &self.smartdma())
            .field("plu", &self.plu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule2 {{ coolflux: {:?}, smartdma: {:?}, plu: {:?} }}",
            self.coolflux(),
            self.smartdma(),
            self.plu()
        )
    }
}
#[doc = "Miscellaneous CPU0 Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0LockReg(pub u32);
impl Cpu0LockReg {
    #[doc = "LOCK_NS_VTOR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> super::vals::LockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: super::vals::LockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> super::vals::LockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::LockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: super::vals::LockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> super::vals::LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: super::vals::LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LOCK_S_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> super::vals::LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::LockSMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_S_MPU."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: super::vals::LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "LOCK_SAU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> super::vals::LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LockSau::from_bits(val as u8)
    }
    #[doc = "LOCK_SAU."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: super::vals::LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CM33_LOCK_REG_LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lock_reg_lock(&self) -> super::vals::Cm33LockRegLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cm33LockRegLock::from_bits(val as u8)
    }
    #[doc = "CM33_LOCK_REG_LOCK."]
    #[inline(always)]
    pub const fn set_cm33_lock_reg_lock(&mut self, val: super::vals::Cm33LockRegLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cpu0LockReg {
    #[inline(always)]
    fn default() -> Cpu0LockReg {
        Cpu0LockReg(0)
    }
}
impl core::fmt::Debug for Cpu0LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .field("cm33_lock_reg_lock", &self.cm33_lock_reg_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?}, cm33_lock_reg_lock: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau(),
            self.cm33_lock_reg_lock()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash00MemRule(pub u32);
impl Flash00MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash00MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash00MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash00MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash00MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash00MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash00MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash00MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash00MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash00MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash00MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash00MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash00MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flash00MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flash00MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flash00MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flash00MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flash00MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flash00MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flash00MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flash00MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flash00MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flash00MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flash00MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flash00MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flash00MemRule {
    #[inline(always)]
    fn default() -> Flash00MemRule {
        Flash00MemRule(0)
    }
}
impl core::fmt::Debug for Flash00MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash00MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash00MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash00MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash02MemRule(pub u32);
impl Flash02MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash02MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash02MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash02MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash02MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash02MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash02MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash02MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash02MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash02MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash02MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash02MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash02MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flash02MemRule {
    #[inline(always)]
    fn default() -> Flash02MemRule {
        Flash02MemRule(0)
    }
}
impl core::fmt::Debug for Flash02MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash02MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash02MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash02MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash03MemRule(pub u32);
impl Flash03MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash03MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash03MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash03MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash03MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash03MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash03MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash03MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash03MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash03MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash03MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash03MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash03MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flash03MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flash03MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flash03MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flash03MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flash03MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flash03MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flash03MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flash03MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flash03MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flash03MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flash03MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flash03MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flash03MemRule {
    #[inline(always)]
    fn default() -> Flash03MemRule {
        Flash03MemRule(0)
    }
}
impl core::fmt::Debug for Flash03MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash03MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash03MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash03MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::MasterSecAntiPolRegSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecAntiPolRegSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::MasterSecAntiPolRegSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0(&self) -> super::vals::MasterSecAntiPolRegEDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecAntiPolRegEDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_e_dma0(&mut self, val: super::vals::MasterSecAntiPolRegEDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1(&self) -> super::vals::MasterSecAntiPolRegEDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecAntiPolRegEDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_e_dma1(&mut self, val: super::vals::MasterSecAntiPolRegEDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::MasterSecAntiPolRegPkc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecAntiPolRegPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::MasterSecAntiPolRegPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "USB HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> super::vals::MasterSecAntiPolRegUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::MasterSecAntiPolRegUsbHs::from_bits(val as u8)
    }
    #[doc = "USB HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: super::vals::MasterSecAntiPolRegUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Master SEC Level Antipol Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_antipol_lock(&self) -> super::vals::MasterSecLevelAntipolLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelAntipolLock::from_bits(val as u8)
    }
    #[doc = "Master SEC Level Antipol Lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_antipol_lock(
        &mut self,
        val: super::vals::MasterSecLevelAntipolLock,
    ) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("smartdma", &self.smartdma())
            .field("e_dma0", &self.e_dma0())
            .field("e_dma1", &self.e_dma1())
            .field("pkc", &self.pkc())
            .field("usb_hs", &self.usb_hs())
            .field(
                "master_sec_level_antipol_lock",
                &self.master_sec_level_antipol_lock(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ smartdma: {:?}, e_dma0: {:?}, e_dma1: {:?}, pkc: {:?}, usb_hs: {:?}, master_sec_level_antipol_lock: {:?} }}",
            self.smartdma(),
            self.e_dma0(),
            self.e_dma1(),
            self.pkc(),
            self.usb_hs(),
            self.master_sec_level_antipol_lock()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::MasterSecLevelSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecLevelSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::MasterSecLevelSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0(&self) -> super::vals::MasterSecLevelEDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecLevelEDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_e_dma0(&mut self, val: super::vals::MasterSecLevelEDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1(&self) -> super::vals::MasterSecLevelEDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecLevelEDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_e_dma1(&mut self, val: super::vals::MasterSecLevelEDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::MasterSecLevelPkc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecLevelPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::MasterSecLevelPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "USB HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> super::vals::MasterSecLevelUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::MasterSecLevelUsbHs::from_bits(val as u8)
    }
    #[doc = "USB HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: super::vals::MasterSecLevelUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Master SEC Level Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_lock(&self) -> super::vals::MasterSecLevelLock {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::MasterSecLevelLock::from_bits(val as u8)
    }
    #[doc = "Master SEC Level Lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_lock(&mut self, val: super::vals::MasterSecLevelLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("smartdma", &self.smartdma())
            .field("e_dma0", &self.e_dma0())
            .field("e_dma1", &self.e_dma1())
            .field("pkc", &self.pkc())
            .field("usb_hs", &self.usb_hs())
            .field("master_sec_level_lock", &self.master_sec_level_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ smartdma: {:?}, e_dma0: {:?}, e_dma1: {:?}, pkc: {:?}, usb_hs: {:?}, master_sec_level_lock: {:?} }}",
            self.smartdma(),
            self.e_dma0(),
            self.e_dma1(),
            self.pkc(),
            self.usb_hs(),
            self.master_sec_level_lock()
        )
    }
}
#[doc = "Secure Control Duplicate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlDpRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlDpRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlDpRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlDpRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> super::vals::MiscCtrlDpRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlDpRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlDpRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlDpRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Secure Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: super::vals::MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> super::vals::MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlRegDisableStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "RAMA Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamaMemRule(pub u32);
impl RamaMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamaMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamaMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamaMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamaMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamaMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamaMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamaMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamaMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamaMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamaMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamaMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamaMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamaMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamaMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamaMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamaMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamaMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamaMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamaMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamaMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamaMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamaMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamaMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamaMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamaMemRule {
    #[inline(always)]
    fn default() -> RamaMemRule {
        RamaMemRule(0)
    }
}
impl core::fmt::Debug for RamaMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamaMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamaMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamaMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMB Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RambMemRule(pub u32);
impl RambMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RambMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RambMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RambMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RambMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RambMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RambMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RambMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RambMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RambMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RambMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RambMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RambMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RambMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RambMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RambMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RambMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RambMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RambMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RambMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RambMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RambMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RambMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RambMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RambMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RambMemRule {
    #[inline(always)]
    fn default() -> RambMemRule {
        RambMemRule(0)
    }
}
impl core::fmt::Debug for RambMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RambMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RambMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RambMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMC Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamcMemRule(pub u32);
impl RamcMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamcMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamcMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamcMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamcMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamcMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamcMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamcMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamcMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamcMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamcMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamcMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamcMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamcMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamcMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamcMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamcMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamcMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamcMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamcMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamcMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamcMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamcMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamcMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamcMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamcMemRule {
    #[inline(always)]
    fn default() -> RamcMemRule {
        RamcMemRule(0)
    }
}
impl core::fmt::Debug for RamcMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamcMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamcMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamcMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMD Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamdMemRule(pub u32);
impl RamdMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamdMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamdMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamdMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamdMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamdMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamdMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamdMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamdMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamdMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamdMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamdMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamdMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamdMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamdMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamdMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamdMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamdMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamdMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamdMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamdMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamdMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamdMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamdMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamdMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamdMemRule {
    #[inline(always)]
    fn default() -> RamdMemRule {
        RamdMemRule(0)
    }
}
impl core::fmt::Debug for RamdMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamdMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamdMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamdMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAME Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RameMemRule(pub u32);
impl RameMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RameMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RameMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RameMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RameMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RameMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RameMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RameMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RameMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RameMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RameMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RameMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RameMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RameMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RameMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RameMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RameMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RameMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RameMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RameMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RameMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RameMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RameMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RameMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RameMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RameMemRule {
    #[inline(always)]
    fn default() -> RameMemRule {
        RameMemRule(0)
    }
}
impl core::fmt::Debug for RameMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RameMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RameMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RameMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMX Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamxMemRule(pub u32);
impl RamxMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamxMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamxMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamxMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamxMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamxMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamxMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamxMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamxMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamxMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamxMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamxMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamxMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamxMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamxMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamxMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamxMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamxMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamxMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamxMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamxMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamxMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamxMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamxMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamxMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamxMemRule {
    #[inline(always)]
    fn default() -> RamxMemRule {
        RamxMemRule(0)
    }
}
impl core::fmt::Debug for RamxMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamxMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamxMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamxMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "ROM Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomMemRule(pub u32);
impl RomMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RomMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RomMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RomMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RomMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RomMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RomMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RomMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RomMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RomMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RomMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RomMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RomMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RomMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RomMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RomMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RomMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RomMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RomMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RomMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RomMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RomMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RomMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RomMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RomMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RomMemRule {
    #[inline(always)]
    fn default() -> RomMemRule {
        RomMemRule(0)
    }
}
impl core::fmt::Debug for RomMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RomMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "GPIO Mask for Port index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask(pub u32);
impl SecGpioMask {
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin0_sec_mask(&self) -> super::vals::Pio0Pin0SecMask {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pio0Pin0SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin0_sec_mask(&mut self, val: super::vals::Pio0Pin0SecMask) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin1_sec_mask(&self) -> super::vals::Pio0Pin1SecMask {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pio0Pin1SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin1_sec_mask(&mut self, val: super::vals::Pio0Pin1SecMask) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin2_sec_mask(&self) -> super::vals::Pio0Pin2SecMask {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pio0Pin2SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin2_sec_mask(&mut self, val: super::vals::Pio0Pin2SecMask) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin3_sec_mask(&self) -> super::vals::Pio0Pin3SecMask {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pio0Pin3SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin3_sec_mask(&mut self, val: super::vals::Pio0Pin3SecMask) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin4_sec_mask(&self) -> super::vals::Pio0Pin4SecMask {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pio0Pin4SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin4_sec_mask(&mut self, val: super::vals::Pio0Pin4SecMask) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin5_sec_mask(&self) -> super::vals::Pio0Pin5SecMask {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pio0Pin5SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin5_sec_mask(&mut self, val: super::vals::Pio0Pin5SecMask) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin6_sec_mask(&self) -> super::vals::Pio0Pin6SecMask {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio0Pin6SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin6_sec_mask(&mut self, val: super::vals::Pio0Pin6SecMask) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin7_sec_mask(&self) -> super::vals::Pio0Pin7SecMask {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pio0Pin7SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin7_sec_mask(&mut self, val: super::vals::Pio0Pin7SecMask) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin8_sec_mask(&self) -> super::vals::Pio0Pin8SecMask {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio0Pin8SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin8_sec_mask(&mut self, val: super::vals::Pio0Pin8SecMask) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin9_sec_mask(&self) -> super::vals::Pio0Pin9SecMask {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio0Pin9SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin9_sec_mask(&mut self, val: super::vals::Pio0Pin9SecMask) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin10_sec_mask(&self) -> super::vals::Pio0Pin10SecMask {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio0Pin10SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin10_sec_mask(&mut self, val: super::vals::Pio0Pin10SecMask) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin11_sec_mask(&self) -> super::vals::Pio0Pin11SecMask {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pio0Pin11SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin11_sec_mask(&mut self, val: super::vals::Pio0Pin11SecMask) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin12_sec_mask(&self) -> super::vals::Pio0Pin12SecMask {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pio0Pin12SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin12_sec_mask(&mut self, val: super::vals::Pio0Pin12SecMask) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin13_sec_mask(&self) -> super::vals::Pio0Pin13SecMask {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pio0Pin13SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin13_sec_mask(&mut self, val: super::vals::Pio0Pin13SecMask) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin14_sec_mask(&self) -> super::vals::Pio0Pin14SecMask {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pio0Pin14SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin14_sec_mask(&mut self, val: super::vals::Pio0Pin14SecMask) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin15_sec_mask(&self) -> super::vals::Pio0Pin15SecMask {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pio0Pin15SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin15_sec_mask(&mut self, val: super::vals::Pio0Pin15SecMask) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin16_sec_mask(&self) -> super::vals::Pio0Pin16SecMask {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pio0Pin16SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin16_sec_mask(&mut self, val: super::vals::Pio0Pin16SecMask) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin17_sec_mask(&self) -> super::vals::Pio0Pin17SecMask {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pio0Pin17SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin17_sec_mask(&mut self, val: super::vals::Pio0Pin17SecMask) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin18_sec_mask(&self) -> super::vals::Pio0Pin18SecMask {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pio0Pin18SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin18_sec_mask(&mut self, val: super::vals::Pio0Pin18SecMask) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin19_sec_mask(&self) -> super::vals::Pio0Pin19SecMask {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pio0Pin19SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin19_sec_mask(&mut self, val: super::vals::Pio0Pin19SecMask) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin20_sec_mask(&self) -> super::vals::Pio0Pin20SecMask {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pio0Pin20SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin20_sec_mask(&mut self, val: super::vals::Pio0Pin20SecMask) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin21_sec_mask(&self) -> super::vals::Pio0Pin21SecMask {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pio0Pin21SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin21_sec_mask(&mut self, val: super::vals::Pio0Pin21SecMask) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin22_sec_mask(&self) -> super::vals::Pio0Pin22SecMask {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pio0Pin22SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin22_sec_mask(&mut self, val: super::vals::Pio0Pin22SecMask) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin23_sec_mask(&self) -> super::vals::Pio0Pin23SecMask {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pio0Pin23SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin23_sec_mask(&mut self, val: super::vals::Pio0Pin23SecMask) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin24_sec_mask(&self) -> super::vals::Pio0Pin24SecMask {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pio0Pin24SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin24_sec_mask(&mut self, val: super::vals::Pio0Pin24SecMask) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin25_sec_mask(&self) -> super::vals::Pio0Pin25SecMask {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pio0Pin25SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin25_sec_mask(&mut self, val: super::vals::Pio0Pin25SecMask) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin26_sec_mask(&self) -> super::vals::Pio0Pin26SecMask {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pio0Pin26SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin26_sec_mask(&mut self, val: super::vals::Pio0Pin26SecMask) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin27_sec_mask(&self) -> super::vals::Pio0Pin27SecMask {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pio0Pin27SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin27_sec_mask(&mut self, val: super::vals::Pio0Pin27SecMask) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin28_sec_mask(&self) -> super::vals::Pio0Pin28SecMask {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pio0Pin28SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin28_sec_mask(&mut self, val: super::vals::Pio0Pin28SecMask) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin29_sec_mask(&self) -> super::vals::Pio0Pin29SecMask {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pio0Pin29SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin29_sec_mask(&mut self, val: super::vals::Pio0Pin29SecMask) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin30_sec_mask(&self) -> super::vals::Pio0Pin30SecMask {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pio0Pin30SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin30_sec_mask(&mut self, val: super::vals::Pio0Pin30SecMask) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin31_sec_mask(&self) -> super::vals::Pio0Pin31SecMask {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pio0Pin31SecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin31_sec_mask(&mut self, val: super::vals::Pio0Pin31SecMask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpioMask {
    #[inline(always)]
    fn default() -> SecGpioMask {
        SecGpioMask(0)
    }
}
impl core::fmt::Debug for SecGpioMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask")
            .field("pio0_pin0_sec_mask", &self.pio0_pin0_sec_mask())
            .field("pio0_pin1_sec_mask", &self.pio0_pin1_sec_mask())
            .field("pio0_pin2_sec_mask", &self.pio0_pin2_sec_mask())
            .field("pio0_pin3_sec_mask", &self.pio0_pin3_sec_mask())
            .field("pio0_pin4_sec_mask", &self.pio0_pin4_sec_mask())
            .field("pio0_pin5_sec_mask", &self.pio0_pin5_sec_mask())
            .field("pio0_pin6_sec_mask", &self.pio0_pin6_sec_mask())
            .field("pio0_pin7_sec_mask", &self.pio0_pin7_sec_mask())
            .field("pio0_pin8_sec_mask", &self.pio0_pin8_sec_mask())
            .field("pio0_pin9_sec_mask", &self.pio0_pin9_sec_mask())
            .field("pio0_pin10_sec_mask", &self.pio0_pin10_sec_mask())
            .field("pio0_pin11_sec_mask", &self.pio0_pin11_sec_mask())
            .field("pio0_pin12_sec_mask", &self.pio0_pin12_sec_mask())
            .field("pio0_pin13_sec_mask", &self.pio0_pin13_sec_mask())
            .field("pio0_pin14_sec_mask", &self.pio0_pin14_sec_mask())
            .field("pio0_pin15_sec_mask", &self.pio0_pin15_sec_mask())
            .field("pio0_pin16_sec_mask", &self.pio0_pin16_sec_mask())
            .field("pio0_pin17_sec_mask", &self.pio0_pin17_sec_mask())
            .field("pio0_pin18_sec_mask", &self.pio0_pin18_sec_mask())
            .field("pio0_pin19_sec_mask", &self.pio0_pin19_sec_mask())
            .field("pio0_pin20_sec_mask", &self.pio0_pin20_sec_mask())
            .field("pio0_pin21_sec_mask", &self.pio0_pin21_sec_mask())
            .field("pio0_pin22_sec_mask", &self.pio0_pin22_sec_mask())
            .field("pio0_pin23_sec_mask", &self.pio0_pin23_sec_mask())
            .field("pio0_pin24_sec_mask", &self.pio0_pin24_sec_mask())
            .field("pio0_pin25_sec_mask", &self.pio0_pin25_sec_mask())
            .field("pio0_pin26_sec_mask", &self.pio0_pin26_sec_mask())
            .field("pio0_pin27_sec_mask", &self.pio0_pin27_sec_mask())
            .field("pio0_pin28_sec_mask", &self.pio0_pin28_sec_mask())
            .field("pio0_pin29_sec_mask", &self.pio0_pin29_sec_mask())
            .field("pio0_pin30_sec_mask", &self.pio0_pin30_sec_mask())
            .field("pio0_pin31_sec_mask", &self.pio0_pin31_sec_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask {{ pio0_pin0_sec_mask: {:?}, pio0_pin1_sec_mask: {:?}, pio0_pin2_sec_mask: {:?}, pio0_pin3_sec_mask: {:?}, pio0_pin4_sec_mask: {:?}, pio0_pin5_sec_mask: {:?}, pio0_pin6_sec_mask: {:?}, pio0_pin7_sec_mask: {:?}, pio0_pin8_sec_mask: {:?}, pio0_pin9_sec_mask: {:?}, pio0_pin10_sec_mask: {:?}, pio0_pin11_sec_mask: {:?}, pio0_pin12_sec_mask: {:?}, pio0_pin13_sec_mask: {:?}, pio0_pin14_sec_mask: {:?}, pio0_pin15_sec_mask: {:?}, pio0_pin16_sec_mask: {:?}, pio0_pin17_sec_mask: {:?}, pio0_pin18_sec_mask: {:?}, pio0_pin19_sec_mask: {:?}, pio0_pin20_sec_mask: {:?}, pio0_pin21_sec_mask: {:?}, pio0_pin22_sec_mask: {:?}, pio0_pin23_sec_mask: {:?}, pio0_pin24_sec_mask: {:?}, pio0_pin25_sec_mask: {:?}, pio0_pin26_sec_mask: {:?}, pio0_pin27_sec_mask: {:?}, pio0_pin28_sec_mask: {:?}, pio0_pin29_sec_mask: {:?}, pio0_pin30_sec_mask: {:?}, pio0_pin31_sec_mask: {:?} }}",
            self.pio0_pin0_sec_mask(),
            self.pio0_pin1_sec_mask(),
            self.pio0_pin2_sec_mask(),
            self.pio0_pin3_sec_mask(),
            self.pio0_pin4_sec_mask(),
            self.pio0_pin5_sec_mask(),
            self.pio0_pin6_sec_mask(),
            self.pio0_pin7_sec_mask(),
            self.pio0_pin8_sec_mask(),
            self.pio0_pin9_sec_mask(),
            self.pio0_pin10_sec_mask(),
            self.pio0_pin11_sec_mask(),
            self.pio0_pin12_sec_mask(),
            self.pio0_pin13_sec_mask(),
            self.pio0_pin14_sec_mask(),
            self.pio0_pin15_sec_mask(),
            self.pio0_pin16_sec_mask(),
            self.pio0_pin17_sec_mask(),
            self.pio0_pin18_sec_mask(),
            self.pio0_pin19_sec_mask(),
            self.pio0_pin20_sec_mask(),
            self.pio0_pin21_sec_mask(),
            self.pio0_pin22_sec_mask(),
            self.pio0_pin23_sec_mask(),
            self.pio0_pin24_sec_mask(),
            self.pio0_pin25_sec_mask(),
            self.pio0_pin26_sec_mask(),
            self.pio0_pin27_sec_mask(),
            self.pio0_pin28_sec_mask(),
            self.pio0_pin29_sec_mask(),
            self.pio0_pin30_sec_mask(),
            self.pio0_pin31_sec_mask()
        )
    }
}
#[doc = "Security Violation Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "Security Violation Info Validity for Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "Violation information valid flag for AHB port 0."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 0."]
    #[inline(always)]
    pub const fn set_vio_info_valid0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Violation information valid flag for AHB port 1."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 1."]
    #[inline(always)]
    pub const fn set_vio_info_valid1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Violation information valid flag for AHB port 2."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 2."]
    #[inline(always)]
    pub const fn set_vio_info_valid2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Violation information valid flag for AHB port 3."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 3."]
    #[inline(always)]
    pub const fn set_vio_info_valid3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Violation information valid flag for AHB port 4."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 4."]
    #[inline(always)]
    pub const fn set_vio_info_valid4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Violation information valid flag for AHB port 5."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 5."]
    #[inline(always)]
    pub const fn set_vio_info_valid5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Violation information valid flag for AHB port 6."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 6."]
    #[inline(always)]
    pub const fn set_vio_info_valid6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Violation information valid flag for AHB port 7."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 7."]
    #[inline(always)]
    pub const fn set_vio_info_valid7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Violation information valid flag for AHB port 8."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 8."]
    #[inline(always)]
    pub const fn set_vio_info_valid8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Violation information valid flag for AHB port 9."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 9."]
    #[inline(always)]
    pub const fn set_vio_info_valid9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Violation information valid flag for AHB port 10."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 10."]
    #[inline(always)]
    pub const fn set_vio_info_valid10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Violation information valid flag for AHB port 11."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 11."]
    #[inline(always)]
    pub const fn set_vio_info_valid11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Violation information valid flag for AHB port 12."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 12."]
    #[inline(always)]
    pub const fn set_vio_info_valid12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Violation information valid flag for AHB port 13."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 13."]
    #[inline(always)]
    pub const fn set_vio_info_valid13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Violation information valid flag for AHB port 14."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 14."]
    #[inline(always)]
    pub const fn set_vio_info_valid14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Violation information valid flag for AHB port 15."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 15."]
    #[inline(always)]
    pub const fn set_vio_info_valid15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Violation information valid flag for AHB port 16."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 16."]
    #[inline(always)]
    pub const fn set_vio_info_valid16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Violation information valid flag for AHB port 17."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 17."]
    #[inline(always)]
    pub const fn set_vio_info_valid17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Violation information valid flag for AHB port 18."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 18."]
    #[inline(always)]
    pub const fn set_vio_info_valid18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid0", &self.vio_info_valid0())
            .field("vio_info_valid1", &self.vio_info_valid1())
            .field("vio_info_valid2", &self.vio_info_valid2())
            .field("vio_info_valid3", &self.vio_info_valid3())
            .field("vio_info_valid4", &self.vio_info_valid4())
            .field("vio_info_valid5", &self.vio_info_valid5())
            .field("vio_info_valid6", &self.vio_info_valid6())
            .field("vio_info_valid7", &self.vio_info_valid7())
            .field("vio_info_valid8", &self.vio_info_valid8())
            .field("vio_info_valid9", &self.vio_info_valid9())
            .field("vio_info_valid10", &self.vio_info_valid10())
            .field("vio_info_valid11", &self.vio_info_valid11())
            .field("vio_info_valid12", &self.vio_info_valid12())
            .field("vio_info_valid13", &self.vio_info_valid13())
            .field("vio_info_valid14", &self.vio_info_valid14())
            .field("vio_info_valid15", &self.vio_info_valid15())
            .field("vio_info_valid16", &self.vio_info_valid16())
            .field("vio_info_valid17", &self.vio_info_valid17())
            .field("vio_info_valid18", &self.vio_info_valid18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid0: {=bool:?}, vio_info_valid1: {=bool:?}, vio_info_valid2: {=bool:?}, vio_info_valid3: {=bool:?}, vio_info_valid4: {=bool:?}, vio_info_valid5: {=bool:?}, vio_info_valid6: {=bool:?}, vio_info_valid7: {=bool:?}, vio_info_valid8: {=bool:?}, vio_info_valid9: {=bool:?}, vio_info_valid10: {=bool:?}, vio_info_valid11: {=bool:?}, vio_info_valid12: {=bool:?}, vio_info_valid13: {=bool:?}, vio_info_valid14: {=bool:?}, vio_info_valid15: {=bool:?}, vio_info_valid16: {=bool:?}, vio_info_valid17: {=bool:?}, vio_info_valid18: {=bool:?} }}",
            self.vio_info_valid0(),
            self.vio_info_valid1(),
            self.vio_info_valid2(),
            self.vio_info_valid3(),
            self.vio_info_valid4(),
            self.vio_info_valid5(),
            self.vio_info_valid6(),
            self.vio_info_valid7(),
            self.vio_info_valid8(),
            self.vio_info_valid9(),
            self.vio_info_valid10(),
            self.vio_info_valid11(),
            self.vio_info_valid12(),
            self.vio_info_valid13(),
            self.vio_info_valid14(),
            self.vio_info_valid15(),
            self.vio_info_valid16(),
            self.vio_info_valid17(),
            self.vio_info_valid18()
        )
    }
}
#[doc = "Security Violation Miscellaneous Information at Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "Security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> super::vals::SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "Security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: super::vals::SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation Info Data Access."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> super::vals::SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "Security Violation Info Data Access."]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: super::vals::SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Security violation master number."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> super::vals::SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "Security violation master number."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: super::vals::SecVioInfoMaster) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
