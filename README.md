# LLVM to Cairo Translator

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build Status](https://github.com/LucasLvy/llvm-to-cairo/actions/workflows/test.yml/badge.svg)
![Issues](https://img.shields.io/github/issues/LucasLvy/llvm-to-cairo)

[![Exploration_Team](https://img.shields.io/badge/Exploration_Team-29296E.svg?&style=for-the-badge&logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz48c3ZnIGlkPSJhIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxODEgMTgxIj48ZGVmcz48c3R5bGU+LmJ7ZmlsbDojZmZmO308L3N0eWxlPjwvZGVmcz48cGF0aCBjbGFzcz0iYiIgZD0iTTE3Ni43Niw4OC4xOGwtMzYtMzcuNDNjLTEuMzMtMS40OC0zLjQxLTIuMDQtNS4zMS0xLjQybC0xMC42MiwyLjk4LTEyLjk1LDMuNjNoLjc4YzUuMTQtNC41Nyw5LjktOS41NSwxNC4yNS0xNC44OSwxLjY4LTEuNjgsMS44MS0yLjcyLDAtNC4yN0w5Mi40NSwuNzZxLTEuOTQtMS4wNC00LjAxLC4xM2MtMTIuMDQsMTIuNDMtMjMuODMsMjQuNzQtMzYsMzcuNjktMS4yLDEuNDUtMS41LDMuNDQtLjc4LDUuMThsNC4yNywxNi41OGMwLDIuNzIsMS40Miw1LjU3LDIuMDcsOC4yOS00LjczLTUuNjEtOS43NC0xMC45Ny0xNS4wMi0xNi4wNi0xLjY4LTEuODEtMi41OS0xLjgxLTQuNCwwTDQuMzksODguMDVjLTEuNjgsMi4zMy0xLjgxLDIuMzMsMCw0LjUzbDM1Ljg3LDM3LjNjMS4zNiwxLjUzLDMuNSwyLjEsNS40NCwxLjQybDExLjQtMy4xMSwxMi45NS0zLjYzdi45MWMtNS4yOSw0LjE3LTEwLjIyLDguNzYtMTQuNzYsMTMuNzNxLTMuNjMsMi45OC0uNzgsNS4zMWwzMy40MSwzNC44NGMyLjIsMi4yLDIuOTgsMi4yLDUuMTgsMGwzNS40OC0zNy4xN2MxLjU5LTEuMzgsMi4xNi0zLjYsMS40Mi01LjU3LTEuNjgtNi4wOS0zLjI0LTEyLjMtNC43OS0xOC4zOS0uNzQtMi4yNy0xLjIyLTQuNjItMS40Mi02Ljk5LDQuMyw1LjkzLDkuMDcsMTEuNTIsMTQuMjUsMTYuNzEsMS42OCwxLjY4LDIuNzIsMS42OCw0LjQsMGwzNC4zMi0zNS43NHExLjU1LTEuODEsMC00LjAxWm0tNzIuMjYsMTUuMTVjLTMuMTEtLjc4LTYuMDktMS41NS05LjE5LTIuNTktMS43OC0uMzQtMy42MSwuMy00Ljc5LDEuNjhsLTEyLjk1LDEzLjg2Yy0uNzYsLjg1LTEuNDUsMS43Ni0yLjA3LDIuNzJoLS42NWMxLjMtNS4zMSwyLjcyLTEwLjYyLDQuMDEtMTUuOGwxLjY4LTYuNzNjLjg0LTIuMTgsLjE1LTQuNjUtMS42OC02LjA5bC0xMi45NS0xNC4xMmMtLjY0LS40NS0xLjE0LTEuMDgtMS40Mi0xLjgxbDE5LjA0LDUuMTgsMi41OSwuNzhjMi4wNCwuNzYsNC4zMywuMTQsNS43LTEuNTVsMTIuOTUtMTQuMzhzLjc4LTEuMDQsMS42OC0xLjE3Yy0xLjgxLDYuNi0yLjk4LDE0LjEyLTUuNDQsMjAuNDYtMS4wOCwyLjk2LS4wOCw2LjI4LDIuNDYsOC4xNiw0LjI3LDQuMTQsOC4yOSw4LjU1LDEyLjk1LDEyLjk1LDAsMCwxLjMsLjkxLDEuNDIsMi4wN2wtMTMuMzQtMy42M1oiLz48L3N2Zz4=)](https://github.com/keep-starknet-strange)

## Table of Contents

- [About](#about)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## About

**LLVM to Cairo** is a tool that translates LLVM Intermediate Representation (IR) to Cairo, a Turing-complete language
designed for creating provable programs for general computation.

This tool would allow you to prove any language that compiles to llvm.

## Design choice

Why not translate LLVM IR to Sierra ?

Well this cairo code:

```rs
pub fn fib(a: u32, b: u32, n: u32) -> u32 {
    if n == 0 {
        b
    } else {
        fib(b, a + b, n - 1)
    }
}
```

generates this Sierra:

```ts
type RangeCheck = RangeCheck [storable: true, drop: false, dup: false, zero_sized: false];
type Const<felt252, 375233589013918064796019> = Const<felt252, 375233589013918064796019> [storable: false, drop: false, dup: false, zero_sized: false];
type u32 = u32 [storable: true, drop: true, dup: true, zero_sized: false];
type Tuple<u32> = Struct<ut@Tuple, u32> [storable: true, drop: true, dup: true, zero_sized: false];
type Const<felt252, 155785504323917466144735657540098748279> = Const<felt252, 155785504323917466144735657540098748279> [storable: false, drop: false, dup: false, zero_sized: false];
type core::panics::Panic = Struct<ut@core::panics::Panic> [storable: true, drop: true, dup: true, zero_sized: true];
type Array<felt252> = Array<felt252> [storable: true, drop: true, dup: false, zero_sized: false];
type Tuple<core::panics::Panic, Array<felt252>> = Struct<ut@Tuple, core::panics::Panic, Array<felt252>> [storable: true, drop: true, dup: false, zero_sized: false];
type Const<felt252, 155785504329508738615720351733824384887> = Const<felt252, 155785504329508738615720351733824384887> [storable: false, drop: false, dup: false, zero_sized: false];
type felt252 = felt252 [storable: true, drop: true, dup: true, zero_sized: false];
type core::panics::PanicResult::<(core::integer::u32,)> = Enum<ut@core::panics::PanicResult::<(core::integer::u32,)>, Tuple<u32>, Tuple<core::panics::Panic, Array<felt252>>> [storable: true, drop: true, dup: false, zero_sized: false];
type Const<u32, 1> = Const<u32, 1> [storable: false, drop: false, dup: false, zero_sized: false];
type Const<u32, 0> = Const<u32, 0> [storable: false, drop: false, dup: false, zero_sized: false];
type GasBuiltin = GasBuiltin [storable: true, drop: false, dup: false, zero_sized: false];

libfunc disable_ap_tracking = disable_ap_tracking;
libfunc withdraw_gas = withdraw_gas;
libfunc branch_align = branch_align;
libfunc const_as_immediate<Const<u32, 0>> = const_as_immediate<Const<u32, 0>>;
libfunc dup<u32> = dup<u32>;
libfunc store_temp<RangeCheck> = store_temp<RangeCheck>;
libfunc u32_eq = u32_eq;
libfunc u32_overflowing_add = u32_overflowing_add;
libfunc const_as_immediate<Const<u32, 1>> = const_as_immediate<Const<u32, 1>>;
libfunc store_temp<u32> = store_temp<u32>;
libfunc u32_overflowing_sub = u32_overflowing_sub;
libfunc store_temp<GasBuiltin> = store_temp<GasBuiltin>;
libfunc function_call<user@fib::fib::fib> = function_call<user@fib::fib::fib>;
libfunc drop<u32> = drop<u32>;
libfunc array_new<felt252> = array_new<felt252>;
libfunc const_as_immediate<Const<felt252, 155785504329508738615720351733824384887>> = const_as_immediate<Const<felt252, 155785504329508738615720351733824384887>>;
libfunc store_temp<felt252> = store_temp<felt252>;
libfunc array_append<felt252> = array_append<felt252>;
libfunc struct_construct<core::panics::Panic> = struct_construct<core::panics::Panic>;
libfunc struct_construct<Tuple<core::panics::Panic, Array<felt252>>> = struct_construct<Tuple<core::panics::Panic, Array<felt252>>>;
libfunc enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 1> = enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 1>;
libfunc store_temp<core::panics::PanicResult::<(core::integer::u32,)>> = store_temp<core::panics::PanicResult::<(core::integer::u32,)>>;
libfunc const_as_immediate<Const<felt252, 155785504323917466144735657540098748279>> = const_as_immediate<Const<felt252, 155785504323917466144735657540098748279>>;
libfunc struct_construct<Tuple<u32>> = struct_construct<Tuple<u32>>;
libfunc enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 0> = enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 0>;
libfunc const_as_immediate<Const<felt252, 375233589013918064796019>> = const_as_immediate<Const<felt252, 375233589013918064796019>>;

disable_ap_tracking() -> (); // 0
withdraw_gas([0], [1]) { fallthrough([5], [6]) 61([7], [8]) }; // 1
branch_align() -> (); // 2
const_as_immediate<Const<u32, 0>>() -> ([9]); // 3
dup<u32>([4]) -> ([4], [10]); // 4
store_temp<RangeCheck>([5]) -> ([5]); // 5
u32_eq([10], [9]) { fallthrough() 52() }; // 6
branch_align() -> (); // 7
dup<u32>([3]) -> ([3], [11]); // 8
u32_overflowing_add([5], [2], [11]) { fallthrough([12], [13]) 37([14], [15]) }; // 9
branch_align() -> (); // 10
const_as_immediate<Const<u32, 1>>() -> ([16]); // 11
store_temp<u32>([16]) -> ([16]); // 12
u32_overflowing_sub([12], [4], [16]) { fallthrough([17], [18]) 22([19], [20]) }; // 13
branch_align() -> (); // 14
store_temp<RangeCheck>([17]) -> ([17]); // 15
store_temp<GasBuiltin>([6]) -> ([6]); // 16
store_temp<u32>([3]) -> ([3]); // 17
store_temp<u32>([13]) -> ([13]); // 18
store_temp<u32>([18]) -> ([18]); // 19
function_call<user@fib::fib::fib>([17], [6], [3], [13], [18]) -> ([21], [22], [23]); // 20
return([21], [22], [23]); // 21
branch_align() -> (); // 22
drop<u32>([20]) -> (); // 23
drop<u32>([13]) -> (); // 24
drop<u32>([3]) -> (); // 25
array_new<felt252>() -> ([24]); // 26
const_as_immediate<Const<felt252, 155785504329508738615720351733824384887>>() -> ([25]); // 27
store_temp<felt252>([25]) -> ([25]); // 28
array_append<felt252>([24], [25]) -> ([26]); // 29
struct_construct<core::panics::Panic>() -> ([27]); // 30
struct_construct<Tuple<core::panics::Panic, Array<felt252>>>([27], [26]) -> ([28]); // 31
enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 1>([28]) -> ([29]); // 32
store_temp<RangeCheck>([19]) -> ([19]); // 33
store_temp<GasBuiltin>([6]) -> ([6]); // 34
store_temp<core::panics::PanicResult::<(core::integer::u32,)>>([29]) -> ([29]); // 35
return([19], [6], [29]); // 36
branch_align() -> (); // 37
drop<u32>([15]) -> (); // 38
drop<u32>([4]) -> (); // 39
drop<u32>([3]) -> (); // 40
array_new<felt252>() -> ([30]); // 41
const_as_immediate<Const<felt252, 155785504323917466144735657540098748279>>() -> ([31]); // 42
store_temp<felt252>([31]) -> ([31]); // 43
array_append<felt252>([30], [31]) -> ([32]); // 44
struct_construct<core::panics::Panic>() -> ([33]); // 45
struct_construct<Tuple<core::panics::Panic, Array<felt252>>>([33], [32]) -> ([34]); // 46
enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 1>([34]) -> ([35]); // 47
store_temp<RangeCheck>([14]) -> ([14]); // 48
store_temp<GasBuiltin>([6]) -> ([6]); // 49
store_temp<core::panics::PanicResult::<(core::integer::u32,)>>([35]) -> ([35]); // 50
return([14], [6], [35]); // 51
branch_align() -> (); // 52
drop<u32>([4]) -> (); // 53
drop<u32>([2]) -> (); // 54
struct_construct<Tuple<u32>>([3]) -> ([36]); // 55
enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 0>([36]) -> ([37]); // 56
store_temp<RangeCheck>([5]) -> ([5]); // 57
store_temp<GasBuiltin>([6]) -> ([6]); // 58
store_temp<core::panics::PanicResult::<(core::integer::u32,)>>([37]) -> ([37]); // 59
return([5], [6], [37]); // 60
branch_align() -> (); // 61
drop<u32>([2]) -> (); // 62
drop<u32>([4]) -> (); // 63
drop<u32>([3]) -> (); // 64
array_new<felt252>() -> ([38]); // 65
const_as_immediate<Const<felt252, 375233589013918064796019>>() -> ([39]); // 66
store_temp<felt252>([39]) -> ([39]); // 67
array_append<felt252>([38], [39]) -> ([40]); // 68
struct_construct<core::panics::Panic>() -> ([41]); // 69
struct_construct<Tuple<core::panics::Panic, Array<felt252>>>([41], [40]) -> ([42]); // 70
enum_init<core::panics::PanicResult::<(core::integer::u32,)>, 1>([42]) -> ([43]); // 71
store_temp<RangeCheck>([7]) -> ([7]); // 72
store_temp<GasBuiltin>([8]) -> ([8]); // 73
store_temp<core::panics::PanicResult::<(core::integer::u32,)>>([43]) -> ([43]); // 74
return([7], [8], [43]); // 75

fib::fib::fib@0([0]: RangeCheck, [1]: GasBuiltin, [2]: u32, [3]: u32, [4]: u32) -> (RangeCheck, GasBuiltin, core::panics::PanicResult::<(core::integer::u32,)>);
```

As you can see if we chose to emit sierra we would need to take care of a lot of annoying things that are automatically
handled by the cairo compiler such as gas metering, panics etc.

## Installation

To install the `llvm-to-cairo` tool, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/LucasLvy/llvm-to-cairo.git
   cd llvm-to-cairo
   ```

2. Install [LLVM 18](https://apt.llvm.org/) if not already installed

On macos:

```sh
brew install llvm-18
```

3. Export the prefix path of the installation

If you installed with brew:

```sh
export LLVM_SYS_180_PREFIX=$(brew --prefix)/opt/llvm@18
```

4. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

To use the `llvm-to-cairo` tool, you can run the following command:

```sh
./compile_to_llvm.sh <name_of_your_file>
```

This script compiles the given Rust file located in `examples/<name_of_your_file>/<name_of_your_file>.rs` to LLVM IR and
saves the output in the `examples/<name_of_your_file>/<name_of_your_file>.ll` directory with a `.ll` extension.

Then compile it with the binary that doesn't exist yet. Later this will be abstracted by a cli and you'll just need to
provide the path to the rust file.

### Example

1. Create a Rust file `examples/fib/fib.rs` with the following content:

   ```rust
   #[no_mangle]
   pub fn fib(a: u32, b: u32, n: u32) -> u32 {
       if n == 0 {
           b
       } else {
           fib(b, a + b, n - 1)
       }
   }
   ```

2. Run the translation script:

   ```sh
   ./compile_to_llvm.sh fib
   ```

3. The LLVM IR will be saved in `examples/fib/fib.ll`.

4. Then call the `compile` method with the path to the `fib.ll` file. Currently just run:

```sh
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! To contribute to the project.
