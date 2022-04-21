use vm::MemberValue;

use crate::*;

pub(crate) fn construct_virtual_vec<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<MemberValue<'eval>>::new(),
    )))
}

pub(crate) fn virtual_vec_element_move_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

pub(crate) fn virtual_vec_element_ref_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        StackValue::Primitive(value) => value.as_i32().unwrap().try_into().unwrap(),
        _ => panic!(),
    };
    if i > this_value.len() {
        todo!()
    }
    Ok(this_value[i].stack_ref())
}

pub(crate) fn virtual_vec_element_borrow_mut_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let i: usize = match values[1] {
        StackValue::Primitive(value) => value.as_i32().unwrap().try_into().unwrap(),
        _ => panic!(),
    };
    let (this_value, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    if i > this_value.len() {
        todo!()
    }
    Ok(this_value[i].stack_mut(stack_idx))
}

#[test]
fn test_i32_as_usize() {
    let a = (-1i32) as usize; // this will not fail
    p!(a);
    let b: usize = (-1i32).try_into().unwrap(); // this will fail
}
