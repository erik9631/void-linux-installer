# GDB actually does this automatically on Unix-like systems through `rust-gdb`, which is a wrapper script that sets up all these pretty printers. 
# The issue is specifically on Windows with the GNU toolchain (`x86_64-pc-windows-gnu`), where `rust-gdb` isn't available. As we saw earlier when you tried to run `rust-gdb.exe`, you got the error message saying it's not available for this toolchain.


import gdb
import sys
import os
import platform

# Get the Rust toolchain path in a platform-agnostic way
home = os.path.expanduser("~")
rust_path = f"{home}/.rustup/toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/etc"
sys.path.insert(0, rust_path)
import gdb_lookup

def register_rust_pretty_on_new_objfile(objfile):
    print("new objfile", objfile.new_objfile)
    gdb_lookup.register_printers(objfile.new_objfile)

gdb.events.new_objfile.connect(register_rust_pretty_on_new_objfile)