pub trait SampleIterator {
    type Sample;

    fn next(&mut self) -> Self::Sample;
}
