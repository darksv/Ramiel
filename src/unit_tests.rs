#[cfg(test)]
use crate::{DoubleTarget, Target, ArithmeticTarget, CPU, Instruction, JumpCondition};

mod instructions_unit {
    use super::*;

    #[test]
    fn add() {//ADD A, r8
        let mut cpu = CPU::default();
        cpu.registers.a = 1;
        cpu.registers.b = 2;
        cpu.execute(Instruction::ADD(Target::Register(ArithmeticTarget::A), Target::Register(ArithmeticTarget::B)));

        // Debug prints
        println!("Register A: {}", cpu.registers.a);
        println!("Register B: {}", cpu.registers.b);

        assert_eq!(cpu.registers.a, 3);
    }

    #[test]
    fn addhl() {//ADD HL, r16
        let mut cpu = CPU::default();
        cpu.registers.h = 0x12;
        cpu.registers.l = 0x34;
        cpu.registers.b = 0x56;
        cpu.registers.c = 0x78;
        cpu.execute(Instruction::ADD(Target::Register16(DoubleTarget::HL), Target::Register16(DoubleTarget::BC)));
        assert_eq!(cpu.registers.h, 0x68);
        assert_eq!(cpu.registers.l, 0xAC);
    }

    #[test]
    fn adc() {//ADC r8
        let mut cpu = CPU::default();
        cpu.registers.a = 3;
        cpu.registers.b = 255;
        cpu.execute(Instruction::ADC(Target::Register(ArithmeticTarget::B)));
        assert_eq!(cpu.registers.a, 3);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_adc_a_hl_no_carry() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0x12);
        cpu.registers.a = 0x34;
        cpu.registers.f.carry = false;

        cpu.execute(Instruction::ADC(Target::MemoryR16(DoubleTarget::HL)));

        assert_eq!(cpu.registers.a, 0x46);
        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.subtract);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
    }

    #[test]
    fn test_adc_a_hl_with_carry() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0x12);
        cpu.registers.a = 0x34;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADC(Target::MemoryR16(DoubleTarget::HL)));

        assert_eq!(cpu.registers.a, 0x47);
        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.subtract);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
    }

    #[test]
    fn test_adc_a_hl_with_overflow() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0xFF);
        cpu.registers.a = 0x01;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADC(Target::MemoryR16(DoubleTarget::HL)));

        assert_eq!(cpu.registers.a, 0x01);
        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.subtract);
        assert!(cpu.registers.f.carry);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn and() {//AND r8
        let mut cpu = CPU::default();
        cpu.registers.a = 0b10101010;
        cpu.registers.b = 0b11001100;
        cpu.execute(Instruction::AND(ArithmeticTarget::B));
        assert_eq!(cpu.registers.a, 0b10001000);
    }

    #[test]
    fn bit(){ //BIT r8
        let mut cpu = CPU::default();
        cpu.registers.a = 0b00001000;
        cpu.execute(Instruction::BIT(3,ArithmeticTarget::A));
        assert!(!cpu.registers.f.zero);
    }

    #[test]
    fn ccf(){ //CCF
        let mut cpu = CPU::default();
        cpu.registers.f.carry = true;
        cpu.execute(Instruction::CCF());
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn cp(){ //CP r8
        let mut cpu = CPU::default();
        cpu.registers.a = 0x12;
        cpu.registers.b = 0x12;
        cpu.execute(Instruction::CP(ArithmeticTarget::B));
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn cpl(){ //CPL
        let mut cpu = CPU::default();
        cpu.registers.a = 0b10101010;
        cpu.execute(Instruction::CPL());
        assert_eq!(cpu.registers.a, 0b01010101);
        assert!(cpu.registers.f.subtract);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn daa() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0x3c; // 0x3c = 60 DEC
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
        cpu.execute(Instruction::DAA());
        assert_eq!(cpu.registers.a, 0x42); // Sprawdź, czy A = 42 DEC
        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn dec() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0x01;
        cpu.execute(Instruction::DEC(ArithmeticTarget::A));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.zero);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn inc() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0x01;
        cpu.execute(Instruction::INC(ArithmeticTarget::A));
        assert_eq!(cpu.registers.a, 0x02);
        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn ei() {
        let mut cpu = CPU::default();
        cpu.registers.ime = false;
        cpu.execute(Instruction::EI());
        assert!(cpu.registers.ime);
    }

    #[test]
    fn di() {
        let mut cpu = CPU::default();
        cpu.registers.ime = true;
        cpu.execute(Instruction::DI());
        assert!(!cpu.registers.ime);
    }

    #[test]
    fn ld() { // LD r8, r8
        let mut cpu = CPU::default();
        cpu.registers.a = 0x01;
        cpu.execute(Instruction::LD(Target::Register(ArithmeticTarget::B), Target::Register(ArithmeticTarget::A)));
        assert_eq!(cpu.registers.b, 0x01);
    }

    #[test]
    fn xor(){
        let mut cpu = CPU::default();
        cpu.registers.a = 0b10101010;
        cpu.registers.b = 0b11001100;
        cpu.execute(Instruction::XOR(ArithmeticTarget::B));
        assert_eq!(cpu.registers.a, 0b01100110);
    }
    #[test]
    fn test_jp() {
        let mut cpu = CPU::default();
        cpu.execute(Instruction::JP(JumpCondition::Always, 0x1234));
        assert_eq!(cpu.pc, 0x1234);
    }

    #[test]
    fn test_jp_nz() {
        let mut cpu = CPU::default();
        cpu.registers.f.zero = false;
        cpu.execute(Instruction::JP(JumpCondition::NotZero, 0x1234));
        assert_eq!(cpu.pc, 0x1234);
    }
}