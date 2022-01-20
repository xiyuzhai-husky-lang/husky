use super::*;
use common::*;

#[test]
fn add_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(2.into());
    stack.push_primitive(3.into());
    stack.add().unwrap();
    should_eq!(stack.finish_as_primitive(), 5.into());
}

#[test]
fn add_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(2.0.into());
    stack.push_primitive(3.2.into());
    stack.add().unwrap();
    should_eq!(stack.finish_as_primitive(), 5.2.into());
}

#[test]
fn add_assign_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive(2.into());
    stack.push_primitive(3.into());
    stack.add_assign(1).unwrap();
    should_eq!(stack.primitive(1), 5.into());
}

#[test]
fn add_assign_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive(2.0.into());
    stack.push_primitive(3.2.into());
    stack.add_assign(1).unwrap();
    should_eq!(stack.primitive(1), 5.2.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn and() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(true.into());
    stack.push_primitive(false.into());
    stack.and().unwrap();
    should_eq!(stack.finish_as_primitive(), false.into());
}

#[test]
fn bitand_u32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(10u32.into());
    stack.push_primitive(3u32.into());
    stack.bitand().unwrap();
    should_eq!(stack.finish_as_primitive(), 2u32.into());
}

#[test]
fn bitand_assign_u32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(10u32.into());
    stack.push_primitive(10u32.into());
    stack.push_primitive(3u32.into());
    stack.bitand_assign(1).unwrap();
    should_eq!(stack.primitive(1), 2u32.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn bitor_u32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(0b0011u32.into());
    stack.push_primitive(0b0101u32.into());
    stack.bitor().unwrap();
    should_eq!(stack.finish_as_primitive(), 0b0111u32.into());
}

#[test]
fn bitor_assign_u32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(10u32.into());
    stack.push_primitive(0b0011u32.into());
    stack.push_primitive(0b0101u32.into());
    stack.bitor_assign(1).unwrap();
    should_eq!(stack.primitive(1), 0b0111u32.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn div_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(5.into());
    stack.push_primitive(2.into());
    stack.div().unwrap();
    should_eq!(stack.finish_as_primitive(), 2.into());
}

#[test]
fn mul_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(2.into());
    stack.push_primitive(3.into());
    stack.mul().unwrap();
    should_eq!(stack.finish_as_primitive(), 6.into());
}

#[test]
fn mul_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(2.0.into());
    stack.push_primitive(3.2.into());
    stack.mul().unwrap();
    should_eq!(stack.finish_as_primitive(), 6.4.into());
}

#[test]
fn mul_assign_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive(2.into());
    stack.push_primitive(3.into());
    stack.mul_assign(1).unwrap();
    should_eq!(stack.primitive(1), 6.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn mul_assign_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive(2.0.into());
    stack.push_primitive(3.2.into());
    stack.mul_assign(1).unwrap();
    should_eq!(stack.primitive(1), 6.4.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn test_rust_rem_euclid() {
    assert_eq!((-2 as i32).rem_euclid(5), 3);
    assert_eq!((-2.2 as f32).rem_euclid(5.0), 2.8);
}

#[test]
fn rem_euclid_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive((-2).into());
    stack.push_primitive(7.into());
    stack.rem_euclid().unwrap();
    should_eq!(stack.finish_as_primitive(), 5.into());
}

#[test]
fn rem_euclid_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive((-2.1).into());
    stack.push_primitive(5.0.into());
    stack.rem_euclid().unwrap();
    should_eq!(stack.finish_as_primitive(), 2.9.into());
}

#[test]
fn rem_euclid_assign_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive((-2).into());
    stack.push_primitive(7.into());
    stack.rem_euclid_assign(1).unwrap();
    should_eq!(stack.primitive(1), 5.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn rem_euclid_assign_f32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive((-2.1).into());
    stack.push_primitive(5.0.into());
    stack.rem_euclid_assign(1).unwrap();
    should_eq!(stack.primitive(1), 2.9.into());
    should_eq!(stack.len(), 2);
}

#[test]
fn sub_i32() {
    let mut stack = VirtualStack::new_with_size(100);
    stack.push_primitive(1.into());
    stack.push_primitive((-2).into());
    stack.push_primitive(7.into());
    stack.sub().unwrap();
    should_eq!(stack.primitive(1), (-9).into());
    should_eq!(stack.len(), 2);
}

impl<'stack> VirtualStack<'stack> {
    fn primitive(&self, idx: u16) -> PrimitiveValue {
        should_ok!(should_ok!(self.var(idx.into())).as_primitive())
    }

    fn finish_as_primitive(&mut self) -> PrimitiveValue {
        should_ok!(self.finish().unwrap().as_primitive())
    }

    fn push_primitive(&mut self, value: PrimitiveValue) {
        self.exec(&InstructionKind::PushPrimitive(value)).unwrap();
    }

    fn calculate_primitive_binary(&mut self, func: PrimitiveBinaryFunc) -> InterpretResult<()> {
        self.exec(&InstructionKind::Primitive(PrimitiveOpn::Binary(func)))?;
        Ok(())
    }

    fn assign_primitive_binary(
        &mut self,
        kind: PrimitiveBinaryFunc,
        dst_idx: u16,
    ) -> InterpretResult<()> {
        self.exec(&InstructionKind::Primitive(PrimitiveOpn::BinaryAssign {
            func: kind,
            dst_idx,
        }))?;
        Ok(())
    }

    fn add(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::Add)
    }

    fn and(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::And)
    }

    fn add_assign(&mut self, idx: u16) -> InterpretResult<()> {
        self.assign_primitive_binary(PrimitiveBinaryFunc::Add, idx)
    }

    fn bitand(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::BitAnd)
    }

    fn bitand_assign(&mut self, idx: u16) -> InterpretResult<()> {
        self.assign_primitive_binary(PrimitiveBinaryFunc::BitAnd, idx)
    }

    fn bitor(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::BitOr)
    }

    fn bitor_assign(&mut self, idx: u16) -> InterpretResult<()> {
        self.assign_primitive_binary(PrimitiveBinaryFunc::BitOr, idx)
    }

    fn div(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::Div)
    }

    fn mul(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::Mul)
    }

    fn mul_assign(&mut self, idx: u16) -> InterpretResult<()> {
        self.assign_primitive_binary(PrimitiveBinaryFunc::Mul, idx)
    }

    fn rem_euclid(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::RemEuclid)
    }

    fn rem_euclid_assign(&mut self, idx: u16) -> InterpretResult<()> {
        self.assign_primitive_binary(PrimitiveBinaryFunc::RemEuclid, idx)
    }

    fn sub(&mut self) -> InterpretResult<()> {
        self.calculate_primitive_binary(PrimitiveBinaryFunc::Sub)
    }
}
