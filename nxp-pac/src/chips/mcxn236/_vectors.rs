unsafe extern "C" {
    fn OR();
    fn EDMA_0_CH0();
    fn EDMA_0_CH1();
    fn EDMA_0_CH2();
    fn EDMA_0_CH3();
    fn EDMA_0_CH4();
    fn EDMA_0_CH5();
    fn EDMA_0_CH6();
    fn EDMA_0_CH7();
    fn EDMA_0_CH8();
    fn EDMA_0_CH9();
    fn EDMA_0_CH10();
    fn EDMA_0_CH11();
    fn EDMA_0_CH12();
    fn EDMA_0_CH13();
    fn EDMA_0_CH14();
    fn EDMA_0_CH15();
    fn GPIO00();
    fn GPIO01();
    fn GPIO10();
    fn GPIO11();
    fn GPIO20();
    fn GPIO21();
    fn GPIO30();
    fn GPIO31();
    fn GPIO40();
    fn GPIO41();
    fn GPIO50();
    fn GPIO51();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn RESERVED49();
    fn CTIMER2();
    fn LP_FLEXCOMM0();
    fn LP_FLEXCOMM1();
    fn LP_FLEXCOMM2();
    fn LP_FLEXCOMM3();
    fn LP_FLEXCOMM4();
    fn LP_FLEXCOMM5();
    fn LP_FLEXCOMM6();
    fn LP_FLEXCOMM7();
    fn RESERVED59();
    fn RESERVED60();
    fn ADC0();
    fn ADC1();
    fn PINT0();
    fn PDM_EVENT();
    fn RESERVED65();
    fn RESERVED66();
    fn USB0_DCD();
    fn RTC();
    fn SMARTDMA();
    fn RESERVED70();
    fn CTIMER3();
    fn CTIMER4();
    fn OS_EVENT();
    fn RESERVED74();
    fn SAI0();
    fn SAI1();
    fn RESERVED77();
    fn CAN0();
    fn CAN1();
    fn RESERVED80();
    fn RESERVED81();
    fn USB1_HS_PHY();
    fn USB1_HS();
    fn SEC_HYPERVISOR_CALL();
    fn RESERVED85();
    fn RESERVED86();
    fn FREQME();
    fn SEC_VIO();
    fn ELS();
    fn PKC();
    fn PUF();
    fn RESERVED92();
    fn EDMA_1_CH0();
    fn EDMA_1_CH1();
    fn EDMA_1_CH2();
    fn EDMA_1_CH3();
    fn EDMA_1_CH4();
    fn EDMA_1_CH5();
    fn EDMA_1_CH6();
    fn EDMA_1_CH7();
    fn RESERVED101();
    fn RESERVED102();
    fn RESERVED103();
    fn RESERVED104();
    fn RESERVED105();
    fn RESERVED106();
    fn RESERVED107();
    fn RESERVED108();
    fn CDOG0();
    fn CDOG1();
    fn I3C0();
    fn I3C1();
    fn RESERVED113();
    fn GDET();
    fn VBAT0();
    fn EWM0();
    fn RESERVED117();
    fn RESERVED118();
    fn RESERVED119();
    fn RESERVED120();
    fn FLEXIO();
    fn RESERVED122();
    fn RESERVED123();
    fn RESERVED124();
    fn HSCMP0();
    fn HSCMP1();
    fn RESERVED127();
    fn FLEXPWM0_RELOAD_ERROR();
    fn FLEXPWM0_FAULT();
    fn FLEXPWM0_SUBMODULE0();
    fn FLEXPWM0_SUBMODULE1();
    fn FLEXPWM0_SUBMODULE2();
    fn FLEXPWM0_SUBMODULE3();
    fn FLEXPWM1_RELOAD_ERROR();
    fn FLEXPWM1_FAULT();
    fn FLEXPWM1_SUBMODULE0();
    fn FLEXPWM1_SUBMODULE1();
    fn FLEXPWM1_SUBMODULE2();
    fn FLEXPWM1_SUBMODULE3();
    fn QDC0_COMPARE();
    fn QDC0_HOME();
    fn QDC0_WDG_SAB();
    fn QDC0_IDX();
    fn QDC1_COMPARE();
    fn QDC1_HOME();
    fn QDC1_WDG_SAB();
    fn QDC1_IDX();
    fn ITRC0();
    fn RESERVED149();
    fn ELS_ERR();
    fn PKC_ERR();
    fn ERM_SINGLE_BIT_ERROR();
    fn ERM_MULTI_BIT_ERROR();
    fn FMU0();
    fn RESERVED155();
    fn RESERVED156();
    fn RESERVED157();
    fn RESERVED158();
    fn LPTMR0();
    fn LPTMR1();
    fn SCG();
    fn SPC();
    fn WUU();
    fn PORT_EFT();
    fn RESERVED165();
    fn RESERVED166();
    fn RESERVED167();
    fn WWDT0();
    fn WWDT1();
    fn CMC0();
    fn RESERVED171();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 156] = [
    Vector { _handler: OR },
    Vector {
        _handler: EDMA_0_CH0,
    },
    Vector {
        _handler: EDMA_0_CH1,
    },
    Vector {
        _handler: EDMA_0_CH2,
    },
    Vector {
        _handler: EDMA_0_CH3,
    },
    Vector {
        _handler: EDMA_0_CH4,
    },
    Vector {
        _handler: EDMA_0_CH5,
    },
    Vector {
        _handler: EDMA_0_CH6,
    },
    Vector {
        _handler: EDMA_0_CH7,
    },
    Vector {
        _handler: EDMA_0_CH8,
    },
    Vector {
        _handler: EDMA_0_CH9,
    },
    Vector {
        _handler: EDMA_0_CH10,
    },
    Vector {
        _handler: EDMA_0_CH11,
    },
    Vector {
        _handler: EDMA_0_CH12,
    },
    Vector {
        _handler: EDMA_0_CH13,
    },
    Vector {
        _handler: EDMA_0_CH14,
    },
    Vector {
        _handler: EDMA_0_CH15,
    },
    Vector { _handler: GPIO00 },
    Vector { _handler: GPIO01 },
    Vector { _handler: GPIO10 },
    Vector { _handler: GPIO11 },
    Vector { _handler: GPIO20 },
    Vector { _handler: GPIO21 },
    Vector { _handler: GPIO30 },
    Vector { _handler: GPIO31 },
    Vector { _handler: GPIO40 },
    Vector { _handler: GPIO41 },
    Vector { _handler: GPIO50 },
    Vector { _handler: GPIO51 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector {
        _handler: RESERVED49,
    },
    Vector { _handler: CTIMER2 },
    Vector {
        _handler: LP_FLEXCOMM0,
    },
    Vector {
        _handler: LP_FLEXCOMM1,
    },
    Vector {
        _handler: LP_FLEXCOMM2,
    },
    Vector {
        _handler: LP_FLEXCOMM3,
    },
    Vector {
        _handler: LP_FLEXCOMM4,
    },
    Vector {
        _handler: LP_FLEXCOMM5,
    },
    Vector {
        _handler: LP_FLEXCOMM6,
    },
    Vector {
        _handler: LP_FLEXCOMM7,
    },
    Vector {
        _handler: RESERVED59,
    },
    Vector {
        _handler: RESERVED60,
    },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: PINT0 },
    Vector {
        _handler: PDM_EVENT,
    },
    Vector {
        _handler: RESERVED65,
    },
    Vector {
        _handler: RESERVED66,
    },
    Vector { _handler: USB0_DCD },
    Vector { _handler: RTC },
    Vector { _handler: SMARTDMA },
    Vector {
        _handler: RESERVED70,
    },
    Vector { _handler: CTIMER3 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: OS_EVENT },
    Vector {
        _handler: RESERVED74,
    },
    Vector { _handler: SAI0 },
    Vector { _handler: SAI1 },
    Vector {
        _handler: RESERVED77,
    },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector {
        _handler: RESERVED80,
    },
    Vector {
        _handler: RESERVED81,
    },
    Vector {
        _handler: USB1_HS_PHY,
    },
    Vector { _handler: USB1_HS },
    Vector {
        _handler: SEC_HYPERVISOR_CALL,
    },
    Vector {
        _handler: RESERVED85,
    },
    Vector {
        _handler: RESERVED86,
    },
    Vector { _handler: FREQME },
    Vector { _handler: SEC_VIO },
    Vector { _handler: ELS },
    Vector { _handler: PKC },
    Vector { _handler: PUF },
    Vector {
        _handler: RESERVED92,
    },
    Vector {
        _handler: EDMA_1_CH0,
    },
    Vector {
        _handler: EDMA_1_CH1,
    },
    Vector {
        _handler: EDMA_1_CH2,
    },
    Vector {
        _handler: EDMA_1_CH3,
    },
    Vector {
        _handler: EDMA_1_CH4,
    },
    Vector {
        _handler: EDMA_1_CH5,
    },
    Vector {
        _handler: EDMA_1_CH6,
    },
    Vector {
        _handler: EDMA_1_CH7,
    },
    Vector {
        _handler: RESERVED101,
    },
    Vector {
        _handler: RESERVED102,
    },
    Vector {
        _handler: RESERVED103,
    },
    Vector {
        _handler: RESERVED104,
    },
    Vector {
        _handler: RESERVED105,
    },
    Vector {
        _handler: RESERVED106,
    },
    Vector {
        _handler: RESERVED107,
    },
    Vector {
        _handler: RESERVED108,
    },
    Vector { _handler: CDOG0 },
    Vector { _handler: CDOG1 },
    Vector { _handler: I3C0 },
    Vector { _handler: I3C1 },
    Vector {
        _handler: RESERVED113,
    },
    Vector { _handler: GDET },
    Vector { _handler: VBAT0 },
    Vector { _handler: EWM0 },
    Vector {
        _handler: RESERVED117,
    },
    Vector {
        _handler: RESERVED118,
    },
    Vector {
        _handler: RESERVED119,
    },
    Vector {
        _handler: RESERVED120,
    },
    Vector { _handler: FLEXIO },
    Vector {
        _handler: RESERVED122,
    },
    Vector {
        _handler: RESERVED123,
    },
    Vector {
        _handler: RESERVED124,
    },
    Vector { _handler: HSCMP0 },
    Vector { _handler: HSCMP1 },
    Vector {
        _handler: RESERVED127,
    },
    Vector {
        _handler: FLEXPWM0_RELOAD_ERROR,
    },
    Vector {
        _handler: FLEXPWM0_FAULT,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE0,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE1,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE2,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE3,
    },
    Vector {
        _handler: FLEXPWM1_RELOAD_ERROR,
    },
    Vector {
        _handler: FLEXPWM1_FAULT,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE0,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE1,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE2,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE3,
    },
    Vector {
        _handler: QDC0_COMPARE,
    },
    Vector {
        _handler: QDC0_HOME,
    },
    Vector {
        _handler: QDC0_WDG_SAB,
    },
    Vector { _handler: QDC0_IDX },
    Vector {
        _handler: QDC1_COMPARE,
    },
    Vector {
        _handler: QDC1_HOME,
    },
    Vector {
        _handler: QDC1_WDG_SAB,
    },
    Vector { _handler: QDC1_IDX },
    Vector { _handler: ITRC0 },
    Vector {
        _handler: RESERVED149,
    },
    Vector { _handler: ELS_ERR },
    Vector { _handler: PKC_ERR },
    Vector {
        _handler: ERM_SINGLE_BIT_ERROR,
    },
    Vector {
        _handler: ERM_MULTI_BIT_ERROR,
    },
    Vector { _handler: FMU0 },
    Vector {
        _handler: RESERVED155,
    },
    Vector {
        _handler: RESERVED156,
    },
    Vector {
        _handler: RESERVED157,
    },
    Vector {
        _handler: RESERVED158,
    },
    Vector { _handler: LPTMR0 },
    Vector { _handler: LPTMR1 },
    Vector { _handler: SCG },
    Vector { _handler: SPC },
    Vector { _handler: WUU },
    Vector { _handler: PORT_EFT },
    Vector {
        _handler: RESERVED165,
    },
    Vector {
        _handler: RESERVED166,
    },
    Vector {
        _handler: RESERVED167,
    },
    Vector { _handler: WWDT0 },
    Vector { _handler: WWDT1 },
    Vector { _handler: CMC0 },
    Vector {
        _handler: RESERVED171,
    },
];
