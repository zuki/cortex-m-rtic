target extended-remote :3333

# print demangled symbols
set print asm-demangle on
set print pretty on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break HardFault
#break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
#break main

#break sdmmc.rs:396
#break types.rs:27

monitor arm semihosting enable

load

continue
