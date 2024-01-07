set -e
. ./scripts/ci_env.sh
cargo test --verbose -- --nocapture
# check available disk spaces
df -h 
# print current path
echo "print current path"
pwd
# check file sizes on current path
echo "check file sizes on current path"
tree --du -h ~/ -L 3
tree --du -h ~/ -L 2
tree --du -h ~/ -L 1
