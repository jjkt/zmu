#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo ""
echo -e "\e[1m========================================"
echo -e "\e[1mTEST: Rustbook examples"
echo -e "\e[1m========================================\e[0m"
if [ ! -d "tests/rustbook" ] ; then
   git clone https://github.com/rust-embedded/cortex-m-quickstart tests/rustbook
else
    cd "tests/rustbook"
    git pull https://github.com/rust-embedded/cortex-m-quickstart
    cd ..
    cd ..
fi

sed -i 's/{{authors}}/jjkt/g' tests/rustbook/Cargo.toml
sed -i 's/{{project-name}}/zmu-tests/g' tests/rustbook/Cargo.toml


cd tests/rustbook

declare -a arr=("panic" "test_on_host" "hello" "exception" "itm"  "crash")

for i in "${arr[@]}"
do
   echo "cargo build --example $i"
   cargo build --example $i
done

cd ../..

for i in "${arr[@]}"
do
   echo "./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/$i"
   timeout 1s ./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/$i || true
   echo ""
done
