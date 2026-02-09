#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo ""
echo -e "\e[1m========================================"
echo -e "\e[1mTEST: lilos"
echo -e "\e[1m========================================\e[0m"

if [ ! -d "tests/lilos" ] ; then
   git clone https://github.com/cbiffle/lilos.git tests/lilos
fi

pushd tests/lilos
git fetch --tags
git checkout v1.2.0
popd


cd tests/lilos
./build-all.sh
cd ../..

timeout 5s ./target/release/zmu-armv6m run /workspace/target/thumbv6m-none-eabi/debug/lilos-testsuite-stm32g0 || true
