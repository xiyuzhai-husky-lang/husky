#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum AarchRegisterClass {
    GeneralPurpose,
    StackPointer,
    FloatingPoint,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum AarchRegister {
    // 64-bit general-purpose registers
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    XZR,

    // 32-bit general-purpose registers
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,
    WZR,

    // Stack pointer
    SP,
    WSP,

    // 128-bit floating-point registers
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
    Q8,
    Q9,
    Q10,
    Q11,
    Q12,
    Q13,
    Q14,
    Q15,
    Q16,
    Q17,
    Q18,
    Q19,
    Q20,
    Q21,
    Q22,
    Q23,
    Q24,
    Q25,
    Q26,
    Q27,
    Q28,
    Q29,
    Q30,
    Q31,

    // 64-bit floating-point registers
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,

    // 32-bit floating-point registers
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S20,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,

    // 16-bit floating-point registers
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
    H10,
    H11,
    H12,
    H13,
    H14,
    H15,
    H16,
    H17,
    H18,
    H19,
    H20,
    H21,
    H22,
    H23,
    H24,
    H25,
    H26,
    H27,
    H28,
    H29,
    H30,
    H31,

    // 8-bit floating-point registers
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
    B16,
    B17,
    B18,
    B19,
    B20,
    B21,
    B22,
    B23,
    B24,
    B25,
    B26,
    B27,
    B28,
    B29,
    B30,
    B31,
}

impl AarchRegister {
    pub fn class(self) -> AarchRegisterClass {
        match self {
            r if AarchRegister::X0 <= r && r <= AarchRegister::XZR => {
                AarchRegisterClass::GeneralPurpose
            }
            r if AarchRegister::W0 <= r && r <= AarchRegister::WZR => {
                AarchRegisterClass::GeneralPurpose
            }
            AarchRegister::SP | AarchRegister::WSP => AarchRegisterClass::StackPointer,
            r if AarchRegister::Q0 <= r && r <= AarchRegister::Q31
                || AarchRegister::D0 <= r && r <= AarchRegister::D31
                || AarchRegister::S0 <= r && r <= AarchRegister::S31
                || AarchRegister::H0 <= r && r <= AarchRegister::H31
                || AarchRegister::B0 <= r && r <= AarchRegister::B31 =>
            {
                AarchRegisterClass::FloatingPoint
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn aarch_register_class_works() {
    assert_eq!(
        AarchRegister::X0.class(),
        AarchRegisterClass::GeneralPurpose
    );
    assert_eq!(
        AarchRegister::X1.class(),
        AarchRegisterClass::GeneralPurpose
    );
    assert_eq!(
        AarchRegister::X2.class(),
        AarchRegisterClass::GeneralPurpose
    );
    assert_eq!(
        AarchRegister::X3.class(),
        AarchRegisterClass::GeneralPurpose
    );
    assert_eq!(
        AarchRegister::X4.class(),
        AarchRegisterClass::GeneralPurpose
    );
    assert_eq!(
        AarchRegister::X5.class(),
        AarchRegisterClass::GeneralPurpose
    );
}
