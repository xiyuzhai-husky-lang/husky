use tch::{Device, Tensor};

fn main() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    tch::maybe_init_cuda();
    let t = t * 2;
    println!("cuda_if_available {:?}", Device::cuda_if_available());
    let t = t.to_device(Device::Cuda(0));
    t.print();
}
