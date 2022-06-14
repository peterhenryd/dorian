use std::time::Instant;
use dorian::dorian::Dorian;
use dorian::llvm::execution_engine::{ExecutionEngine, ExtFn};
use dorian::llvm::{IntPredicate, OptimizationLevel};
use dorian::types::CreateType;
use dorian::types::fun::FunData;
use dorian::types::int::IntData;
use dorian::value::data::BuildValue;
use dorian::value::int::{BinOp, Int, IntValue};
use dorian::value::Value;

fn main() {
    let dorian = Dorian::new();
    let mut module = dorian.create_module("test");

    let int = IntData::Bits(32).create(&dorian);
    let mut fun = module
        .add_fn(
            "fib",
            &FunData::new(vec![&int], &int, false
            ).create(&dorian));

    let const_1 = int.const_int(1, false);
    let const_2 = int.const_int(2, false);

    let mut entry = fun.add_block("entry");
    let mut base = fun.add_block("base");
    let mut rec = fun.add_block("rec");

    if let [n] = fun.fetch_params().as_slice() {
        let n = n.as_int_value().unwrap();

        // ENTRY BLOCK
        {
            let cond = entry.compare_ints(IntPredicate::Ule, &n, &const_1.get_val());

            entry.if_statement(cond, base, rec);
        }
        // BASE BLOCK
        {
            base.return_value(&const_1.get_val());
        }
        // REC BLOCK
        {
            let a_n_minus_1 = Int::Bin(BinOp::Sub, &n, &const_1.get_val())
                .build(&rec);
            let b_n_minus_2 = Int::Bin(BinOp::Sub, &n, &const_2.get_val())
                .build(&rec);
            let c_fib_a = rec.call_fun::<IntValue>(&fun, vec![&a_n_minus_1]);
            let d_fib_b = rec.call_fun::<IntValue>(&fun, vec![&b_n_minus_2]);
            let e_c_plus_d = Int::Bin(BinOp::Add, &c_fib_a, &d_fib_b)
                .build(&rec);

            rec.return_value(&e_c_plus_d);
        }
    }

    let execution_engine = module.create_execution_engine(OptimizationLevel::Aggressive);

    let fib = unsafe {
        execution_engine.get_fun::<ExtFn<(u32,), u32>>("fib")
    }.unwrap();

    let time = Instant::now();

    assert_eq!(fib(10), 34);

    println!("{:?}", time.elapsed());
}