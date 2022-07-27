gcc -c csrc/husky_vm.c -o build/husky_vm.o
mkdir -p build/lib/
ar rcs build/lib/libhusky_vm.a build/husky_vm.o
