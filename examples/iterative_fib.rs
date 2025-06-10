use inkwell::OptimizationLevel;
use dorian_ast::backend::Backend;
use dorian_ast::block::builder::BlockBuilder;
use dorian_ast::function::Function;
use dorian_ast::module::Module;
use dorian_ast::{ty, val};
use dorian_llvm::Llvm;

fn main() {
    let fib_function = Function::new("fib")
        .with_return_type(ty::u32())
        .add_param(ty::u32())
        .build_block(build_fib_body);

    let mut module = Module::new("fib_example");
    module.add_function(fib_function);

    let llvm = Llvm::new();
    let compiled_module = llvm.compile_module(&module);
    
    println!("{}", compiled_module.to_string());

    let execution_engine = compiled_module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    let fib_fn = unsafe {
        execution_engine
            .get_function::<unsafe extern "C" fn(u32) -> u32>("fib")
            .unwrap()
    };

    let result = unsafe { fib_fn.call(10) };

    assert_eq!(result, 55, "The 10th Fibonacci number should be 55");
}

fn build_fib_body(scope: &mut BlockBuilder) {
    use val::*;

    let condition = le(arg(0), lit(1i32));
    scope
        .if_then(condition, |scope| {
            scope.ret(arg(0));
        });

    scope.bind("a", lit(0u32));
    scope.bind("b", lit(1u32));
    scope.bind("i", lit(2u32));

    scope.loop_while(le(var("i"), arg(0)), |scope| {
        scope.assign(var("i"), add(var("i"), lit(1u32)));
        scope.bind("c", add(var("a"), var("b")));
        scope.assign(var("a"), var("b"));
        scope.assign(var("b"), var("c"));
    });

    scope.ret(var("b"));
}
