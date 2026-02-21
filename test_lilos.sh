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
./build-all.sh
popd


echo "armv6m: testsuite-stm32g0"
timeout 5s ./target/release/zmu-armv6m run tests/lilos/target/thumbv6m-none-eabi/debug/lilos-testsuite-stm32g0 || true
echo "armv7m: testsuite-lm3s6965"
timeout 5s ./target/release/zmu-armv7m run tests/lilos/target/thumbv7m-none-eabi/debug/lilos-testsuite-lm3s6965 || true
