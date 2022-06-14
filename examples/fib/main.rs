use std::time::Instant;
use dorian::dorian::Dorian;
use dorian::llvm::execution_engine::ExtFn;
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

    let const_0 = int.const_int(0, false);
    let const_1 = int.const_int(1, false);
    let const_2 = int.const_int(2, false);

    let mut entry = fun.add_block("entry");
    let mut base = fun.add_block("base");
    let mut ret_0 = fun.add_block("ret_0");
    let mut ret_1 = fun.add_block("ret_1");
    let mut rec = fun.add_block("rec");

    if let [n] = fun.fetch_params().as_slice() {
        let n = n.as_int_value().unwrap();

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
            let a_n_minus_1 = Int::Bin(BinOp::Sub, &n, &const_1.get_val())
                .build(&rec);
            let b_n_minus_2 = Int::Bin(BinOp::Sub, &n, &const_2.get_val())
                .build(&rec);
            let c_fib_a = rec.call_fun::<IntValue>(&fun, vec![&a_n_minus_1.as_any_value()]);
            let d_fib_b = rec.call_fun::<IntValue>(&fun, vec![&b_n_minus_2.as_any_value()]);
            let e_c_plus_d = Int::Bin(BinOp::Add, &c_fib_a, &d_fib_b)
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