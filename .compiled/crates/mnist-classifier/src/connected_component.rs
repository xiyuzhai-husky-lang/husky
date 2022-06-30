pub struct ConnectedComponent {
    pub(crate) mask: domains::ml::datasets::cv::mnist::BinaryImage28,
}

impl ConnectedComponent {
}

pub(crate) fn horizontal_extend(a: b32, x: b32) -> b32 {
let mut y = a & (x | (x << 1i32) | (x >> 1i32));
let mut z = a & (y | (y << 1i32) | (y >> 1i32));
while z != y {
y = z;
z = a & (y | (y << 1i32) | (y >> 1i32));
}
y
}

pub(crate) fn find_connected_components(img: &domains::ml::datasets::cv::mnist::BinaryImage28) -> Vec<crate::connected_component::ConnectedComponent {
let mut result = Vec<crate::connected_component::ConnectedComponent::__call__();
let mut unsearched = img.clone();
for j in 0..30i32 {
while unsearched[j] != 0 {
let a = unsearched[j];
let shift = a.trailing_zeros();
let mut mask = domains::ml::datasets::cv::mnist::BinaryImage28::__call__();
mask[j] = a(1u32 << shift);
let mut flag = false;
while !flag {
flag = true;
let mut i = j;
while i < 30i32 - 1i32 {
let old_row = mask[i + 1i32];
let new_row = old_row | img[i + 1i32](mask[i]);
if !new_row {
break;
}if old_row != new_row {
flag = false;
mask[i + 1i32] = new_row;
}    i += 1;
}
while i >= j {
let old_row = mask[i];
let new_row = old_row | img[i](mask[i + 1i32]);
if old_row != new_row {
flag = false;
mask[i] = new_row;
}    i -= 1;
}
}
for k in j..30i32 {
unsearched[k] &= (~mask[k]);
}
result.push(crate::connected_component::ConnectedComponent::__call__(mask));
}
}
result
}
