use crate::dorian::Dorian;
use crate::types::int::IntData;
use crate::types::function::FnData;
use crate::types::data::TypeData;
use crate::value::Value;
use crate::value::int::{Int, BinOp};
use crate::value::data::BuildValue;

#[test]
fn test() {
    let dorian = Dorian::new();

    let mut test = dorian.create_module("test");

    let i8 = IntData::new(8)
        .create(&dorian);
    let fn_type = FnData::new(vec![&i8, &i8], &i8, false)
        .create(&dorian);

    let mut function = test.add_fn("add", &fn_type);

    let mut entry = function.add_block();
    if let [lhs, rhs] = function.fetch_params().as_slice() {
        let lhs = lhs.as_int_value().unwrap();
        let rhs = rhs.as_int_value().unwrap();

        let result = Int::Bin(BinOp::Add, &lhs, &rhs)
            .build(&entry);

        entry.return_value(&result);
    }

    println!("{}", test.to_string());
}
