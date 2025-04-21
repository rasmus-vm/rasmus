use crate::entities::traits::AsSigned;

use crate::result::RResult;

#[macro_export]
macro_rules! binop {
    ($stack: expr, $first_type: path, $second_type: path, $ret: path, $($op: tt)*) => {
        if let Some($first_type(second)) = $stack.pop_value() {
            if let Some($second_type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value($ret(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
    ($stack: expr, $type: path, $($op: tt)*) => {
        if let Some($type(second)) = $stack.pop_value() {
            if let Some($type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value($type(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
}

pub fn bitselect<T>(first: T, second: T, third: T) -> RResult<T>
where
    T: ::std::ops::Not<Output = T>
        + ::std::ops::BitAnd<Output = T>
        + ::std::ops::BitOr<Output = T>
        + ::std::marker::Copy,
{
    Ok((first & third) | (second & !third))
}

pub fn eq<L, R>(lhs: L, rhs: R) -> RResult<u32>
where
    L: ::std::cmp::PartialEq<R>,
{
    Ok(if lhs == rhs { 1 } else { 0 })
}

pub fn neq<L, R>(lhs: L, rhs: R) -> RResult<u32>
where
    L: ::std::cmp::PartialEq<R>,
{
    Ok(if lhs != rhs { 1 } else { 0 })
}

pub fn eqz<T>(lhs: T) -> RResult<u32>
where
    T: Into<u64>,
{
    Ok(if lhs.into() == 0u64 { 1 } else { 0 })
}

pub fn lts<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) < (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn ltu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs < rhs { 1 } else { 0 })
}

pub fn gts<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) > (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn gtu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs > rhs { 1 } else { 0 })
}

pub fn les<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) <= (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn leu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs <= rhs { 1 } else { 0 })
}

pub fn ges<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) >= (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn geu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs >= rhs { 1 } else { 0 })
}

#[macro_export]
macro_rules! testop_impl {
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(exec_fn: impl FnOnce($type) -> RResult<u32>, stack: &mut Stack) -> RResult<()> {
            if let Some($pattern(first)) = stack.pop_value() {
                let result = exec_fn(first)?;
                stack.push_entry(StackEntry::Value(crate::instances::value::Val::I32(result)));
            } else {
                return Err(Trap);
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! relop_impl {
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(
            exec_fn: impl FnOnce($type, $type) -> RResult<u32>,
            stack: &mut Stack,
        ) -> RResult<()> {
            if let Some($pattern(second)) = stack.pop_value() {
                if let Some($pattern(first)) = stack.pop_value() {
                    let result = exec_fn(first, second)?;
                    stack.push_entry(StackEntry::Value(crate::instances::value::Val::I32(result)));
                } else {
                    return Err(Trap);
                }
            } else {
                return Err(Trap);
            }
            Ok(())
        }
    };
}

// Rust float is already defined in IEEE 754 standard, so using `as`.
#[macro_export]
macro_rules! float {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| arg as $ret_type
    };
}

#[macro_export]
macro_rules! is_ref_null {
    ($stack: expr) => {
        if let Some($crate::instances::value::Val::Ref(reference)) = $stack.pop_value() {
            let is_null = match reference {
                $crate::instances::ref_inst::RefInst::Null(_) => 1u32,
                _ => 0u32,
            };
            $stack.push_entry(StackEntry::Value(Val::I32(is_null)));
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! ref_func_m {
    ($stack: expr, $func_idx: expr) => {
        match $stack.current_frame() {
            Some(frame) => match frame.module.funcaddrs.get($func_idx) {
                Some(funcaddr) => $stack.push_entry(StackEntry::Value(Val::Ref(
                    $crate::instances::ref_inst::RefInst::Func(funcaddr.clone()),
                ))),
                None => {
                    return Err(Trap);
                }
            },
            None => {
                return Err(Trap);
            }
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_float_f32_from_u32() {
        let make_float = float!(u32, f32);

        assert_eq!(make_float(0), 0.0f32, "should properly convert zero");
        assert_eq!(
            make_float(1),
            1.0f32,
            "should properly convert exact number"
        );
        assert_eq!(
            make_float(1),
            1.0f32,
            "should properly convert exact number"
        );
    }
}
