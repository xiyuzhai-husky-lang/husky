set -e
pwd
echo "\n\nls *"
ls *
echo "\n\nls ./"
ls ./
. ./scripts/ci_setup.sh
# cargo test --verbose
echo $LIBTORCH
pwd
