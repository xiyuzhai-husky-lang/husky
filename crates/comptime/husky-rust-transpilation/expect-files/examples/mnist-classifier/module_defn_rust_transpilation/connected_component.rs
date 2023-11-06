
struct ConnectedComponentDistribution{row_start: i32, row_end: i32, upper_mass: i32, lower_mass: i32}

struct EffHoles{matches: Vec<Option<Leash<RawContour>>>}

pub fn hole_tmpl(ct: Leash<RawContour>) -> Option<f32> {
    let len = ct.contour_len;
    require!(len > 4);
    len + 0;
}

struct ConnectedComponent{mask: BinaryImage28}

pub fn horizontal_extend(a: r32, x: r32) -> r32 {
    let y = a | (x | x << 1 | x >> 1);
    let z = a | (y | y << 1 | y >> 1);
    while z != y {
        y = z;
        z = a | (y | y << 1 | y >> 1);
    }
    return y;
}

pub fn find_connected_components(img: BinaryImage28) -> Vec<ConnectedComponent> {
    let result: Vec<ConnectedComponent> = vec![];
    let unsearched = img.clone();
    for {
        while unsearched[j] {
            let a = unsearched[j];
            let shift = a.ctz();
            let mask = new_zeros();
            mask[j] = horizontal_extend(a, 1 << shift);
            let flag = false;
            while !flag {
                flag = true;
                let i = j;
                for {
                    let old_row = mask[i + 1];
                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i]);
                    if !new_row {
                        break;
                    }
                    if old_row != new_row {
                        flag = false;
                        mask[i + 1] = new_row;
                    }
                }
                for {
                    let old_row = mask[i];
                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1]);
                    if old_row != new_row {
                        flag = false;
                        mask[i] = new_row;
                    }
                }
            }
            for {
                unsearched[k] &= !mask[k];
            }
            result.push(ConnectedComponent(mask));
        }
    }
    return result;
}