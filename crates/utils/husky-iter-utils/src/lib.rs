#[macro_export]
macro_rules! chain {
    ($first: expr, $($others: expr),* $(,)?) => {
        $first.into_iter()$(.chain($others))*
    }
}

#[macro_export]
macro_rules! chain_collect {
    ($first: expr, $($others: expr),* $(,)?) => {
        $first.into_iter()$(.chain($others))*.collect()
    }
}

#[macro_export]
macro_rules! chain_as_ref_err_collect {
    ($first: expr, $($others: expr),* $(,)?) => {
        $first.as_ref().err().into_iter()$(.chain($others.as_ref().err()))*.collect()
    }
}
