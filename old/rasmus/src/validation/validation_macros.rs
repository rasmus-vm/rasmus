#[macro_export]
macro_rules! extract_lane_stack_type {
    (i8 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i16 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i32 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i64 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i64()],
        }
    };
    (f32 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::f32()],
        }
    };
    (f64 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::f64()],
        }
    };
}

#[macro_export]
macro_rules! replace_lane_stack_type {
    (i8 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i16 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i32 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i64 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i64()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (f32 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::f32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (f64 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::f64()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
}

// TODO: rid of returning StackType, just check
#[macro_export]
macro_rules! check {
  (extract_lane $n:ident, $dim:expr, $lane_idx:expr) => {{
      let dim: u8 = $dim;

      if $lane_idx.0 >= dim {
          return Err(ValidationError::LaneIndexIsOutOfRange {
              value: $lane_idx.0,
              max_allowed: dim - 1,
          });
      }

      crate::extract_lane_stack_type!($n $dim,  vec![OpdType::Strict(ValType::v128())])
  }};
  (replace_lane $n:ident, $dim:expr, $lane_idx:expr) => {{
      let dim: u8 = $dim;

      if $lane_idx.0 >= dim {
          return Err(ValidationError::LaneIndexIsOutOfRange {
              value: $lane_idx.0,
              max_allowed: dim - 1,
          });
      }

      crate::replace_lane_stack_type!($n $dim,  vec![OpdType::Strict(ValType::v128())])
  }};
  (memarg $bits:expr, $ctx:expr, $memarg:expr) => {{
    if $ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    if 2u8.pow($memarg.0.0) > $bits / 8 {
        return Err(ValidationError::MemargAlignTooBig);
    }
  }};
  (memarg_vec_load $ctx:expr, $memarg:expr, $n:expr, $m:expr) => {{
    if $ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    if 2u8.pow($memarg.0.0) > $n / 8 * $m {
        return Err(ValidationError::MemargAlignTooBig);
    }

    StackType {
        inputs: vec![
            OpdType::Strict(ValType::i32()),
        ],
        outputs: vec![
            ValType::v128()
        ],
    }
  }};
  (memarg_vec_load_lane $ctx:expr, $memarg:expr, $lane_idx:expr, $n:expr) => {{
    let max = $n as u8 / 8;
    if $lane_idx.0 >= max {
        return Err(ValidationError::LaneIdxTooBix);
    }

    crate::check!{
        memarg $n, $ctx, $memarg
    }

    StackType {
        inputs: vec![
            OpdType::Strict(ValType::i32()),
            OpdType::Strict(ValType::v128()),
        ],
        outputs: vec![
            ValType::v128()
        ],
    }
  }};
  (memarg_vec_store_lane $ctx:expr, $memarg:expr, $lane_idx:expr, $n:expr) => {
    crate::check!{
        memarg_vec_load_lane $ctx, $memarg, $lane_idx, $n
    }
  };
}
