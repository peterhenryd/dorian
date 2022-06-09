use dorian::dorian::Dorian;
use dorian::llvm::execution_engine::ExtFn;
use dorian::llvm::OptimizationLevel;
use dorian::types::CreateType;
use dorian::types::fun::FunData;
use dorian::types::int::IntData;
use dorian::value::data::BuildValue;
use dorian::value::int::{BinOp, Int};
use dorian::value::Value;

fn main() {
    let dorian = Dorian::new();

    let mut test = dorian.create_module("test");

    let i64 = IntData::Bits(64).create(&dorian);
    let fun_type = FunData::new(vec![&i64, &i64], &i64, false).create(&dorian);

    let mut fun = test.add_fn("add", &fun_type);

    let mut entry = fun.add_block("entry");
    if let [lhs, rhs] = fun.fetch_params().as_slice() {
        let lhs = lhs.as_int_value().unwrap();
        let rhs = rhs.as_int_value().unwrap();

        let result = Int::Bin(BinOp::Add, &lhs, &rhs).build(&entry);

        entry.return_value(&result);
    }

    println!("{}", test.to_string());

    let engine = test.create_execution_engine(OptimizationLevel::Aggressive);
    let add = engine.get_fun::<ExtFn<(i64, i64), i64>>("add").unwrap();

    assert_eq!(unsafe { add(5, 4) }, 9);
}
