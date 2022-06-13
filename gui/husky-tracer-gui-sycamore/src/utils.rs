#[macro_use]
macro_rules! memo {
    ($scope: ident, $lambda: expr) => {{
        create_memo($scope, move ||$lambda)
    }};
    ($scope: ident, $lambda: expr, $first_dependency: ident) => {{
        let $first_dependency = $first_dependency.clone();
        memo!($scope, $lambda)
    }};
    ($scope: ident, $lambda: expr, $first_dependency: ident, $($other_dependencies: ident),*) => {{
        let $first_dependency = $first_dependency.clone();
        memo!($scope, $lambda, $($other_dependencies),*)
    }};
}

pub(crate) use memo;
