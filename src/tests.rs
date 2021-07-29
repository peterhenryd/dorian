use crate::dorian::Dorian;
use crate::types::kind::{IntData, TypeData, FnData};

#[test]
fn test() {
    let dorian = Dorian::new();

    let mut test = dorian.create_module("test");

    let i8 = IntData::new(8);
    let i8 = i8.create(&dorian);
    let fn_type = FnData::new(vec![i8, i8], i8, false);
    let fn_type = fn_type.create(&dorian);

    let mut function = test.add_fn("main", fn_type)
        .unwrap();

    let mut entry = function.add_block();

    entry;

    println!("{}", test.to_string());
}
