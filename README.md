# Sand Hardware Description Framework
Sand is a hardware description framework that aims to accelerate hybrid hardware-software development for embedded systems.
By leveraging the Rust language, it allows for high quality and performant simulation software.
Rust's powerful macro system is used to enable writing modules in a synthesizable domain-specific language (DSL).

This framework is based on LLHD, allowing us to generate a synthesizable intermediate representation (IR) that is easy to simulate.
The DSL has all the relevant characteristics of a competent HDL, with the concept of timing and various types of descriptions, such as dataflow, behavioral and structural.
Because the DSL is essentially a modified subset of Rust, it can be implemented as software as well.
The framework is therefore able to generate LLVM IR and LLHD IR, on which we can estimate runtime costs independent of inter-module communication costs.
These comparative estimations allow automatic partitioning of the modules across the software-hardware boundary.

## Hardware Description DSL
I believe hardware description is a niche domain, therefore a domain-specific language is better adapted for this task.
In the majority of non Intel/AMD/Nvidia use cases, hardware (FPGA or ASIC) is simply a way to accelerate a heavy computation in the context of an embedded software application.
Developing a framework that blurs the line between software and hardware is the current industry trend targeting high performance, high efficiency embedded applications.
With this in mind, the DSL we will use has all the better adapted paradigms for hardware description, with no explicit concept of central memory.
We also want the language to feel wholesome and correct, kind of like Rust.

Here are some goals and philosophies for the DSL:
- Make synthesizable code obviously different from non synthesizable code
- There should be only one obvious way to do something
- Short circuits and other electrical errors should be compilation errors
- Every synthesizable unit is a function where the arguments are the input or inout ports and the named return values are the output ports (root level function signature may be implicitly defined by the design tool)
- Writing to inout ports must be protected (preferably provably)
- Logic should be rich (9 values of VHDL)
- Types should be logic based and composable (structures are nice)
- Testing, simulation and verification is a priority
- Packaging is a priority
- Adapting to different types of hardware should only involve changing configuration values.
- Type inference
