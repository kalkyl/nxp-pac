#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "ADC is idle. There are no pending triggers to service and no active commands are being processed."]
    NotActive = 0x0,
    #[doc = "ADC is processing a conversion, running through the power-up delay, or servicing a trigger."]
    Busy = 0x01,
}
impl AdcActive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcActive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcActive {
    #[inline(always)]
    fn from(val: u8) -> AdcActive {
        AdcActive::from_bits(val)
    }
}
impl From<AdcActive> for u8 {
    #[inline(always)]
    fn from(val: AdcActive) -> u8 {
        AdcActive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2."]
    Average2 = 0x01,
    #[doc = "4."]
    Average4 = 0x02,
    #[doc = "8."]
    Average8 = 0x03,
    #[doc = "16."]
    Average16 = 0x04,
    #[doc = "32."]
    Average32 = 0x05,
    #[doc = "64."]
    Average64 = 0x06,
    #[doc = "128."]
    Average128 = 0x07,
    #[doc = "256."]
    Average256 = 0x08,
    #[doc = "512."]
    Average512 = 0x09,
    #[doc = "1024."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avgs {
    #[inline(always)]
    fn from(val: u8) -> Avgs {
        Avgs::from_bits(val)
    }
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(val: Avgs) -> u8 {
        Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalAvgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CalAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalAvgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalAvgs {
    #[inline(always)]
    fn from(val: u8) -> CalAvgs {
        CalAvgs::from_bits(val)
    }
}
impl From<CalAvgs> for u8 {
    #[inline(always)]
    fn from(val: CalAvgs) -> u8 {
        CalAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalRdy {
    #[doc = "Calibration is incomplete or has not been run."]
    NotSet = 0x0,
    #[doc = "ADC is calibrated."]
    HardwareCalStepCompleted = 0x01,
}
impl CalRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalRdy {
    #[inline(always)]
    fn from(val: u8) -> CalRdy {
        CalRdy::from_bits(val)
    }
}
impl From<CalRdy> for u8 {
    #[inline(always)]
    fn from(val: CalRdy) -> u8 {
        CalRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalReq {
    #[doc = "No request made."]
    NoCalibrationRequest = 0x0,
    #[doc = "Request has been made."]
    CalibrationRequestPending = 0x01,
}
impl CalReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalReq {
    #[inline(always)]
    fn from(val: u8) -> CalReq {
        CalReq::from_bits(val)
    }
}
impl From<CalReq> for u8 {
    #[inline(always)]
    fn from(val: CalReq) -> u8 {
        CalReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofs {
    #[doc = "Calibration function disabled."]
    NoActiveOffsetCalibrationRequest = 0x0,
    #[doc = "Request for offset calibration function."]
    OffsetCalibrationRequestPending = 0x01,
}
impl Calofs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofs {
    #[inline(always)]
    fn from(val: u8) -> Calofs {
        Calofs::from_bits(val)
    }
}
impl From<Calofs> for u8 {
    #[inline(always)]
    fn from(val: Calofs) -> u8 {
        Calofs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsi {
    #[doc = "Not implemented."]
    CalFunctionNotAvailable = 0x0,
    #[doc = "Implemented."]
    CalFunctionAvailable = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdact {
    #[doc = "No command currently in progress."]
    NoCommandActive = 0x0,
    #[doc = "Command 1 currently being executed."]
    Command1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    Command2 = 0x02,
    #[doc = "Associated command number currently being executed."]
    CommandX3 = 0x03,
    #[doc = "Associated command number currently being executed."]
    CommandX4 = 0x04,
    #[doc = "Associated command number currently being executed."]
    CommandX5 = 0x05,
    #[doc = "Associated command number currently being executed."]
    CommandX6 = 0x06,
    #[doc = "Associated command number currently being executed."]
    CommandX7 = 0x07,
    #[doc = "Associated command number currently being executed."]
    CommandX8 = 0x08,
    #[doc = "Associated command number currently being executed."]
    CommandX9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a data word in RESFIFO. 0h is only found in the initial FIFO state, prior to the storage of an ADC conversion result into a RESFIFO buffer."]
    NotValid = 0x0,
    #[doc = "CMD1."]
    Cmd1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    Cmd15 = 0x0f,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpen {
    #[doc = "Disabled."]
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
}
impl Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmpen {
        Cmpen::from_bits(val)
    }
}
impl From<Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmpen) -> u8 {
        Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
    #[doc = "Not supported."]
    CscaleNotSupported = 0x0,
    #[doc = "Supported with one-bit CSCALE control field."]
    BitWidth1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Supported with six-bit CSCALE control field."]
    BitWidth6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctype {
    #[doc = "Single-Ended mode. Only A-side channel is converted."]
    SingleEndedASideChannel = 0x0,
    #[doc = "Single-Ended mode. Only B-side channel is converted."]
    SingleEndedBSideChannel = 0x01,
    #[doc = "Differential mode. A-B."]
    DifferentialAMinusB = 0x02,
    #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
    DualAAndB = 0x03,
}
impl Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctype {
    #[inline(always)]
    fn from(val: u8) -> Ctype {
        Ctype::from_bits(val)
    }
}
impl From<Ctype> for u8 {
    #[inline(always)]
    fn from(val: Ctype) -> u8 {
        Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Not supported."]
    DifferentialNotSupported = 0x0,
    #[doc = "Supported. CMDLn\\[CTYPE\\] controls fields implemented."]
    DifferentialSupported = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "ADC is enabled in low-power mode."]
    Enabled = 0x0,
    #[doc = "ADC is disabled in low-power mode."]
    Disabled = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelA {
    #[doc = "FIFO 0."]
    StoreToFifo0 = 0x0,
    #[doc = "FIFO 1."]
    StoreToFifo1 = 0x01,
}
impl FifoSelA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelA {
    #[inline(always)]
    fn from(val: u8) -> FifoSelA {
        FifoSelA::from_bits(val)
    }
}
impl From<FifoSelA> for u8 {
    #[inline(always)]
    fn from(val: FifoSelA) -> u8 {
        FifoSelA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelB {
    #[doc = "FIFO 0."]
    StoreToFifo0 = 0x0,
    #[doc = "FIFO 1."]
    StoreToFifo1 = 0x01,
}
impl FifoSelB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelB {
    #[inline(always)]
    fn from(val: u8) -> FifoSelB {
        FifoSelB::from_bits(val)
    }
}
impl From<FifoSelB> for u8 {
    #[inline(always)]
    fn from(val: FifoSelB) -> u8 {
        FifoSelB::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fifosize(u8);
impl Fifosize {
    #[doc = "2."]
    pub const Entries2: Self = Self(0x01);
    #[doc = "4."]
    pub const Entries4: Self = Self(0x04);
    #[doc = "8."]
    pub const Entries8: Self = Self(0x08);
    #[doc = "16."]
    pub const Entries16: Self = Self(0x10);
    #[doc = "32."]
    pub const Entries32: Self = Self(0x20);
    #[doc = "64."]
    pub const Entries64: Self = Self(0x40);
}
impl Fifosize {
    pub const fn from_bits(val: u8) -> Fifosize {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fifosize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Entries2"),
            0x04 => f.write_str("Entries4"),
            0x08 => f.write_str("Entries8"),
            0x10 => f.write_str("Entries16"),
            0x20 => f.write_str("Entries32"),
            0x40 => f.write_str("Entries64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Entries2"),
            0x04 => defmt::write!(f, "Entries4"),
            0x08 => defmt::write!(f, "Entries8"),
            0x10 => defmt::write!(f, "Entries16"),
            0x20 => defmt::write!(f, "Entries32"),
            0x40 => defmt::write!(f, "Entries64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof0 {
    #[doc = "No result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    NoOverflow = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    OverflowDetected = 0x01,
}
impl Fof0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof0 {
    #[inline(always)]
    fn from(val: u8) -> Fof0 {
        Fof0::from_bits(val)
    }
}
impl From<Fof0> for u8 {
    #[inline(always)]
    fn from(val: Fof0) -> u8 {
        Fof0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    NoOverflow = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    OverflowDetected = 0x01,
}
impl Fof1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof1 {
    #[inline(always)]
    fn from(val: u8) -> Fof1 {
        Fof1::from_bits(val)
    }
}
impl From<Fof1> for u8 {
    #[inline(always)]
    fn from(val: Fof1) -> u8 {
        Fof1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GccRdy {
    #[doc = "Invalid."]
    GainCalNotValid = 0x0,
    #[doc = "Valid."]
    HardwareCalRoutineCompleted = 0x01,
}
impl GccRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GccRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GccRdy {
    #[inline(always)]
    fn from(val: u8) -> GccRdy {
        GccRdy::from_bits(val)
    }
}
impl From<GccRdy> for u8 {
    #[inline(always)]
    fn from(val: GccRdy) -> u8 {
        GccRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HptExdi {
    #[doc = "Enabled."]
    Enabled = 0x0,
    #[doc = "Disabled."]
    Disabled = 0x01,
}
impl HptExdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HptExdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HptExdi {
    #[inline(always)]
    fn from(val: u8) -> HptExdi {
        HptExdi::from_bits(val)
    }
}
impl From<HptExdi> for u8 {
    #[inline(always)]
    fn from(val: HptExdi) -> u8 {
        HptExdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Not implemented."]
    InternalClkNotAvailable = 0x0,
    #[doc = "Implemented."]
    InternalClkAvailable = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loop {
    #[doc = "Looping not enabled. Command executes one time."]
    CmdExec1x = 0x0,
    #[doc = "Loop one time. Command executes two times."]
    CmdExec2x = 0x01,
    #[doc = "Loop two times. Command executes three times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
}
impl Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loop {
    #[inline(always)]
    fn from(val: u8) -> Loop {
        Loop::from_bits(val)
    }
}
impl From<Loop> for u8 {
    #[inline(always)]
    fn from(val: Loop) -> u8 {
        Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    Result1 = 0x0,
    #[doc = "Result is from second conversion in command."]
    Result2 = 0x01,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult2 = 0x02,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult3 = 0x03,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult4 = 0x04,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult5 = 0x05,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult6 = 0x06,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult7 = 0x07,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult8 = 0x08,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    Result16 = 0x0f,
}
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2's complement output."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2's complement output."]
    Data16Bits = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single VREFH input supported."]
    MultipleRefNotSupported = 0x0,
    #[doc = "Multiple VREFH inputs supported."]
    MultipleRefSupported = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "CMD1."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    DoCmd15Next = 0x0f,
}
impl Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Next {
    #[inline(always)]
    fn from(val: u8) -> Next {
        Next::from_bits(val)
    }
}
impl From<Next> for u8 {
    #[inline(always)]
    fn from(val: Next) -> u8 {
        Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumFifo {
    #[doc = "N/A."]
    NoFifoImplemented = 0x0,
    #[doc = "One."]
    Cnt1 = 0x01,
    #[doc = "Two."]
    Cnt2 = 0x02,
    #[doc = "Three."]
    Cnt3 = 0x03,
    #[doc = "Four."]
    Cnt4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl NumFifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumFifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumFifo {
    #[inline(always)]
    fn from(val: u8) -> NumFifo {
        NumFifo::from_bits(val)
    }
}
impl From<NumFifo> for u8 {
    #[inline(always)]
    fn from(val: NumFifo) -> u8 {
        NumFifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumSec {
    #[doc = "One."]
    SingleConvertor = 0x0,
    #[doc = "Two."]
    DualConvertor = 0x01,
}
impl NumSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumSec {
    #[inline(always)]
    fn from(val: u8) -> NumSec {
        NumSec::from_bits(val)
    }
}
impl From<NumSec> for u8 {
    #[inline(always)]
    fn from(val: NumSec) -> u8 {
        NumSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrsel {
    #[doc = "Low power."]
    Lowest = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "High power."]
    Highest = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy0 {
    #[doc = "Not above watermark."]
    BelowThreshold = 0x0,
    #[doc = "Above watermark."]
    AboveThreshold = 0x01,
}
impl Rdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy0 {
    #[inline(always)]
    fn from(val: u8) -> Rdy0 {
        Rdy0::from_bits(val)
    }
}
impl From<Rdy0> for u8 {
    #[inline(always)]
    fn from(val: Rdy0) -> u8 {
        Rdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy1 {
    #[doc = "Not above watermark."]
    BelowThreshold = 0x0,
    #[doc = "Above watermark."]
    AboveThreshold = 0x01,
}
impl Rdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy1 {
    #[inline(always)]
    fn from(val: u8) -> Rdy1 {
        Rdy1::from_bits(val)
    }
}
impl From<Rdy1> for u8 {
    #[inline(always)]
    fn from(val: Rdy1) -> u8 {
        Rdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "Option 1."]
    Option1 = 0x0,
    #[doc = "Option 2."]
    Option2 = 0x01,
    #[doc = "Option 3."]
    Option3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res {
    #[doc = "Up to 13-bit differential or 12-bit single-ended resolution supported."]
    Max13Bit = 0x0,
    #[doc = "Up to 16-bit differential or 16-bit single-ended resolution supported. CMDLn\\[MODE\\] available for selecting the resolution of conversions for the associated command."]
    Max16Bit = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    ReleasedFromReset = 0x0,
    #[doc = "ADC logic is reset."]
    HeldInReset = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo0 {
    #[doc = "No effect."]
    NoAction = 0x0,
    #[doc = "FIFO 0 is reset."]
    TriggerReset = 0x01,
}
impl Rstfifo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo0 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo0 {
        Rstfifo0::from_bits(val)
    }
}
impl From<Rstfifo0> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo0) -> u8 {
        Rstfifo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo1 {
    #[doc = "No effect."]
    NoAction = 0x0,
    #[doc = "FIFO 1 is reset."]
    TriggerReset = 0x01,
}
impl Rstfifo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo1 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo1 {
        Rstfifo1::from_bits(val)
    }
}
impl From<Rstfifo1> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo1) -> u8 {
        Rstfifo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "5.5 ADCK cycles."]
    Sample5p5 = 0x01,
    #[doc = "7.5 ADCK cycles."]
    Sample7p5 = 0x02,
    #[doc = "11.5 ADCK cycles."]
    Sample11p5 = 0x03,
    #[doc = "19.5 ADCK cycles."]
    Sample19p5 = 0x04,
    #[doc = "35.5 ADCK cycles."]
    Sample35p5 = 0x05,
    #[doc = "67.5 ADCK cycles."]
    Sample67p5 = 0x06,
    #[doc = "131.5 ADCK cycles."]
    Sample131p5 = 0x07,
}
impl Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts {
    #[inline(always)]
    fn from(val: u8) -> Sts {
        Sts::from_bits(val)
    }
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(val: Sts) -> u8 {
        Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    NotValid = 0x0,
    #[doc = "CMD1."]
    ExecuteCmd1 = 0x01,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd2 = 0x02,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd3 = 0x03,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd4 = 0x04,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd5 = 0x05,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd6 = 0x06,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd7 = 0x07,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd8 = 0x08,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    ExecuteCmd15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompFlag {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    NoTrigger = 0x0,
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    Bit0MeansTrigger0Completed = 0x01,
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    Bit1MeansTrigger1Completed = 0x02,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted3 = 0x03,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted4 = 0x04,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted5 = 0x05,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted6 = 0x06,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted7 = 0x07,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted8 = 0x08,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    AllBitsSetIndicateAllTriggersCompleted = 0x0f,
}
impl TcompFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompFlag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompFlag {
    #[inline(always)]
    fn from(val: u8) -> TcompFlag {
        TcompFlag::from_bits(val)
    }
}
impl From<TcompFlag> for u8 {
    #[inline(always)]
    fn from(val: TcompFlag) -> u8 {
        TcompFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompIe {
    #[doc = "All disabled."]
    Disabled = 0x0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    Trigger0CompleteEnabled = 0x01,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    Trigger1CompleteEnabled = 0x02,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled3 = 0x03,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled4 = 0x04,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled5 = 0x05,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled6 = 0x06,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled7 = 0x07,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled8 = 0x08,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "All enabled."]
    AllTriggerCompletesEnabled = 0x0f,
}
impl TcompIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompIe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompIe {
    #[inline(always)]
    fn from(val: u8) -> TcompIe {
        TcompIe::from_bits(val)
    }
}
impl From<TcompIe> for u8 {
    #[inline(always)]
    fn from(val: TcompIe) -> u8 {
        TcompIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompInt {
    #[doc = "Either IE\\[TCOMP_IE\\] = 0, or no trigger sequences have run to completion."]
    FlagClear = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    CompletionDetected = 0x01,
}
impl TcompInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompInt {
    #[inline(always)]
    fn from(val: u8) -> TcompInt {
        TcompInt::from_bits(val)
    }
}
impl From<TcompInt> for u8 {
    #[inline(always)]
    fn from(val: TcompInt) -> u8 {
        TcompInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcInt {
    #[doc = "No trigger exceptions have occurred."]
    NoException = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgment."]
    ExceptionDetected = 0x01,
}
impl TexcInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcInt {
    #[inline(always)]
    fn from(val: u8) -> TexcInt {
        TexcInt::from_bits(val)
    }
}
impl From<TexcInt> for u8 {
    #[inline(always)]
    fn from(val: TexcInt) -> u8 {
        TexcInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcNum {
    #[doc = "No triggers have been interrupted by a high-priority exception."]
    NoExceptions = 0x0,
    #[doc = "Trigger 0 has been interrupted by a high-priority exception."]
    Bit0MeansTrigger0Interrupted = 0x01,
    #[doc = "Trigger 1 has been interrupted by a high-priority exception."]
    Bit1MeansTrigger1Interrupted = 0x02,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted3 = 0x03,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted4 = 0x04,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted5 = 0x05,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted6 = 0x06,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted7 = 0x07,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted8 = 0x08,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been interrupted by a high-priority exception."]
    AllBitsSetIndicateAllTriggersInterrupted = 0x0f,
}
impl TexcNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcNum {
    #[inline(always)]
    fn from(val: u8) -> TexcNum {
        TexcNum::from_bits(val)
    }
}
impl From<TexcNum> for u8 {
    #[inline(always)]
    fn from(val: TexcNum) -> u8 {
        TexcNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Highest priority, Level 1."]
    HighestPriority = 0x0,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority1 = 0x01,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority2 = 0x02,
    #[doc = "Lowest priority, Level 4."]
    LowestPriority = 0x03,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tprictrl {
    #[doc = "Current conversion is aborted and the new command specified by the trigger is started."]
    AbortCurrentOnPriority = 0x0,
    #[doc = "Current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop is completed. CMDHn\\[LOOP\\] is ignored and the higher-priority trigger is serviced."]
    FinishCurrentOnPriority = 0x01,
    #[doc = "Current command is completed (averaging, looping, compare) before servicing the higher-priority trigger."]
    FinishSequenceOnPriority = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    Trig0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    Trig1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    Trig2 = 0x02,
    #[doc = "Command (sequence) associated with Trigger 3 currently being executed."]
    Trig3 = 0x03,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
    #[doc = "Trigger source 0."]
    Trigger0 = 0x0,
    #[doc = "Trigger source 1."]
    Trigger1 = 0x01,
    #[doc = "Trigger source 2."]
    Trigger2 = 0x02,
    #[doc = "Trigger source 3."]
    Trigger3 = 0x03,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vr1rngi {
    #[doc = "Range control not required."]
    Ref1FixedVoltageRange = 0x0,
    #[doc = "Range control required."]
    Ref1SelectableVoltageRange = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
