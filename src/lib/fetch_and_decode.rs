use crate::fetch_and_decode::OPCODE::OP00EE;


#[derive(PartialEq, Debug)]
pub enum OPCODE {
    OP00E0,
    OP00EE,
    OP1NNN,
    OP2NNN,
    OP3XNN,
    OP4XNN,
    OP5XY0,
    OP6XNN,
    OP7XNN,
    OP8XY0,
    OP8XY1,
    OP8XY2,
    OP8XY3,
    OP8XY4,
    OP8XY5,
    OP8XY6,
    OP8XY7,
    OP8XYE,
    OP9XY0,
    OPANNN,
    OPBNNN,
    OPCXNN,
    OPDXYN,
    OPEX9E,
    OPEXA1,
    OPFX07,
    OPFX0A,
    OPFX15,
    OPFX18,
    OPFX1E,
    OPFX29,
    OPFX33,
    OPFX55,
    OPFX65,
    None,
}
pub fn fetch(code: &[u8; 2]) -> OPCODE {
    let firstnib = code[0] >> 4;
    match firstnib{
        0x00 => {
            match code[1] {
                0xe0 => OPCODE::OP00E0,
                0xee => OPCODE::OP00EE,
                _ => OPCODE::None,
            }
        },
        0x01 => OPCODE::OP1NNN,
        0x02 => OPCODE::OP2NNN,
        0x03 => OPCODE::OP3XNN,
        0x04 => OPCODE::OP4XNN,
        0x05 => OPCODE::OP5XY0,
        0x06 => OPCODE::OP6XNN,
        0x07 => OPCODE::OP7XNN,
        0x08 => { match  code[1] & 0x0F {
            0x00 => OPCODE::OP8XY0,
            0x01 => OPCODE::OP8XY1,
            0x02 => OPCODE::OP8XY2,
            0x03 => OPCODE::OP8XY3,
            0x04 => OPCODE::OP8XY4,
            0x05 => OPCODE::OP8XY5,
            0x06 => OPCODE::OP8XY6,
            0x07 => OPCODE::OP8XY7,
            0x0e => OPCODE::OP8XYE,
            _ => OPCODE::None,
            }
        },
        0x09 => OPCODE::OP9XY0,
        0x0a => OPCODE::OPANNN,
        0x0b => OPCODE::OPBNNN,
        0x0c => OPCODE::OPCXNN,
        0x0d => OPCODE::OPDXYN,
        0x0e => {
            match code[1]{
                0x9E => OPCODE::OPEX9E,
                0xA1 => OPCODE::OPEXA1,
                _ => OPCODE::None
            }
        },
        0x0f => {
            match code[1] {
                0x07 => OPCODE::OPFX07,
                0x0A => OPCODE::OPFX0A,
                0x15 => OPCODE::OPFX15,
                0x18 => OPCODE::OPFX18,
                0x1E => OPCODE::OPFX1E,
                0x29 => OPCODE::OPFX29,
                0x33 => OPCODE::OPFX33,
                0x55 => OPCODE::OPFX55,
                0x65 => OPCODE::OPFX65,
                _ => OPCODE::None,
            }
        },
        _ => OPCODE::None, // Panic in the end
    }
}


mod test{
    use super::*;
    #[test]
    fn test_fetch_op_not_exist(){
        assert_eq!(fetch(&[0, 0]), OPCODE::None)
    }
    #[test]
    fn test_fetch_00ee(){
        assert_eq!(fetch(&[0x00, 0xee]), OPCODE::OP00EE)
    }
    #[test]
    fn test_fetch_00e0(){
        assert_eq!(fetch(&[0x00, 0xe0]), OPCODE::OP00E0)
    }
    #[test]
    fn test_fetch_0NNN_None(){
        for n in 0..100{
            assert_eq!(fetch(&[0x01, n]), OPCODE::None);
            assert_eq!(fetch(&[0x04, n]), OPCODE::None);
            assert_eq!(fetch(&[0x03, n]), OPCODE::None);
            assert_eq!(fetch(&[0x02, n]), OPCODE::None);
        }
    }
    #[test]
    fn test_fetch_1NNN(){
        assert_eq!(fetch(&[0x1a, 0x21]), OPCODE::OP1NNN)
    }
    #[test]
    fn test_fetch_2NNN(){
        assert_eq!(fetch(&[0x2a, 0x21]), OPCODE::OP2NNN)
    }
    #[test]
    fn test_fetch_3XNN(){
        assert_eq!(fetch(&[0x34, 0x21]), OPCODE::OP3XNN)
    }
    #[test]
    fn test_fetch_4XNN(){
        assert_eq!(fetch(&[0x49, 0x11]), OPCODE::OP4XNN)
    }
    #[test]
    fn test_fetch_5XY0(){
        assert_eq!(fetch(&[0x5a, 0x20]), OPCODE::OP5XY0)
    }
    #[test]
    fn test_fetch_6XNN(){
        assert_eq!(fetch(&[0x6a, 0x21]), OPCODE::OP6XNN)
    }
    #[test]
    fn test_fetch_7XNN(){
        assert_eq!(fetch(&[0x7c, 0x83]), OPCODE::OP7XNN)
    }
    #[test]
    fn test_fetch_8XY0(){
        assert_eq!(fetch(&[0x84, 0x30]), OPCODE::OP8XY0)
    }
    #[test]
    fn test_fetch_8XY1(){
        assert_eq!(fetch(&[0x84, 0x31]), OPCODE::OP8XY1)
    }
    #[test]
    fn test_fetch_8XY2(){
        assert_eq!(fetch(&[0x84, 0x32]), OPCODE::OP8XY2)
    }
    #[test]
    fn test_fetch_8XY3(){
        assert_eq!(fetch(&[0x84, 0x33]), OPCODE::OP8XY3)
    }
    #[test]
    fn test_fetch_8XY4(){
        assert_eq!(fetch(&[0x8E, 0x94]), OPCODE::OP8XY4)
    }
    #[test]
    fn test_fetch_8XY5(){
        assert_eq!(fetch(&[0x84, 0x35]), OPCODE::OP8XY5)
    }
    #[test]
    fn test_fetch_8XY6(){
        assert_eq!(fetch(&[0x84, 0x36]), OPCODE::OP8XY6)
    }
    #[test]
    fn test_fetch_8XY7(){
        assert_eq!(fetch(&[0x8c, 0x97]), OPCODE::OP8XY7)
    }
    #[test]
    fn test_fetch_8XYE(){
        assert_eq!(fetch(&[0x84, 0xdE]), OPCODE::OP8XYE)
    }
    #[test]
    fn test_fetch_9XY0(){
        assert_eq!(fetch(&[0x92, 0x10]), OPCODE::OP9XY0)
    }
    #[test]
    fn test_fetch_ANNN(){
        assert_eq!(fetch(&[0xA0, 0x12]), OPCODE::OPANNN)
    }
    #[test]
    fn test_fetch_BNNN(){
        assert_eq!(fetch(&[0xBc, 0x19]), OPCODE::OPBNNN)
    }
    #[test]
    fn test_fetch_CXNN(){
        assert_eq!(fetch(&[0xCF, 0x9c]), OPCODE::OPCXNN)
    }
    #[test]
    fn test_fetch_DXYN(){
        assert_eq!(fetch(&[0xDb, 0xF1]), OPCODE::OPDXYN)
    }
    #[test]
    fn test_fetch_EXA1(){
        assert_eq!(fetch(&[0xE9, 0xA1]), OPCODE::OPEXA1)
    }
    #[test]
    fn test_fetch_EX9E(){
        assert_eq!(fetch(&[0xE4, 0x9E]), OPCODE::OPEX9E)
    }
    #[test]
    fn test_fetch_FX07(){
        assert_eq!(fetch(&[0xF7, 0x07]), OPCODE::OPFX07)
    }
    #[test]
    fn test_fetch_FX0A(){
        assert_eq!(fetch(&[0xF7, 0x0A]), OPCODE::OPFX0A)
    }
    #[test]
    fn test_fetch_FX15(){
        assert_eq!(fetch(&[0xF4, 0x15]), OPCODE::OPFX15)
    }
    #[test]
    fn test_fetch_FX18(){
        assert_eq!(fetch(&[0xF7, 0x18]), OPCODE::OPFX18)
    }
    #[test]
    fn test_fetch_FX1E(){
        assert_eq!(fetch(&[0xF2, 0x1E]), OPCODE::OPFX1E)
    }
    #[test]
    fn test_fetch_FX29(){
        assert_eq!(fetch(&[0xF7, 0x29]), OPCODE::OPFX29)
    }
    #[test]
    fn test_fetch_FX33(){
        assert_eq!(fetch(&[0xF7, 0x33]), OPCODE::OPFX33)
    }
    #[test]
    fn test_fetch_FX55(){
        assert_eq!(fetch(&[0xFC, 0x55]), OPCODE::OPFX55)
    }
    #[test]
    fn test_fetch_F65(){
        assert_eq!(fetch(&[0xF7, 0x65]), OPCODE::OPFX65)
    }
    #[test]
    fn test_fetch_bad_F(){
        assert_eq!(fetch(&[0xFB, 0x11]), OPCODE::None)
    }
    #[test]
    fn test_fetch_bad_E(){
        assert_eq!(fetch(&[0xE7, 0x2E]), OPCODE::None)
    }
}