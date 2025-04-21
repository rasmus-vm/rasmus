(module
  (import "factorial-lib" "factorial" (func $factorial_lib (param i32) (result i32)))

  (func $factorial (param $n i32) (result i32)
    (return
      (call $factorial_lib (local.get $n))
    )
  )

  (export "factorial" (func $factorial))
)