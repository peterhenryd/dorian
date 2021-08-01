use crate::dorian::Dorian;
use crate::llvm::execution_engine::ExtFn;
use crate::llvm::{OptimizationLevel, AddressSpace};
use crate::types::fun::FunData;
use crate::types::int::IntData;
use crate::value::data::BuildValue;
use crate::value::Value;
use crate::types::ptr::PtrData;
use crate::value::int::IntValue;
use crate::types::CreateType;

#[test]
fn test() {
    let dorian = Dorian::new();

    let mut test = dorian.create_module("test");

    let i64 = IntData::Bits(64).create(&dorian);
    let i64_ptr = PtrData::of(i64, AddressSpace::Generic)
        .create(&dorian);

    let fun_type = FunData::new(vec![&i64_ptr], &i64, false).create(&dorian);

    let mut fun = test.add_fn("deref_int", &fun_type);

    let mut entry = fun.add_block("entry");
    if let [val] = fun.fetch_params().as_slice() {
        let val = val.as_ptr_value::<IntValue>().unwrap();

        let result = val.deref().build(&entry);

        entry.return_value(&result);
    }

    println!("{}", test.to_string());

    let engine = test.create_execution_engine(OptimizationLevel::Aggressive);
    let add = engine.get_fun::<ExtFn<(*mut i64,), i64>>("deref_int").unwrap();

    assert_eq!(unsafe { add(&5i64 as *const _ as *mut _) }, 5);
}