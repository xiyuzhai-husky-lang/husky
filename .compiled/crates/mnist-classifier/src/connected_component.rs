use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectedComponent {
    pub(crate) mask: domains::ml::datasets::cv::mnist::BinaryImage28,
}

impl ConnectedComponent {
    pub(crate) fn __call__(mask: domains::ml::datasets::cv::mnist::BinaryImage28) -> Self {
        Self { mask }
    }
}

impl __HasStaticTypeInfo for ConnectedComponent {
    type StaticSelf = ConnectedComponent;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::connected_component::ConnectedComponent".into()
    }
}

impl<'eval> __AnyValue<'eval> for ConnectedComponent {
    fn print_short(&self) -> String {
        todo!()
    }

    fn to_json_value(&self) -> __JsonValue {
        todo!()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {
        todo!()
    }

    fn static_ty() -> __EntityRoutePtr {
        __ty_route_from_static_binded::<Self>("mnist_classifier::connected_component::ConnectedComponent")
    }
}

pub(crate) fn horizontal_extend(a: u32, x: u32) -> u32 {
    let mut y = a & (x | (x << 1) | (x >> 1));
    let mut z = a & (y | (y << 1) | (y >> 1));
    while z != y {
        y = z;
        z = a & (y | (y << 1) | (y >> 1));
    }
    return y
}

pub(crate) fn find_connected_components(img: &domains::ml::datasets::cv::mnist::BinaryImage28) -> Vec<ConnectedComponent> {
    let mut result = Vec::<ConnectedComponent>::__call__();
    let mut unsearched = img.clone();
    for j in 0..30 {
        while unsearched[(j) as usize] != 0 {
            let a = unsearched[(j) as usize];
            let shift = a.ctz();
            let mut mask = domains::ml::datasets::cv::mnist::BinaryImage28::__call__();
            mask[(j) as usize] = horizontal_extend(a, 1u32 << shift);
            let mut flag = false;
            while !flag {
                flag = true;
                let mut i = j;
                while i < 30 - 1 {
                    let old_row = mask[(i + 1) as usize];
                    let new_row = old_row | horizontal_extend(img[(i + 1) as usize], mask[(i) as usize]);
                    if (0 == new_row) {
                        break;
                    }
                    if old_row != new_row {
                        flag = false;
                        mask[(i + 1) as usize] = new_row;
                    }
                    i += 1;
                }
                while i >= j {
                    let old_row = mask[(i) as usize];
                    let new_row = old_row | horizontal_extend(img[(i) as usize], mask[(i + 1) as usize]);
                    if old_row != new_row {
                        flag = false;
                        mask[(i) as usize] = new_row;
                    }
                    i -= 1;
                }
            }
            for k in j..30 {
                unsearched[(k) as usize] &= (!mask[(k) as usize]);
            }
            result.push(ConnectedComponent::__call__(mask));
        }
    }
    return result
}
