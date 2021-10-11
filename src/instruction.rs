#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP_ADDR(u16),
    CALL_ADDR(u16),
    SE_BYTE(u16, u8),
    SNE_BYTE(u16, u8),
    SE(u16, u16),
    LD_BYTE(u16, u8),
    ADD_BYTE(u16, u8),
    LD(u16, u16),
    OR(u16, u16),
    AND(u16, u16),
    XOR(u16, u16),
    ADD(u16, u16),
    SUB(u16, u16),
    SHR(u16),
    SUBN(u16, u16),
    SHL(u16),
    SNE(u16, u16),
    LD_I(u16),
    JP_V0(u16),
    RND_BYTE(u16, u8),
    DRW(u16, u16, u8),
    SKP(u16),
    SKNP(u16),
    LD_DT(u16),
    LD_KEY(u16),
    LD_DT_SET(u16),
    LD_ST_SET(u16),
    ADD_I(u16),
    LD_F(u16),
    LD_B(u16),
    LD_STORE_I(u16),
    LD_READ_I(u16),
}

impl Instruction {
    fn parse_xkk(bytes: u16) -> (u16, u8) {
        let vx = (bytes & 0x0F00) >> 8;
        let kk = (bytes & 0x00FF) as u8;

        (vx, kk)
    }

    fn parse_xy(bytes: u16) -> (u16, u16) {
        let vx = (bytes & 0x0F00) >> 8;
        let vy = (bytes & 0x00F0) >> 4;

        (vx, vy)
    }

    pub fn parse(bytes: u16) -> Result<Instruction, String> {
        let parsed = match bytes {
            0x00E0 => Instruction::CLS,
            0x00EE => Instruction::RET,
            0x0000..=0x0FFF => Instruction::SYS(bytes & 0x0FFF),
            0x1000..=0x1FFF => Instruction::JP_ADDR(bytes & 0x0FFF),
            0x2000..=0x2FFF => Instruction::CALL_ADDR(bytes & 0x0FFF),
            0x3000..=0x3FFF => {
                let (vx, kk) = Instruction::parse_xkk(bytes);
                Instruction::SE_BYTE(vx, kk)
            }
            0x4000..=0x4FFF => {
                let (vx, arg) = Instruction::parse_xkk(bytes);
                Instruction::SNE_BYTE(vx, arg)
            }
            0x5000..=0x5FF0 => {
                let (vx, vy) = Instruction::parse_xy(bytes);
                Instruction::SE(vx, vy)
            }
            0x6000..=0x6FFF => {
                let (vx, kk) = Instruction::parse_xkk(bytes);
                Instruction::LD_BYTE(vx, kk)
            }
            0x7000..=0x7FFF => {
                let (vx, kk) = Instruction::parse_xkk(bytes);
                Instruction::ADD_BYTE(vx, kk)
            }
            0x8000..=0x8FFE => {
                let (vx, vy) = Instruction::parse_xy(bytes);
                let opcode = bytes & 0x000F;

                match opcode {
                    0x0 => Instruction::LD(vx, vy),
                    0x1 => Instruction::OR(vx, vy),
                    0x2 => Instruction::AND(vx, vy),
                    0x3 => Instruction::XOR(vx, vy),
                    0x4 => Instruction::ADD(vx, vy),
                    0x5 => Instruction::SUB(vx, vy),
                    0x6 => Instruction::SHR(vx),
                    0x7 => Instruction::SUBN(vx, vy),
                    0xE => Instruction::SHL(vx),
                    _ => panic!("Unhandled 8xy{:x?}", opcode),
                }
            }
            0x9000..=0x9FF0 => {
                let (vx, vy) = Instruction::parse_xy(bytes);
                Instruction::SNE(vx, vy)
            }
            0xA000..=0xAFFF => Instruction::LD_I(bytes & 0x0FFF),
            0xB000..=0xBFFF => Instruction::JP_V0(bytes & 0x0FFF),
            0xC000..=0xCFFF => {
                let (vx, kk) = Instruction::parse_xkk(bytes);
                Instruction::RND_BYTE(vx, kk)
            }
            0xD000..=0xDFFF => {
                let vx = (bytes & 0x0F00) >> 8;
                let vy = (bytes & 0x00F0) >> 4;
                let n = (bytes & 0x000F) as u8;

                Instruction::DRW(vx, vy, n)
            }
            0xE09E..=0xEF9E => {
                let (vx, _) = Instruction::parse_xy(bytes);
                Instruction::SKP(vx)
            }
            0xE0A1..=0xEFA1 => {
                let (vx, _) = Instruction::parse_xy(bytes);
                Instruction::SKNP(vx)
            }
            0xF007..=0xFF65 => {
                let (vx, _) = Instruction::parse_xy(bytes);
                let opcode = bytes & 0x00FF;

                match opcode {
                    0x07 => Instruction::LD_DT(vx),
                    0x0A => Instruction::LD_KEY(vx),
                    0x15 => Instruction::LD_DT_SET(vx),
                    0x18 => Instruction::LD_ST_SET(vx),
                    0x1E => Instruction::ADD_I(vx),
                    0x29 => Instruction::LD_F(vx),
                    0x33 => Instruction::LD_B(vx),
                    0x55 => Instruction::LD_STORE_I(vx),
                    0x65 => Instruction::LD_READ_I(vx),
                    _ => panic!("Unhandled fx{:x?}", opcode),
                }
            }
            _ => return Err(format!("Unable to parse {:x}", bytes)),
        };

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cls() {
        assert_eq!(Instruction::parse(0x00E0), Ok(Instruction::CLS));
    }

    #[test]
    fn test_parse_ret() {
        assert_eq!(Instruction::parse(0x00EE), Ok(Instruction::RET));
    }

    #[test]
    fn test_parse_sys() {
        assert_eq!(Instruction::parse(0x0123), Ok(Instruction::SYS(0x0123)));
    }

    #[test]
    fn test_parse_jp_addr() {
        assert_eq!(Instruction::parse(0x1123), Ok(Instruction::JP_ADDR(0x0123)));
    }

    #[test]
    fn test_parse_call_addr() {
        assert_eq!(
            Instruction::parse(0x2123),
            Ok(Instruction::CALL_ADDR(0x0123))
        );
    }

    #[test]
    fn test_parse_se_byte() {
        assert_eq!(
            Instruction::parse(0x3123),
            Ok(Instruction::SE_BYTE(0x0001, 0x0023))
        )
    }

    #[test]
    fn test_parse_sne_byte() {
        assert_eq!(
            Instruction::parse(0x4123),
            Ok(Instruction::SNE_BYTE(0x0001, 0x0023))
        )
    }

    #[test]
    fn test_parse_se() {
        assert_eq!(
            Instruction::parse(0x5120),
            Ok(Instruction::SE(0x0001, 0x0002))
        )
    }

    #[test]
    fn test_parse_or() {
        assert_eq!(
            Instruction::parse(0x87B1),
            Ok(Instruction::OR(0x0007, 0x000B))
        )
    }
}
