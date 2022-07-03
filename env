export TORCH_CUDA_VERSION=cu113

# calculate husky root directory
SOURCE=${BASH_SOURCE[0]}
while [ -L "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR=$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )
  SOURCE=$(readlink "$SOURCE")
  [[ $SOURCE != /* ]] && SOURCE=$DIR/$SOURCE # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
export HUSKY_DIR=$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )

export LIBTORCH="/home/xiyuzhai/Documents/library/libtorch-cxx11-abi-shared-with-deps-1.11.0+cu102/libtorch"
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
