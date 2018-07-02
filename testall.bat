cargo build --release
.\target\release\zmu run .\tests\coremark_v1.0\Debug\Exe\c.out
.\target\release\zmu run .\tests\hello_world\Debug\Exe\hello_world.out
.\target\release\zmu run .\tests\pi\Debug\Exe\c.out
.\target\release\zmu run .\tests\instruction_set\Debug\Exe\c.out

cargo build
.\target\debug\zmu run .\tests\coremark_v1.0\Debug\Exe\c.out
.\target\debug\zmu run .\tests\hello_world\Debug\Exe\hello_world.out
.\target\debug\zmu run .\tests\pi\Debug\Exe\c.out
.\target\debug\zmu run .\tests\instruction_set\Debug\Exe\c.out
