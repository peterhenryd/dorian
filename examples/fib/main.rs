use std::time::Instant;
use dorian::llvm::execution_engine::ExtFn;
use dorian::prelude::*;

fn main() {
    // Creates a new Dorian instance.
    let dorian = Dorian::new();
    // Crates a new module, named "test", belonging to the original Dorian instance.
    let mut module = dorian.create_module("test");

    // Creates a reusable 32-bit integer type.
    let int = IntData::Bits(32).create(&dorian);
    // Adds a function named "fib" to the "test" module, taking a 32-bit integer as its parameter,
    // returning a 32-bit integer.
    let mut fun = module
        .add_fn(
            "fib",
            &FunData::new(vec![&int], &int, false
            ).create(&dorian));

    // Creates reusable constant 32-bit integers.
    let const_0 = int.const_int(0, false);
    let const_1 = int.const_int(1, false);
    let const_2 = int.const_int(2, false);

    // Adds control-flow blocks to the `fib` function to be used later.
    let mut entry = fun.add_block("entry");
    let mut base = fun.add_block("base");
    let mut ret_0 = fun.add_block("ret_0");
    let mut ret_1 = fun.add_block("ret_1");
    let mut rec = fun.add_block("rec");

    //
    if let [n] = fun.fetch_params().as_slice() {
        let n = n.as_int_value().unwrap();

        // Braces added for readability.

        // ENTRY BLOCK
        {
            let cond = entry.compare_ints(IntPredicate::Sle, &n, &const_1.get_val());

            entry.if_statement(cond, &base, &rec);
        }
        // BASE BLOCK
        {
            let cond = base.compare_ints(IntPredicate::Slt, &n, &const_1.get_val());

            base.if_statement(cond, &ret_0, &ret_1);
        }

        // RET 0 BLOCK
        ret_0.return_value(const_0.get_val());

        // RET 1 BLOCK
        ret_1.return_value(const_1.get_val());

        // REC BLOCK
        {
            let a_n_minus_1 = Int::Bin(IntBinOp::Sub, &n, &const_1.get_val())
                .build(&rec);
            let b_n_minus_2 = Int::Bin(IntBinOp::Sub, &n, &const_2.get_val())
                .build(&rec);
            let c_fib_a = rec.call_fun::<IntValue>(&fun, vec![&a_n_minus_1.as_any_value()]);
            let d_fib_b = rec.call_fun::<IntValue>(&fun, vec![&b_n_minus_2.as_any_value()]);
            let e_c_plus_d = Int::Bin(IntBinOp::Add, &c_fib_a, &d_fib_b)
                .build(&rec);

            rec.return_value(&e_c_plus_d);
        }
    }

    let execution_engine = module.create_execution_engine(OptimizationLevel::Aggressive);

    let fib = execution_engine.get_fun::<ExtFn<(u32,), u32>>("fib").unwrap();

    let time = Instant::now();

    assert_eq!(unsafe { fib(10) }, 55);

    println!("{:?}", time.elapsed());
}