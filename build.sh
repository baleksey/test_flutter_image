#! /bin/bash
clear
# echo "=== Cleaning flutter ==="
# flutter clean

echo "=== Building Codegen ==="
flutter_rust_bridge_codegen generate

echo "=== Building Flutter for Linux ==="
flutter run -d linux --release