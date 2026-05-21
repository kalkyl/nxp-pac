#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Adc0 {
        Adc0::from_bits(val)
    }
}
impl From<Adc0> for u8 {
    #[inline(always)]
    fn from(val: Adc0) -> u8 {
        Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Adc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1 {
    #[inline(always)]
    fn from(val: u8) -> Adc1 {
        Adc1::from_bits(val)
    }
}
impl From<Adc1> for u8 {
    #[inline(always)]
    fn from(val: Adc1) -> u8 {
        Adc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlPeripheralRule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl AhbSecureCtrlPeripheralRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlPeripheralRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlPeripheralRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlPeripheralRule0Rule0 {
        AhbSecureCtrlPeripheralRule0Rule0::from_bits(val)
    }
}
impl From<AhbSecureCtrlPeripheralRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlPeripheralRule0Rule0) -> u8 {
        AhbSecureCtrlPeripheralRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlPeripheralRule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl AhbSecureCtrlPeripheralRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlPeripheralRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlPeripheralRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlPeripheralRule0Rule1 {
        AhbSecureCtrlPeripheralRule0Rule1::from_bits(val)
    }
}
impl From<AhbSecureCtrlPeripheralRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlPeripheralRule0Rule1) -> u8 {
        AhbSecureCtrlPeripheralRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlPeripheralRule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl AhbSecureCtrlPeripheralRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlPeripheralRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlPeripheralRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlPeripheralRule0Rule2 {
        AhbSecureCtrlPeripheralRule0Rule2::from_bits(val)
    }
}
impl From<AhbSecureCtrlPeripheralRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlPeripheralRule0Rule2) -> u8 {
        AhbSecureCtrlPeripheralRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlPeripheralRule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl AhbSecureCtrlPeripheralRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlPeripheralRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlPeripheralRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlPeripheralRule0Rule3 {
        AhbSecureCtrlPeripheralRule0Rule3::from_bits(val)
    }
}
impl From<AhbSecureCtrlPeripheralRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlPeripheralRule0Rule3) -> u8 {
        AhbSecureCtrlPeripheralRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup1MemRule1Pkc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ApbPeripheralGroup1MemRule1Pkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup1MemRule1Pkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup1MemRule1Pkc {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup1MemRule1Pkc {
        ApbPeripheralGroup1MemRule1Pkc::from_bits(val)
    }
}
impl From<ApbPeripheralGroup1MemRule1Pkc> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup1MemRule1Pkc) -> u8 {
        ApbPeripheralGroup1MemRule1Pkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup1MemRule2Smartdma {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ApbPeripheralGroup1MemRule2Smartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup1MemRule2Smartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup1MemRule2Smartdma {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup1MemRule2Smartdma {
        ApbPeripheralGroup1MemRule2Smartdma::from_bits(val)
    }
}
impl From<ApbPeripheralGroup1MemRule2Smartdma> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup1MemRule2Smartdma) -> u8 {
        ApbPeripheralGroup1MemRule2Smartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atx0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Atx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atx0 {
    #[inline(always)]
    fn from(val: u8) -> Atx0 {
        Atx0::from_bits(val)
    }
}
impl From<Atx0> for u8 {
    #[inline(always)]
    fn from(val: Atx0) -> u8 {
        Atx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cache64Polsel0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cache64Polsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cache64Polsel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cache64Polsel0 {
    #[inline(always)]
    fn from(val: u8) -> Cache64Polsel0 {
        Cache64Polsel0::from_bits(val)
    }
}
impl From<Cache64Polsel0> for u8 {
    #[inline(always)]
    fn from(val: Cache64Polsel0) -> u8 {
        Cache64Polsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Can0Rule0 {
        Can0Rule0::from_bits(val)
    }
}
impl From<Can0Rule0> for u8 {
    #[inline(always)]
    fn from(val: Can0Rule0) -> u8 {
        Can0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Can0Rule1 {
        Can0Rule1::from_bits(val)
    }
}
impl From<Can0Rule1> for u8 {
    #[inline(always)]
    fn from(val: Can0Rule1) -> u8 {
        Can0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Can0Rule2 {
        Can0Rule2::from_bits(val)
    }
}
impl From<Can0Rule2> for u8 {
    #[inline(always)]
    fn from(val: Can0Rule2) -> u8 {
        Can0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Can0Rule3 {
        Can0Rule3::from_bits(val)
    }
}
impl From<Can0Rule3> for u8 {
    #[inline(always)]
    fn from(val: Can0Rule3) -> u8 {
        Can0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Can1Rule0 {
        Can1Rule0::from_bits(val)
    }
}
impl From<Can1Rule0> for u8 {
    #[inline(always)]
    fn from(val: Can1Rule0) -> u8 {
        Can1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Can1Rule1 {
        Can1Rule1::from_bits(val)
    }
}
impl From<Can1Rule1> for u8 {
    #[inline(always)]
    fn from(val: Can1Rule1) -> u8 {
        Can1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Can1Rule2 {
        Can1Rule2::from_bits(val)
    }
}
impl From<Can1Rule2> for u8 {
    #[inline(always)]
    fn from(val: Can1Rule2) -> u8 {
        Can1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Can1Rule3 {
        Can1Rule3::from_bits(val)
    }
}
impl From<Can1Rule3> for u8 {
    #[inline(always)]
    fn from(val: Can1Rule3) -> u8 {
        Can1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog0 {
    #[inline(always)]
    fn from(val: u8) -> Cdog0 {
        Cdog0::from_bits(val)
    }
}
impl From<Cdog0> for u8 {
    #[inline(always)]
    fn from(val: Cdog0) -> u8 {
        Cdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog1 {
    #[inline(always)]
    fn from(val: u8) -> Cdog1 {
        Cdog1::from_bits(val)
    }
}
impl From<Cdog1> for u8 {
    #[inline(always)]
    fn from(val: Cdog1) -> u8 {
        Cdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm33LockRegLock {
    _RESERVED_0 = 0x0,
    #[doc = "CM33_LOCK_REG_LOCK is 1."]
    Cm33LockRegLockEq1 = 0x01,
    #[doc = "CM33_LOCK_REG_LOCK is 0."]
    Cm33LockRegLockEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cm33LockRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm33LockRegLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm33LockRegLock {
    #[inline(always)]
    fn from(val: u8) -> Cm33LockRegLock {
        Cm33LockRegLock::from_bits(val)
    }
}
impl From<Cm33LockRegLock> for u8 {
    #[inline(always)]
    fn from(val: Cm33LockRegLock) -> u8 {
        Cm33LockRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0 {
    #[inline(always)]
    fn from(val: u8) -> Cmp0 {
        Cmp0::from_bits(val)
    }
}
impl From<Cmp0> for u8 {
    #[inline(always)]
    fn from(val: Cmp0) -> u8 {
        Cmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cmp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1 {
    #[inline(always)]
    fn from(val: u8) -> Cmp1 {
        Cmp1::from_bits(val)
    }
}
impl From<Cmp1> for u8 {
    #[inline(always)]
    fn from(val: Cmp1) -> u8 {
        Cmp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cmp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2 {
    #[inline(always)]
    fn from(val: u8) -> Cmp2 {
        Cmp2::from_bits(val)
    }
}
impl From<Cmp2> for u8 {
    #[inline(always)]
    fn from(val: Cmp2) -> u8 {
        Cmp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coolflux {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Coolflux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coolflux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coolflux {
    #[inline(always)]
    fn from(val: u8) -> Coolflux {
        Coolflux::from_bits(val)
    }
}
impl From<Coolflux> for u8 {
    #[inline(always)]
    fn from(val: Coolflux) -> u8 {
        Coolflux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Crc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc {
    #[inline(always)]
    fn from(val: u8) -> Crc {
        Crc::from_bits(val)
    }
}
impl From<Crc> for u8 {
    #[inline(always)]
    fn from(val: Crc) -> u8 {
        Crc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0 {
        Ctimer0::from_bits(val)
    }
}
impl From<Ctimer0> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0) -> u8 {
        Ctimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1 {
        Ctimer1::from_bits(val)
    }
}
impl From<Ctimer1> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1) -> u8 {
        Ctimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2 {
        Ctimer2::from_bits(val)
    }
}
impl From<Ctimer2> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2) -> u8 {
        Ctimer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3 {
        Ctimer3::from_bits(val)
    }
}
impl From<Ctimer3> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3) -> u8 {
        Ctimer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4 {
        Ctimer4::from_bits(val)
    }
}
impl From<Ctimer4> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4) -> u8 {
        Ctimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Dac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac {
    #[inline(always)]
    fn from(val: u8) -> Dac {
        Dac::from_bits(val)
    }
}
impl From<Dac> for u8 {
    #[inline(always)]
    fn from(val: Dac) -> u8 {
        Dac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Dac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0 {
    #[inline(always)]
    fn from(val: u8) -> Dac0 {
        Dac0::from_bits(val)
    }
}
impl From<Dac0> for u8 {
    #[inline(always)]
    fn from(val: Dac0) -> u8 {
        Dac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugMailbox {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl DebugMailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugMailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugMailbox {
    #[inline(always)]
    fn from(val: u8) -> DebugMailbox {
        DebugMailbox::from_bits(val)
    }
}
impl From<DebugMailbox> for u8 {
    #[inline(always)]
    fn from(val: DebugMailbox) -> u8 {
        DebugMailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Digtmp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Digtmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Digtmp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Digtmp {
    #[inline(always)]
    fn from(val: u8) -> Digtmp {
        Digtmp::from_bits(val)
    }
}
impl From<Digtmp> for u8 {
    #[inline(always)]
    fn from(val: Digtmp) -> u8 {
        Digtmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch0 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch0 {
        EDma0Ch0::from_bits(val)
    }
}
impl From<EDma0Ch0> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch0) -> u8 {
        EDma0Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch1 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch1 {
        EDma0Ch1::from_bits(val)
    }
}
impl From<EDma0Ch1> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch1) -> u8 {
        EDma0Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch10 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch10 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch10 {
        EDma0Ch10::from_bits(val)
    }
}
impl From<EDma0Ch10> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch10) -> u8 {
        EDma0Ch10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch11 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch11 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch11 {
        EDma0Ch11::from_bits(val)
    }
}
impl From<EDma0Ch11> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch11) -> u8 {
        EDma0Ch11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch12 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch12 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch12 {
        EDma0Ch12::from_bits(val)
    }
}
impl From<EDma0Ch12> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch12) -> u8 {
        EDma0Ch12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch13 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch13 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch13 {
        EDma0Ch13::from_bits(val)
    }
}
impl From<EDma0Ch13> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch13) -> u8 {
        EDma0Ch13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch14 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch14 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch14 {
        EDma0Ch14::from_bits(val)
    }
}
impl From<EDma0Ch14> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch14) -> u8 {
        EDma0Ch14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch15 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch15 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch15 {
        EDma0Ch15::from_bits(val)
    }
}
impl From<EDma0Ch15> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch15) -> u8 {
        EDma0Ch15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch2 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch2 {
        EDma0Ch2::from_bits(val)
    }
}
impl From<EDma0Ch2> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch2) -> u8 {
        EDma0Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch3 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch3 {
        EDma0Ch3::from_bits(val)
    }
}
impl From<EDma0Ch3> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch3) -> u8 {
        EDma0Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch4 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch4 {
        EDma0Ch4::from_bits(val)
    }
}
impl From<EDma0Ch4> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch4) -> u8 {
        EDma0Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch5 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch5 {
        EDma0Ch5::from_bits(val)
    }
}
impl From<EDma0Ch5> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch5) -> u8 {
        EDma0Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch6 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch6 {
        EDma0Ch6::from_bits(val)
    }
}
impl From<EDma0Ch6> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch6) -> u8 {
        EDma0Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch7 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch7 {
        EDma0Ch7::from_bits(val)
    }
}
impl From<EDma0Ch7> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch7) -> u8 {
        EDma0Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch8 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch8 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch8 {
        EDma0Ch8::from_bits(val)
    }
}
impl From<EDma0Ch8> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch8) -> u8 {
        EDma0Ch8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch9 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch9 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch9 {
        EDma0Ch9::from_bits(val)
    }
}
impl From<EDma0Ch9> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch9) -> u8 {
        EDma0Ch9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Mp {
    #[inline(always)]
    fn from(val: u8) -> EDma0Mp {
        EDma0Mp::from_bits(val)
    }
}
impl From<EDma0Mp> for u8 {
    #[inline(always)]
    fn from(val: EDma0Mp) -> u8 {
        EDma0Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch0 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch0 {
        EDma1Ch0::from_bits(val)
    }
}
impl From<EDma1Ch0> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch0) -> u8 {
        EDma1Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch1 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch1 {
        EDma1Ch1::from_bits(val)
    }
}
impl From<EDma1Ch1> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch1) -> u8 {
        EDma1Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch15 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch15 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch15 {
        EDma1Ch15::from_bits(val)
    }
}
impl From<EDma1Ch15> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch15) -> u8 {
        EDma1Ch15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch2 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch2 {
        EDma1Ch2::from_bits(val)
    }
}
impl From<EDma1Ch2> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch2) -> u8 {
        EDma1Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch3 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch3 {
        EDma1Ch3::from_bits(val)
    }
}
impl From<EDma1Ch3> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch3) -> u8 {
        EDma1Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch4 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch4 {
        EDma1Ch4::from_bits(val)
    }
}
impl From<EDma1Ch4> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch4) -> u8 {
        EDma1Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch5 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch5 {
        EDma1Ch5::from_bits(val)
    }
}
impl From<EDma1Ch5> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch5) -> u8 {
        EDma1Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch6 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch6 {
        EDma1Ch6::from_bits(val)
    }
}
impl From<EDma1Ch6> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch6) -> u8 {
        EDma1Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch7 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch7 {
        EDma1Ch7::from_bits(val)
    }
}
impl From<EDma1Ch7> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch7) -> u8 {
        EDma1Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Mp {
    #[inline(always)]
    fn from(val: u8) -> EDma1Mp {
        EDma1Mp::from_bits(val)
    }
}
impl From<EDma1Mp> for u8 {
    #[inline(always)]
    fn from(val: EDma1Mp) -> u8 {
        EDma1Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eim0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Eim0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eim0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eim0 {
    #[inline(always)]
    fn from(val: u8) -> Eim0 {
        Eim0::from_bits(val)
    }
}
impl From<Eim0> for u8 {
    #[inline(always)]
    fn from(val: Eim0) -> u8 {
        Eim0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Els {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Els {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Els {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Els {
    #[inline(always)]
    fn from(val: u8) -> Els {
        Els::from_bits(val)
    }
}
impl From<Els> for u8 {
    #[inline(always)]
    fn from(val: Els) -> u8 {
        Els::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsAlias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ElsAlias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsAlias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsAlias1 {
    #[inline(always)]
    fn from(val: u8) -> ElsAlias1 {
        ElsAlias1::from_bits(val)
    }
}
impl From<ElsAlias1> for u8 {
    #[inline(always)]
    fn from(val: ElsAlias1) -> u8 {
        ElsAlias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsAlias2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ElsAlias2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsAlias2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsAlias2 {
    #[inline(always)]
    fn from(val: u8) -> ElsAlias2 {
        ElsAlias2::from_bits(val)
    }
}
impl From<ElsAlias2> for u8 {
    #[inline(always)]
    fn from(val: ElsAlias2) -> u8 {
        ElsAlias2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsAlias3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ElsAlias3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsAlias3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsAlias3 {
    #[inline(always)]
    fn from(val: u8) -> ElsAlias3 {
        ElsAlias3::from_bits(val)
    }
}
impl From<ElsAlias3> for u8 {
    #[inline(always)]
    fn from(val: ElsAlias3) -> u8 {
        ElsAlias3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Emvsim0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0 {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0 {
        Emvsim0::from_bits(val)
    }
}
impl From<Emvsim0> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0) -> u8 {
        Emvsim0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Emvsim1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1 {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1 {
        Emvsim1::from_bits(val)
    }
}
impl From<Emvsim1> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1) -> u8 {
        Emvsim1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Enet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet {
    #[inline(always)]
    fn from(val: u8) -> Enet {
        Enet::from_bits(val)
    }
}
impl From<Enet> for u8 {
    #[inline(always)]
    fn from(val: Enet) -> u8 {
        Enet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Erm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erm0 {
    #[inline(always)]
    fn from(val: u8) -> Erm0 {
        Erm0::from_bits(val)
    }
}
impl From<Erm0> for u8 {
    #[inline(always)]
    fn from(val: Erm0) -> u8 {
        Erm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evtg {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Evtg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evtg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evtg {
    #[inline(always)]
    fn from(val: u8) -> Evtg {
        Evtg::from_bits(val)
    }
}
impl From<Evtg> for u8 {
    #[inline(always)]
    fn from(val: Evtg) -> u8 {
        Evtg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ewm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0 {
    #[inline(always)]
    fn from(val: u8) -> Ewm0 {
        Ewm0::from_bits(val)
    }
}
impl From<Ewm0> for u8 {
    #[inline(always)]
    fn from(val: Ewm0) -> u8 {
        Ewm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule0 {
        Flash00MemRuleRule0::from_bits(val)
    }
}
impl From<Flash00MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule0) -> u8 {
        Flash00MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule1 {
        Flash00MemRuleRule1::from_bits(val)
    }
}
impl From<Flash00MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule1) -> u8 {
        Flash00MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule2 {
        Flash00MemRuleRule2::from_bits(val)
    }
}
impl From<Flash00MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule2) -> u8 {
        Flash00MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule3 {
        Flash00MemRuleRule3::from_bits(val)
    }
}
impl From<Flash00MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule3) -> u8 {
        Flash00MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule4 {
        Flash00MemRuleRule4::from_bits(val)
    }
}
impl From<Flash00MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule4) -> u8 {
        Flash00MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule5 {
        Flash00MemRuleRule5::from_bits(val)
    }
}
impl From<Flash00MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule5) -> u8 {
        Flash00MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule6 {
        Flash00MemRuleRule6::from_bits(val)
    }
}
impl From<Flash00MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule6) -> u8 {
        Flash00MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule7 {
        Flash00MemRuleRule7::from_bits(val)
    }
}
impl From<Flash00MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule7) -> u8 {
        Flash00MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash02MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule0 {
        Flash02MemRuleRule0::from_bits(val)
    }
}
impl From<Flash02MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule0) -> u8 {
        Flash02MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash02MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule1 {
        Flash02MemRuleRule1::from_bits(val)
    }
}
impl From<Flash02MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule1) -> u8 {
        Flash02MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash02MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule2 {
        Flash02MemRuleRule2::from_bits(val)
    }
}
impl From<Flash02MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule2) -> u8 {
        Flash02MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash02MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule3 {
        Flash02MemRuleRule3::from_bits(val)
    }
}
impl From<Flash02MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule3) -> u8 {
        Flash02MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule0 {
        Flash03MemRuleRule0::from_bits(val)
    }
}
impl From<Flash03MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule0) -> u8 {
        Flash03MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule1 {
        Flash03MemRuleRule1::from_bits(val)
    }
}
impl From<Flash03MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule1) -> u8 {
        Flash03MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule2 {
        Flash03MemRuleRule2::from_bits(val)
    }
}
impl From<Flash03MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule2) -> u8 {
        Flash03MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule3 {
        Flash03MemRuleRule3::from_bits(val)
    }
}
impl From<Flash03MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule3) -> u8 {
        Flash03MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule4 {
        Flash03MemRuleRule4::from_bits(val)
    }
}
impl From<Flash03MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule4) -> u8 {
        Flash03MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule5 {
        Flash03MemRuleRule5::from_bits(val)
    }
}
impl From<Flash03MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule5) -> u8 {
        Flash03MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule6 {
        Flash03MemRuleRule6::from_bits(val)
    }
}
impl From<Flash03MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule6) -> u8 {
        Flash03MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule7 {
        Flash03MemRuleRule7::from_bits(val)
    }
}
impl From<Flash03MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule7) -> u8 {
        Flash03MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4 {
        Flexcomm4::from_bits(val)
    }
}
impl From<Flexcomm4> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4) -> u8 {
        Flexcomm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5 {
        Flexcomm5::from_bits(val)
    }
}
impl From<Flexcomm5> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5) -> u8 {
        Flexcomm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6 {
        Flexcomm6::from_bits(val)
    }
}
impl From<Flexcomm6> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6) -> u8 {
        Flexcomm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7 {
        Flexcomm7::from_bits(val)
    }
}
impl From<Flexcomm7> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7) -> u8 {
        Flexcomm7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm8 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm8 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm8 {
        Flexcomm8::from_bits(val)
    }
}
impl From<Flexcomm8> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm8) -> u8 {
        Flexcomm8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm9 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm9 {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm9 {
        Flexcomm9::from_bits(val)
    }
}
impl From<Flexcomm9> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm9) -> u8 {
        Flexcomm9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio {
    #[inline(always)]
    fn from(val: u8) -> Flexio {
        Flexio::from_bits(val)
    }
}
impl From<Flexio> for u8 {
    #[inline(always)]
    fn from(val: Flexio) -> u8 {
        Flexio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi {
    #[inline(always)]
    fn from(val: u8) -> Flexspi {
        Flexspi::from_bits(val)
    }
}
impl From<Flexspi> for u8 {
    #[inline(always)]
    fn from(val: Flexspi) -> u8 {
        Flexspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiCmx {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl FlexspiCmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiCmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiCmx {
    #[inline(always)]
    fn from(val: u8) -> FlexspiCmx {
        FlexspiCmx::from_bits(val)
    }
}
impl From<FlexspiCmx> for u8 {
    #[inline(always)]
    fn from(val: FlexspiCmx) -> u8 {
        FlexspiCmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmu0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Fmu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmu0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmu0 {
    #[inline(always)]
    fn from(val: u8) -> Fmu0 {
        Fmu0::from_bits(val)
    }
}
impl From<Fmu0> for u8 {
    #[inline(always)]
    fn from(val: Fmu0) -> u8 {
        Fmu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuTest {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl FmuTest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuTest {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuTest {
    #[inline(always)]
    fn from(val: u8) -> FmuTest {
        FmuTest::from_bits(val)
    }
}
impl From<FmuTest> for u8 {
    #[inline(always)]
    fn from(val: FmuTest) -> u8 {
        FmuTest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Freqme0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme0 {
    #[inline(always)]
    fn from(val: u8) -> Freqme0 {
        Freqme0::from_bits(val)
    }
}
impl From<Freqme0> for u8 {
    #[inline(always)]
    fn from(val: Freqme0) -> u8 {
        Freqme0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gdet {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gdet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gdet {
    #[inline(always)]
    fn from(val: u8) -> Gdet {
        Gdet::from_bits(val)
    }
}
impl From<Gdet> for u8 {
    #[inline(always)]
    fn from(val: Gdet) -> u8 {
        Gdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio0Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias0 {
        Gpio0Alias0::from_bits(val)
    }
}
impl From<Gpio0Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias0) -> u8 {
        Gpio0Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio0Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias1 {
        Gpio0Alias1::from_bits(val)
    }
}
impl From<Gpio0Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias1) -> u8 {
        Gpio0Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio1Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias0 {
        Gpio1Alias0::from_bits(val)
    }
}
impl From<Gpio1Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias0) -> u8 {
        Gpio1Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio1Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias1 {
        Gpio1Alias1::from_bits(val)
    }
}
impl From<Gpio1Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias1) -> u8 {
        Gpio1Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio2Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias0 {
        Gpio2Alias0::from_bits(val)
    }
}
impl From<Gpio2Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias0) -> u8 {
        Gpio2Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio2Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias1 {
        Gpio2Alias1::from_bits(val)
    }
}
impl From<Gpio2Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias1) -> u8 {
        Gpio2Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio3Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias0 {
        Gpio3Alias0::from_bits(val)
    }
}
impl From<Gpio3Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias0) -> u8 {
        Gpio3Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio3Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias1 {
        Gpio3Alias1::from_bits(val)
    }
}
impl From<Gpio3Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias1) -> u8 {
        Gpio3Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio4Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias0 {
        Gpio4Alias0::from_bits(val)
    }
}
impl From<Gpio4Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias0) -> u8 {
        Gpio4Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio4Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias1 {
        Gpio4Alias1::from_bits(val)
    }
}
impl From<Gpio4Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias1) -> u8 {
        Gpio4Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio5Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias0 {
        Gpio5Alias0::from_bits(val)
    }
}
impl From<Gpio5Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias0) -> u8 {
        Gpio5Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio5Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias1 {
        Gpio5Alias1::from_bits(val)
    }
}
impl From<Gpio5Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias1) -> u8 {
        Gpio5Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpdac0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Hpdac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpdac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpdac0 {
    #[inline(always)]
    fn from(val: u8) -> Hpdac0 {
        Hpdac0::from_bits(val)
    }
}
impl From<Hpdac0> for u8 {
    #[inline(always)]
    fn from(val: Hpdac0) -> u8 {
        Hpdac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl I3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0 {
    #[inline(always)]
    fn from(val: u8) -> I3c0 {
        I3c0::from_bits(val)
    }
}
impl From<I3c0> for u8 {
    #[inline(always)]
    fn from(val: I3c0) -> u8 {
        I3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1 {
    #[inline(always)]
    fn from(val: u8) -> I3c1 {
        I3c1::from_bits(val)
    }
}
impl From<I3c1> for u8 {
    #[inline(always)]
    fn from(val: I3c1) -> u8 {
        I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inputmux {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Inputmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputmux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputmux {
    #[inline(always)]
    fn from(val: u8) -> Inputmux {
        Inputmux::from_bits(val)
    }
}
impl From<Inputmux> for u8 {
    #[inline(always)]
    fn from(val: Inputmux) -> u8 {
        Inputmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Intm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intm0 {
    #[inline(always)]
    fn from(val: u8) -> Intm0 {
        Intm0::from_bits(val)
    }
}
impl From<Intm0> for u8 {
    #[inline(always)]
    fn from(val: Intm0) -> u8 {
        Intm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itrc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Itrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itrc {
    #[inline(always)]
    fn from(val: u8) -> Itrc {
        Itrc::from_bits(val)
    }
}
impl From<Itrc> for u8 {
    #[inline(always)]
    fn from(val: Itrc) -> u8 {
        Itrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LockNsMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LockNsMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> LockNsMpu {
        LockNsMpu::from_bits(val)
    }
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: LockNsMpu) -> u8 {
        LockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1."]
    LockNsVtorEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LockNsVtorEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> LockNsVtor {
        LockNsVtor::from_bits(val)
    }
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: LockNsVtor) -> u8 {
        LockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1."]
    LockSMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0."]
    LockSMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1."]
    LockSVtaircrEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0."]
    LockSVtaircrEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_SAU is 1."]
    LockSauEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0."]
    LockSauEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFlexcomm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl LpFlexcomm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpFlexcomm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpFlexcomm0 {
    #[inline(always)]
    fn from(val: u8) -> LpFlexcomm0 {
        LpFlexcomm0::from_bits(val)
    }
}
impl From<LpFlexcomm0> for u8 {
    #[inline(always)]
    fn from(val: LpFlexcomm0) -> u8 {
        LpFlexcomm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFlexcomm1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl LpFlexcomm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpFlexcomm1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpFlexcomm1 {
    #[inline(always)]
    fn from(val: u8) -> LpFlexcomm1 {
        LpFlexcomm1::from_bits(val)
    }
}
impl From<LpFlexcomm1> for u8 {
    #[inline(always)]
    fn from(val: LpFlexcomm1) -> u8 {
        LpFlexcomm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFlexcomm2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl LpFlexcomm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpFlexcomm2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpFlexcomm2 {
    #[inline(always)]
    fn from(val: u8) -> LpFlexcomm2 {
        LpFlexcomm2::from_bits(val)
    }
}
impl From<LpFlexcomm2> for u8 {
    #[inline(always)]
    fn from(val: LpFlexcomm2) -> u8 {
        LpFlexcomm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFlexcomm3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl LpFlexcomm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpFlexcomm3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpFlexcomm3 {
    #[inline(always)]
    fn from(val: u8) -> LpFlexcomm3 {
        LpFlexcomm3::from_bits(val)
    }
}
impl From<LpFlexcomm3> for u8 {
    #[inline(always)]
    fn from(val: LpFlexcomm3) -> u8 {
        LpFlexcomm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpcac {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcac {
    #[inline(always)]
    fn from(val: u8) -> Lpcac {
        Lpcac::from_bits(val)
    }
}
impl From<Lpcac> for u8 {
    #[inline(always)]
    fn from(val: Lpcac) -> u8 {
        Lpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lptmr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr0 {
    #[inline(always)]
    fn from(val: u8) -> Lptmr0 {
        Lptmr0::from_bits(val)
    }
}
impl From<Lptmr0> for u8 {
    #[inline(always)]
    fn from(val: Lptmr0) -> u8 {
        Lptmr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lptmr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr1 {
    #[inline(always)]
    fn from(val: u8) -> Lptmr1 {
        Lptmr1::from_bits(val)
    }
}
impl From<Lptmr1> for u8 {
    #[inline(always)]
    fn from(val: Lptmr1) -> u8 {
        Lptmr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mailbox {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mailbox {
    #[inline(always)]
    fn from(val: u8) -> Mailbox {
        Mailbox::from_bits(val)
    }
}
impl From<Mailbox> for u8 {
    #[inline(always)]
    fn from(val: Mailbox) -> u8 {
        Mailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEDma0 {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegEDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEDma0 {
        MasterSecAntiPolRegEDma0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEDma0) -> u8 {
        MasterSecAntiPolRegEDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEDma1 {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegEDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEDma1 {
        MasterSecAntiPolRegEDma1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEDma1) -> u8 {
        MasterSecAntiPolRegEDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPkc {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPkc {
        MasterSecAntiPolRegPkc::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPkc) -> u8 {
        MasterSecAntiPolRegPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSmartdma {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSmartdma {
        MasterSecAntiPolRegSmartdma::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSmartdma) -> u8 {
        MasterSecAntiPolRegSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsbHs {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsbHs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsbHs {
        MasterSecAntiPolRegUsbHs::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsbHs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsbHs) -> u8 {
        MasterSecAntiPolRegUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntipolLock {
    _RESERVED_0 = 0x0,
    #[doc = "MASTER_SEC_LEVEL_LOCK cannot be written."]
    Enable = 0x01,
    #[doc = "MASTER_SEC_LEVEL_LOCK can be written."]
    Disable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelAntipolLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntipolLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntipolLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntipolLock {
        MasterSecLevelAntipolLock::from_bits(val)
    }
}
impl From<MasterSecLevelAntipolLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntipolLock) -> u8 {
        MasterSecLevelAntipolLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEDma0 {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelEDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEDma0 {
        MasterSecLevelEDma0::from_bits(val)
    }
}
impl From<MasterSecLevelEDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEDma0) -> u8 {
        MasterSecLevelEDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEDma1 {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelEDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEDma1 {
        MasterSecLevelEDma1::from_bits(val)
    }
}
impl From<MasterSecLevelEDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEDma1) -> u8 {
        MasterSecLevelEDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelLock {
    _RESERVED_0 = 0x0,
    #[doc = "MASTER_SEC_LEVEL_LOCK cannot be written."]
    Enable = 0x01,
    #[doc = "MASTER_SEC_LEVEL_LOCK can be written."]
    Disable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelLock {
        MasterSecLevelLock::from_bits(val)
    }
}
impl From<MasterSecLevelLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelLock) -> u8 {
        MasterSecLevelLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPkc {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPkc {
        MasterSecLevelPkc::from_bits(val)
    }
}
impl From<MasterSecLevelPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPkc) -> u8 {
        MasterSecLevelPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSmartdma {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSmartdma {
        MasterSecLevelSmartdma::from_bits(val)
    }
}
impl From<MasterSecLevelSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSmartdma) -> u8 {
        MasterSecLevelSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsbHs {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsbHs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsbHs {
        MasterSecLevelUsbHs::from_bits(val)
    }
}
impl From<MasterSecLevelUsbHs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsbHs) -> u8 {
        MasterSecLevelUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mbc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc {
    #[inline(always)]
    fn from(val: u8) -> Mbc {
        Mbc::from_bits(val)
    }
}
impl From<Mbc> for u8 {
    #[inline(always)]
    fn from(val: Mbc) -> u8 {
        Mbc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Micd {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Micd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Micd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Micd {
    #[inline(always)]
    fn from(val: u8) -> Micd {
        Micd::from_bits(val)
    }
}
impl From<Micd> for u8 {
    #[inline(always)]
    fn from(val: Micd) -> u8 {
        Micd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        MiscCtrlDpRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableStrictMode) -> u8 {
        MiscCtrlDpRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        MiscCtrlDpRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableViolationAbort) -> u8 {
        MiscCtrlDpRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableNsPrivCheck) -> u8 {
        MiscCtrlDpRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSPrivCheck) -> u8 {
        MiscCtrlDpRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    Enabled = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        MiscCtrlDpRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSecureChecking) -> u8 {
        MiscCtrlDpRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegIdauAllNs {
        MiscCtrlDpRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlDpRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegIdauAllNs) -> u8 {
        MiscCtrlDpRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegWriteLock {
        MiscCtrlDpRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlDpRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegWriteLock) -> u8 {
        MiscCtrlDpRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master strict mode is on and can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master strict mode is disabled and can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableStrictMode {
        MiscCtrlRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableStrictMode) -> u8 {
        MiscCtrlRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables privilege checking of non-secure mode access is disabled."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        MiscCtrlRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableNsPrivCheck) -> u8 {
        MiscCtrlRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables privilege checking of secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        MiscCtrlRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSPrivCheck) -> u8 {
        MiscCtrlRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    Enabled = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSecureChecking {
        MiscCtrlRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSecureChecking) -> u8 {
        MiscCtrlRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mrt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0 {
    #[inline(always)]
    fn from(val: u8) -> Mrt0 {
        Mrt0::from_bits(val)
    }
}
impl From<Mrt0> for u8 {
    #[inline(always)]
    fn from(val: Mrt0) -> u8 {
        Mrt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mtr0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mtr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mtr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mtr0 {
    #[inline(always)]
    fn from(val: u8) -> Mtr0 {
        Mtr0::from_bits(val)
    }
}
impl From<Mtr0> for u8 {
    #[inline(always)]
    fn from(val: Mtr0) -> u8 {
        Mtr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Npu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npu {
    #[inline(always)]
    fn from(val: u8) -> Npu {
        Npu::from_bits(val)
    }
}
impl From<Npu> for u8 {
    #[inline(always)]
    fn from(val: Npu) -> u8 {
        Npu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npx {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npx {
    #[inline(always)]
    fn from(val: u8) -> Npx {
        Npx::from_bits(val)
    }
}
impl From<Npx> for u8 {
    #[inline(always)]
    fn from(val: Npx) -> u8 {
        Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp0 {
    #[inline(always)]
    fn from(val: u8) -> Opamp0 {
        Opamp0::from_bits(val)
    }
}
impl From<Opamp0> for u8 {
    #[inline(always)]
    fn from(val: Opamp0) -> u8 {
        Opamp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp1 {
    #[inline(always)]
    fn from(val: u8) -> Opamp1 {
        Opamp1::from_bits(val)
    }
}
impl From<Opamp1> for u8 {
    #[inline(always)]
    fn from(val: Opamp1) -> u8 {
        Opamp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp2 {
    #[inline(always)]
    fn from(val: u8) -> Opamp2 {
        Opamp2::from_bits(val)
    }
}
impl From<Opamp2> for u8 {
    #[inline(always)]
    fn from(val: Opamp2) -> u8 {
        Opamp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimer0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ostimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer0 {
    #[inline(always)]
    fn from(val: u8) -> Ostimer0 {
        Ostimer0::from_bits(val)
    }
}
impl From<Ostimer0> for u8 {
    #[inline(always)]
    fn from(val: Ostimer0) -> u8 {
        Ostimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Otpc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Otpc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Otpc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Otpc {
    #[inline(always)]
    fn from(val: u8) -> Otpc {
        Otpc::from_bits(val)
    }
}
impl From<Otpc> for u8 {
    #[inline(always)]
    fn from(val: Otpc) -> u8 {
        Otpc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pint0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pint0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pint0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pint0 {
    #[inline(always)]
    fn from(val: u8) -> Pint0 {
        Pint0::from_bits(val)
    }
}
impl From<Pint0> for u8 {
    #[inline(always)]
    fn from(val: Pint0) -> u8 {
        Pint0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin0SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin0SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin0SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin0SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin0SecMask {
        Pio0Pin0SecMask::from_bits(val)
    }
}
impl From<Pio0Pin0SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin0SecMask) -> u8 {
        Pio0Pin0SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin10SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin10SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin10SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin10SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin10SecMask {
        Pio0Pin10SecMask::from_bits(val)
    }
}
impl From<Pio0Pin10SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin10SecMask) -> u8 {
        Pio0Pin10SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin11SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin11SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin11SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin11SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin11SecMask {
        Pio0Pin11SecMask::from_bits(val)
    }
}
impl From<Pio0Pin11SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin11SecMask) -> u8 {
        Pio0Pin11SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin12SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin12SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin12SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin12SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin12SecMask {
        Pio0Pin12SecMask::from_bits(val)
    }
}
impl From<Pio0Pin12SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin12SecMask) -> u8 {
        Pio0Pin12SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin13SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin13SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin13SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin13SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin13SecMask {
        Pio0Pin13SecMask::from_bits(val)
    }
}
impl From<Pio0Pin13SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin13SecMask) -> u8 {
        Pio0Pin13SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin14SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin14SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin14SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin14SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin14SecMask {
        Pio0Pin14SecMask::from_bits(val)
    }
}
impl From<Pio0Pin14SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin14SecMask) -> u8 {
        Pio0Pin14SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin15SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin15SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin15SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin15SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin15SecMask {
        Pio0Pin15SecMask::from_bits(val)
    }
}
impl From<Pio0Pin15SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin15SecMask) -> u8 {
        Pio0Pin15SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin16SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin16SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin16SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin16SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin16SecMask {
        Pio0Pin16SecMask::from_bits(val)
    }
}
impl From<Pio0Pin16SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin16SecMask) -> u8 {
        Pio0Pin16SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin17SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin17SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin17SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin17SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin17SecMask {
        Pio0Pin17SecMask::from_bits(val)
    }
}
impl From<Pio0Pin17SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin17SecMask) -> u8 {
        Pio0Pin17SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin18SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin18SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin18SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin18SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin18SecMask {
        Pio0Pin18SecMask::from_bits(val)
    }
}
impl From<Pio0Pin18SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin18SecMask) -> u8 {
        Pio0Pin18SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin19SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin19SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin19SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin19SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin19SecMask {
        Pio0Pin19SecMask::from_bits(val)
    }
}
impl From<Pio0Pin19SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin19SecMask) -> u8 {
        Pio0Pin19SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin1SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin1SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin1SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin1SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin1SecMask {
        Pio0Pin1SecMask::from_bits(val)
    }
}
impl From<Pio0Pin1SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin1SecMask) -> u8 {
        Pio0Pin1SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin20SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin20SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin20SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin20SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin20SecMask {
        Pio0Pin20SecMask::from_bits(val)
    }
}
impl From<Pio0Pin20SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin20SecMask) -> u8 {
        Pio0Pin20SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin21SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin21SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin21SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin21SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin21SecMask {
        Pio0Pin21SecMask::from_bits(val)
    }
}
impl From<Pio0Pin21SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin21SecMask) -> u8 {
        Pio0Pin21SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin22SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin22SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin22SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin22SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin22SecMask {
        Pio0Pin22SecMask::from_bits(val)
    }
}
impl From<Pio0Pin22SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin22SecMask) -> u8 {
        Pio0Pin22SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin23SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin23SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin23SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin23SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin23SecMask {
        Pio0Pin23SecMask::from_bits(val)
    }
}
impl From<Pio0Pin23SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin23SecMask) -> u8 {
        Pio0Pin23SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin24SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin24SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin24SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin24SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin24SecMask {
        Pio0Pin24SecMask::from_bits(val)
    }
}
impl From<Pio0Pin24SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin24SecMask) -> u8 {
        Pio0Pin24SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin25SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin25SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin25SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin25SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin25SecMask {
        Pio0Pin25SecMask::from_bits(val)
    }
}
impl From<Pio0Pin25SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin25SecMask) -> u8 {
        Pio0Pin25SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin26SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin26SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin26SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin26SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin26SecMask {
        Pio0Pin26SecMask::from_bits(val)
    }
}
impl From<Pio0Pin26SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin26SecMask) -> u8 {
        Pio0Pin26SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin27SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin27SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin27SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin27SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin27SecMask {
        Pio0Pin27SecMask::from_bits(val)
    }
}
impl From<Pio0Pin27SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin27SecMask) -> u8 {
        Pio0Pin27SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin28SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin28SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin28SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin28SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin28SecMask {
        Pio0Pin28SecMask::from_bits(val)
    }
}
impl From<Pio0Pin28SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin28SecMask) -> u8 {
        Pio0Pin28SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin29SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin29SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin29SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin29SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin29SecMask {
        Pio0Pin29SecMask::from_bits(val)
    }
}
impl From<Pio0Pin29SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin29SecMask) -> u8 {
        Pio0Pin29SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin2SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin2SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin2SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin2SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin2SecMask {
        Pio0Pin2SecMask::from_bits(val)
    }
}
impl From<Pio0Pin2SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin2SecMask) -> u8 {
        Pio0Pin2SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin30SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin30SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin30SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin30SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin30SecMask {
        Pio0Pin30SecMask::from_bits(val)
    }
}
impl From<Pio0Pin30SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin30SecMask) -> u8 {
        Pio0Pin30SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin31SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin31SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin31SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin31SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin31SecMask {
        Pio0Pin31SecMask::from_bits(val)
    }
}
impl From<Pio0Pin31SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin31SecMask) -> u8 {
        Pio0Pin31SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin3SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin3SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin3SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin3SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin3SecMask {
        Pio0Pin3SecMask::from_bits(val)
    }
}
impl From<Pio0Pin3SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin3SecMask) -> u8 {
        Pio0Pin3SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin4SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin4SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin4SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin4SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin4SecMask {
        Pio0Pin4SecMask::from_bits(val)
    }
}
impl From<Pio0Pin4SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin4SecMask) -> u8 {
        Pio0Pin4SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin5SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin5SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin5SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin5SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin5SecMask {
        Pio0Pin5SecMask::from_bits(val)
    }
}
impl From<Pio0Pin5SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin5SecMask) -> u8 {
        Pio0Pin5SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin6SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin6SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin6SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin6SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin6SecMask {
        Pio0Pin6SecMask::from_bits(val)
    }
}
impl From<Pio0Pin6SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin6SecMask) -> u8 {
        Pio0Pin6SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin7SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin7SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin7SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin7SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin7SecMask {
        Pio0Pin7SecMask::from_bits(val)
    }
}
impl From<Pio0Pin7SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin7SecMask) -> u8 {
        Pio0Pin7SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin8SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin8SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin8SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin8SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin8SecMask {
        Pio0Pin8SecMask::from_bits(val)
    }
}
impl From<Pio0Pin8SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin8SecMask) -> u8 {
        Pio0Pin8SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin9SecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0Pin9SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin9SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin9SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin9SecMask {
        Pio0Pin9SecMask::from_bits(val)
    }
}
impl From<Pio0Pin9SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin9SecMask) -> u8 {
        Pio0Pin9SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRam {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PkcRam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRam {
    #[inline(always)]
    fn from(val: u8) -> PkcRam {
        PkcRam::from_bits(val)
    }
}
impl From<PkcRam> for u8 {
    #[inline(always)]
    fn from(val: PkcRam) -> u8 {
        PkcRam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Plu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plu {
    #[inline(always)]
    fn from(val: u8) -> Plu {
        Plu::from_bits(val)
    }
}
impl From<Plu> for u8 {
    #[inline(always)]
    fn from(val: Plu) -> u8 {
        Plu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0 {
    #[inline(always)]
    fn from(val: u8) -> Port0 {
        Port0::from_bits(val)
    }
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(val: Port0) -> u8 {
        Port0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1 {
    #[inline(always)]
    fn from(val: u8) -> Port1 {
        Port1::from_bits(val)
    }
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(val: Port1) -> u8 {
        Port1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2 {
    #[inline(always)]
    fn from(val: u8) -> Port2 {
        Port2::from_bits(val)
    }
}
impl From<Port2> for u8 {
    #[inline(always)]
    fn from(val: Port2) -> u8 {
        Port2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3 {
    #[inline(always)]
    fn from(val: u8) -> Port3 {
        Port3::from_bits(val)
    }
}
impl From<Port3> for u8 {
    #[inline(always)]
    fn from(val: Port3) -> u8 {
        Port3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4 {
    #[inline(always)]
    fn from(val: u8) -> Port4 {
        Port4::from_bits(val)
    }
}
impl From<Port4> for u8 {
    #[inline(always)]
    fn from(val: Port4) -> u8 {
        Port4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port5 {
    #[inline(always)]
    fn from(val: u8) -> Port5 {
        Port5::from_bits(val)
    }
}
impl From<Port5> for u8 {
    #[inline(always)]
    fn from(val: Port5) -> u8 {
        Port5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufAlias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PufAlias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufAlias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufAlias0 {
    #[inline(always)]
    fn from(val: u8) -> PufAlias0 {
        PufAlias0::from_bits(val)
    }
}
impl From<PufAlias0> for u8 {
    #[inline(always)]
    fn from(val: PufAlias0) -> u8 {
        PufAlias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufAlias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PufAlias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufAlias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufAlias1 {
    #[inline(always)]
    fn from(val: u8) -> PufAlias1 {
        PufAlias1::from_bits(val)
    }
}
impl From<PufAlias1> for u8 {
    #[inline(always)]
    fn from(val: PufAlias1) -> u8 {
        PufAlias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufAlias2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PufAlias2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufAlias2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufAlias2 {
    #[inline(always)]
    fn from(val: u8) -> PufAlias2 {
        PufAlias2::from_bits(val)
    }
}
impl From<PufAlias2> for u8 {
    #[inline(always)]
    fn from(val: PufAlias2) -> u8 {
        PufAlias2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufAlias3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PufAlias3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufAlias3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufAlias3 {
    #[inline(always)]
    fn from(val: u8) -> PufAlias3 {
        PufAlias3::from_bits(val)
    }
}
impl From<PufAlias3> for u8 {
    #[inline(always)]
    fn from(val: PufAlias3) -> u8 {
        PufAlias3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pwm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm {
    #[inline(always)]
    fn from(val: u8) -> Pwm {
        Pwm::from_bits(val)
    }
}
impl From<Pwm> for u8 {
    #[inline(always)]
    fn from(val: Pwm) -> u8 {
        Pwm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pwm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1 {
    #[inline(always)]
    fn from(val: u8) -> Pwm1 {
        Pwm1::from_bits(val)
    }
}
impl From<Pwm1> for u8 {
    #[inline(always)]
    fn from(val: Pwm1) -> u8 {
        Pwm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Qdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc {
    #[inline(always)]
    fn from(val: u8) -> Qdc {
        Qdc::from_bits(val)
    }
}
impl From<Qdc> for u8 {
    #[inline(always)]
    fn from(val: Qdc) -> u8 {
        Qdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Qdc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1 {
    #[inline(always)]
    fn from(val: u8) -> Qdc1 {
        Qdc1::from_bits(val)
    }
}
impl From<Qdc1> for u8 {
    #[inline(always)]
    fn from(val: Qdc1) -> u8 {
        Qdc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule0 {
        RamaMemRuleRule0::from_bits(val)
    }
}
impl From<RamaMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule0) -> u8 {
        RamaMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule1 {
        RamaMemRuleRule1::from_bits(val)
    }
}
impl From<RamaMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule1) -> u8 {
        RamaMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule2 {
        RamaMemRuleRule2::from_bits(val)
    }
}
impl From<RamaMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule2) -> u8 {
        RamaMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule3 {
        RamaMemRuleRule3::from_bits(val)
    }
}
impl From<RamaMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule3) -> u8 {
        RamaMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule4 {
        RamaMemRuleRule4::from_bits(val)
    }
}
impl From<RamaMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule4) -> u8 {
        RamaMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule5 {
        RamaMemRuleRule5::from_bits(val)
    }
}
impl From<RamaMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule5) -> u8 {
        RamaMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule6 {
        RamaMemRuleRule6::from_bits(val)
    }
}
impl From<RamaMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule6) -> u8 {
        RamaMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule7 {
        RamaMemRuleRule7::from_bits(val)
    }
}
impl From<RamaMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule7) -> u8 {
        RamaMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule0 {
        RambMemRuleRule0::from_bits(val)
    }
}
impl From<RambMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule0) -> u8 {
        RambMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule1 {
        RambMemRuleRule1::from_bits(val)
    }
}
impl From<RambMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule1) -> u8 {
        RambMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule2 {
        RambMemRuleRule2::from_bits(val)
    }
}
impl From<RambMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule2) -> u8 {
        RambMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule3 {
        RambMemRuleRule3::from_bits(val)
    }
}
impl From<RambMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule3) -> u8 {
        RambMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule4 {
        RambMemRuleRule4::from_bits(val)
    }
}
impl From<RambMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule4) -> u8 {
        RambMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule5 {
        RambMemRuleRule5::from_bits(val)
    }
}
impl From<RambMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule5) -> u8 {
        RambMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule6 {
        RambMemRuleRule6::from_bits(val)
    }
}
impl From<RambMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule6) -> u8 {
        RambMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule7 {
        RambMemRuleRule7::from_bits(val)
    }
}
impl From<RambMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule7) -> u8 {
        RambMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule0 {
        RamcMemRuleRule0::from_bits(val)
    }
}
impl From<RamcMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule0) -> u8 {
        RamcMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule1 {
        RamcMemRuleRule1::from_bits(val)
    }
}
impl From<RamcMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule1) -> u8 {
        RamcMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule2 {
        RamcMemRuleRule2::from_bits(val)
    }
}
impl From<RamcMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule2) -> u8 {
        RamcMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule3 {
        RamcMemRuleRule3::from_bits(val)
    }
}
impl From<RamcMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule3) -> u8 {
        RamcMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule4 {
        RamcMemRuleRule4::from_bits(val)
    }
}
impl From<RamcMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule4) -> u8 {
        RamcMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule5 {
        RamcMemRuleRule5::from_bits(val)
    }
}
impl From<RamcMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule5) -> u8 {
        RamcMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule6 {
        RamcMemRuleRule6::from_bits(val)
    }
}
impl From<RamcMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule6) -> u8 {
        RamcMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule7 {
        RamcMemRuleRule7::from_bits(val)
    }
}
impl From<RamcMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule7) -> u8 {
        RamcMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule0 {
        RamdMemRuleRule0::from_bits(val)
    }
}
impl From<RamdMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule0) -> u8 {
        RamdMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule1 {
        RamdMemRuleRule1::from_bits(val)
    }
}
impl From<RamdMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule1) -> u8 {
        RamdMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule2 {
        RamdMemRuleRule2::from_bits(val)
    }
}
impl From<RamdMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule2) -> u8 {
        RamdMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule3 {
        RamdMemRuleRule3::from_bits(val)
    }
}
impl From<RamdMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule3) -> u8 {
        RamdMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule4 {
        RamdMemRuleRule4::from_bits(val)
    }
}
impl From<RamdMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule4) -> u8 {
        RamdMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule5 {
        RamdMemRuleRule5::from_bits(val)
    }
}
impl From<RamdMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule5) -> u8 {
        RamdMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule6 {
        RamdMemRuleRule6::from_bits(val)
    }
}
impl From<RamdMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule6) -> u8 {
        RamdMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule7 {
        RamdMemRuleRule7::from_bits(val)
    }
}
impl From<RamdMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule7) -> u8 {
        RamdMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule0 {
        RameMemRuleRule0::from_bits(val)
    }
}
impl From<RameMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule0) -> u8 {
        RameMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule1 {
        RameMemRuleRule1::from_bits(val)
    }
}
impl From<RameMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule1) -> u8 {
        RameMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule2 {
        RameMemRuleRule2::from_bits(val)
    }
}
impl From<RameMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule2) -> u8 {
        RameMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule3 {
        RameMemRuleRule3::from_bits(val)
    }
}
impl From<RameMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule3) -> u8 {
        RameMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule4 {
        RameMemRuleRule4::from_bits(val)
    }
}
impl From<RameMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule4) -> u8 {
        RameMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule5 {
        RameMemRuleRule5::from_bits(val)
    }
}
impl From<RameMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule5) -> u8 {
        RameMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule6 {
        RameMemRuleRule6::from_bits(val)
    }
}
impl From<RameMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule6) -> u8 {
        RameMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule7 {
        RameMemRuleRule7::from_bits(val)
    }
}
impl From<RameMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule7) -> u8 {
        RameMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule0 {
        RamxMemRuleRule0::from_bits(val)
    }
}
impl From<RamxMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule0) -> u8 {
        RamxMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule1 {
        RamxMemRuleRule1::from_bits(val)
    }
}
impl From<RamxMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule1) -> u8 {
        RamxMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule2 {
        RamxMemRuleRule2::from_bits(val)
    }
}
impl From<RamxMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule2) -> u8 {
        RamxMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule3 {
        RamxMemRuleRule3::from_bits(val)
    }
}
impl From<RamxMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule3) -> u8 {
        RamxMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule4 {
        RamxMemRuleRule4::from_bits(val)
    }
}
impl From<RamxMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule4) -> u8 {
        RamxMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule5 {
        RamxMemRuleRule5::from_bits(val)
    }
}
impl From<RamxMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule5) -> u8 {
        RamxMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule6 {
        RamxMemRuleRule6::from_bits(val)
    }
}
impl From<RamxMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule6) -> u8 {
        RamxMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule7 {
        RamxMemRuleRule7::from_bits(val)
    }
}
impl From<RamxMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule7) -> u8 {
        RamxMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule0 {
        RomMemRuleRule0::from_bits(val)
    }
}
impl From<RomMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule0) -> u8 {
        RomMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule1 {
        RomMemRuleRule1::from_bits(val)
    }
}
impl From<RomMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule1) -> u8 {
        RomMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule2 {
        RomMemRuleRule2::from_bits(val)
    }
}
impl From<RomMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule2) -> u8 {
        RomMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule3 {
        RomMemRuleRule3::from_bits(val)
    }
}
impl From<RomMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule3) -> u8 {
        RomMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule4 {
        RomMemRuleRule4::from_bits(val)
    }
}
impl From<RomMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule4) -> u8 {
        RomMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule5 {
        RomMemRuleRule5::from_bits(val)
    }
}
impl From<RomMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule5) -> u8 {
        RomMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule6 {
        RomMemRuleRule6::from_bits(val)
    }
}
impl From<RomMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule6) -> u8 {
        RomMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule7 {
        RomMemRuleRule7::from_bits(val)
    }
}
impl From<RomMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule7) -> u8 {
        RomMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sai0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0 {
    #[inline(always)]
    fn from(val: u8) -> Sai0 {
        Sai0::from_bits(val)
    }
}
impl From<Sai0> for u8 {
    #[inline(always)]
    fn from(val: Sai0) -> u8 {
        Sai0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sai1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1 {
    #[inline(always)]
    fn from(val: u8) -> Sai1 {
        Sai1::from_bits(val)
    }
}
impl From<Sai1> for u8 {
    #[inline(always)]
    fn from(val: Sai1) -> u8 {
        Sai1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scg0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Scg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scg0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scg0 {
    #[inline(always)]
    fn from(val: u8) -> Scg0 {
        Scg0::from_bits(val)
    }
}
impl From<Scg0> for u8 {
    #[inline(always)]
    fn from(val: Scg0) -> u8 {
        Scg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sct0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct0 {
    #[inline(always)]
    fn from(val: u8) -> Sct0 {
        Sct0::from_bits(val)
    }
}
impl From<Sct0> for u8 {
    #[inline(always)]
    fn from(val: Sct0) -> u8 {
        Sct0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code."]
    Code = 0x0,
    #[doc = "Data."]
    Data = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "M33 Code."]
    Cpu0Code = 0x0,
    #[doc = "M33 System."]
    Cpu0Sys = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SMARTDMA Instruction."]
    SdmaInstr = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SMARTDMA Data."]
    SdmaData = 0x05,
    #[doc = "eDMA0."]
    EDma0 = 0x06,
    #[doc = "eDMA1."]
    EDma1 = 0x07,
    #[doc = "PKC."]
    Pkc = 0x08,
    #[doc = "ELS S50."]
    Cssv2 = 0x09,
    #[doc = "PKC M0."]
    Pq = 0x0a,
    #[doc = "NPU Operands."]
    Npuo = 0x0b,
    #[doc = "DSP Instruction."]
    Dspi = 0x0c,
    #[doc = "DSPX."]
    Dspx = 0x0d,
    #[doc = "DSPY."]
    Dspy = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "NPU Data."]
    Npud = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "Ethernet."]
    Ethernet = 0x12,
    #[doc = "USB HS."]
    UsbHs = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access."]
    Read = 0x0,
    #[doc = "Write access."]
    Write = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sema42 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sema42 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sema42 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sema42 {
    #[inline(always)]
    fn from(val: u8) -> Sema42 {
        Sema42::from_bits(val)
    }
}
impl From<Sema42> for u8 {
    #[inline(always)]
    fn from(val: Sema42) -> u8 {
        Sema42::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sfa {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sfa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sfa {
    #[inline(always)]
    fn from(val: u8) -> Sfa {
        Sfa::from_bits(val)
    }
}
impl From<Sfa> for u8 {
    #[inline(always)]
    fn from(val: Sfa) -> u8 {
        Sfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sinc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc0 {
    #[inline(always)]
    fn from(val: u8) -> Sinc0 {
        Sinc0::from_bits(val)
    }
}
impl From<Sinc0> for u8 {
    #[inline(always)]
    fn from(val: Sinc0) -> u8 {
        Sinc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Spc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spc0 {
    #[inline(always)]
    fn from(val: u8) -> Spc0 {
        Spc0::from_bits(val)
    }
}
impl From<Spc0> for u8 {
    #[inline(always)]
    fn from(val: Spc0) -> u8 {
        Spc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syscon {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Syscon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscon {
    #[inline(always)]
    fn from(val: u8) -> Syscon {
        Syscon::from_bits(val)
    }
}
impl From<Syscon> for u8 {
    #[inline(always)]
    fn from(val: Syscon) -> u8 {
        Syscon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trng {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Trng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trng {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trng {
    #[inline(always)]
    fn from(val: u8) -> Trng {
        Trng::from_bits(val)
    }
}
impl From<Trng> for u8 {
    #[inline(always)]
    fn from(val: Trng) -> u8 {
        Trng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Tsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi {
    #[inline(always)]
    fn from(val: u8) -> Tsi {
        Tsi::from_bits(val)
    }
}
impl From<Tsi> for u8 {
    #[inline(always)]
    fn from(val: Tsi) -> u8 {
        Tsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl USdhc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhc0 {
    #[inline(always)]
    fn from(val: u8) -> USdhc0 {
        USdhc0::from_bits(val)
    }
}
impl From<USdhc0> for u8 {
    #[inline(always)]
    fn from(val: USdhc0) -> u8 {
        USdhc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbFsOtgRam {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl UsbFsOtgRam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbFsOtgRam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbFsOtgRam {
    #[inline(always)]
    fn from(val: u8) -> UsbFsOtgRam {
        UsbFsOtgRam::from_bits(val)
    }
}
impl From<UsbFsOtgRam> for u8 {
    #[inline(always)]
    fn from(val: UsbFsOtgRam) -> u8 {
        UsbFsOtgRam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbdcd {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbdcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbdcd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbdcd {
    #[inline(always)]
    fn from(val: u8) -> Usbdcd {
        Usbdcd::from_bits(val)
    }
}
impl From<Usbdcd> for u8 {
    #[inline(always)]
    fn from(val: Usbdcd) -> u8 {
        Usbdcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbfs {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbfs {
    #[inline(always)]
    fn from(val: u8) -> Usbfs {
        Usbfs::from_bits(val)
    }
}
impl From<Usbfs> for u8 {
    #[inline(always)]
    fn from(val: Usbfs) -> u8 {
        Usbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhs {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbhs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhs {
    #[inline(always)]
    fn from(val: u8) -> Usbhs {
        Usbhs::from_bits(val)
    }
}
impl From<Usbhs> for u8 {
    #[inline(always)]
    fn from(val: Usbhs) -> u8 {
        Usbhs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhsphy {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbhsphy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhsphy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhsphy {
    #[inline(always)]
    fn from(val: u8) -> Usbhsphy {
        Usbhsphy::from_bits(val)
    }
}
impl From<Usbhsphy> for u8 {
    #[inline(always)]
    fn from(val: Usbhsphy) -> u8 {
        Usbhsphy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utcik0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Utcik0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utcik0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utcik0 {
    #[inline(always)]
    fn from(val: u8) -> Utcik0 {
        Utcik0::from_bits(val)
    }
}
impl From<Utcik0> for u8 {
    #[inline(always)]
    fn from(val: Utcik0) -> u8 {
        Utcik0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbat {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Vbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbat {
    #[inline(always)]
    fn from(val: u8) -> Vbat {
        Vbat::from_bits(val)
    }
}
impl From<Vbat> for u8 {
    #[inline(always)]
    fn from(val: Vbat) -> u8 {
        Vbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Vref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref {
    #[inline(always)]
    fn from(val: u8) -> Vref {
        Vref::from_bits(val)
    }
}
impl From<Vref> for u8 {
    #[inline(always)]
    fn from(val: Vref) -> u8 {
        Vref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuu0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wuu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuu0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuu0 {
    #[inline(always)]
    fn from(val: u8) -> Wuu0 {
        Wuu0::from_bits(val)
    }
}
impl From<Wuu0> for u8 {
    #[inline(always)]
    fn from(val: Wuu0) -> u8 {
        Wuu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt0 {
        Wwdt0::from_bits(val)
    }
}
impl From<Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Wwdt0) -> u8 {
        Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1 {
        Wwdt1::from_bits(val)
    }
}
impl From<Wwdt1> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1) -> u8 {
        Wwdt1::to_bits(val)
    }
}
