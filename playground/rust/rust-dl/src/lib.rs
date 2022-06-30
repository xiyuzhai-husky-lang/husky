mod try_torch;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use neuroflow::activators::Type::Tanh;
        use neuroflow::data::DataSet;
        use neuroflow::FeedForward;

        /*
            Define neural network with 1 neuron in input layers. Network contains 4 hidden layers.
            And, such as our function returns single value, it is reasonable to have 1 neuron in
            the output layer.
        */
        let mut nn = FeedForward::new(&[1, 7, 8, 8, 7, 1]);

        /*
            Define DataSet.

            DataSet is the Type that significantly simplifies work with neural network.
            Majority of its functionality is still under development :(
        */
        let mut data: DataSet = DataSet::new();
        let mut x = -3.0;

        // Push the data to DataSet (method push accepts two slices: input data and expected output)
        while x <= 2.5 {
            data.push(&[x], &[0.5 * (x.exp().sin()) - (-x.exp()).cos()]);
            x += 0.05;
        }

        // Here, we set necessary parameters and train neural network
        // by our DataSet with 50 000 iterations
        nn.activation(Tanh).learning_rate(0.01).train(&data, 50_000);

        let mut res;

        // Let's check the result
        x = 0.0;
        while x <= 0.3 {
            res = nn.calc(&[x])[0];
            println!(
                "for x = [{:.3}], y = [{:.3}], prediction = [{:.3}]",
                x,
                0.5 * (x.exp().sin()) - (-x.exp()).cos(),
                res
            );
            x += 0.07;
        }
    }
}
