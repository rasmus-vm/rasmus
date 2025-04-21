(module
  ;; import the host console object
  ;; (import "console" "log" (func $log (param i32)))

  (func $factorial (param $n i32) (result i32)
    ;; is 0 or 1
    (i32.or
      (i32.eq (local.get $n) (i32.const 0))
      (i32.eq (local.get $n) (i32.const 1))
    )

    if (result i32)
    	(return (i32.const 1))
    else
    	(return
          (i32.mul (local.get $n) (call $factorial (i32.sub (local.get $n) (i32.const 1))))
        )
    end
  )

  (export "factorial" (func $factorial))


  ;; (func $main
  ;;   (call $factorial (i32.const 0))
  ;;   (call $log)
  ;;   (call $factorial (i32.const 1))
  ;;   (call $log)
  ;;   (call $factorial (i32.const 4))
  ;;   (call $log)
  ;; )

  ;; (start $main) ;; run the first function automatically
)