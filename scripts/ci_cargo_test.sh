set -e
. ./scripts/ci_env.sh
cargo test --verbose
# check available disk spaces
df -h 
# print current path
echo "print current path"
pwd
# check file sizes on current path
echo "check file sizes on current path"
tree --du -h ./
