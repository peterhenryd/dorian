# dorian

Dorian is a declarative abstraction for building LLVM programs. 

The primary appeal of Dorian is that it offers you a set of high-level structures to build programs, while also 
separating the primary structure of the program from its representation in LLVM. Dorian does not to be a complete 
replacement for LLVM, nor does it aim to minimize the performance disparity (between its use and the corresponding use 
of LLVM directly) at the cost of simplicity.

Since Dorian uses [Inkwell](https://github.com/TheDan64/inkwell) (a Rust wrapper for LLVM), features that are not
abstracted by Dorian are still available through Inkwell upon compilation.