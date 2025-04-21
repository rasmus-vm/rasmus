(module
  (func $condition (param $n i32) (result i32)
    local.get $n
    if (result i32)
    	(return (i32.const 10))
    else
    	(return (i32.const 20))
    end
  )

  (export "condition" (func $condition))
)