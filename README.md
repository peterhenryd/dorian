# dorian

Dorian is an intuitive high-level abstraction for declaring imperative programs that can be compiled or interpreted 
using [LLVM](https://llvm.org) (via [Inkwell](https://github.com/TheDan64/inkwell)) and 
[Cranelift](https://cranelift.dev).

## Status

Dorian is in an early stage of development. The API is unstable and may change frequently. Moreover, the library is
largely incomplete and unusable for anything other than [experimentation](/recursive_fib.rs).

If you'd like to use Dorian as a Cargo dependency, please do not use the published `dorian` crate from Cargo. 
Instead, add the following to your `Cargo.toml`:

```toml
dorian = { git = "https://github.com/peterhenryd/dorian", features = ["llvm", "cranelift"] }
```

Once Dorian has basic functionality and supports compilation to both LLVM and Cranelift, it will be published to Cargo. 
As of now, the Cranelift backend is not yet implemented, and the LLVM backend is only partially implemented.

Dorian supports LLVM 18 as that is the latest version supported by Inkwell. In the future, support may be added for 
earlier versions of LLVM that are supported by Inkwell.

## License

Dorian is licensed under the [MIT License](LICENSE).
