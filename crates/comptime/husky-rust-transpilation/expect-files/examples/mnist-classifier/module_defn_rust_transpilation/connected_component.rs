
struct ConnectedComponentDistribution{row_start: i32, row_end: i32, upper_mass: i32, lower_mass: i32}

struct EffHoles{matches: Vec<Option<Leash<RawContour>>>}

pub fn hole_tmpl(ct: Leash<RawContour>) -> Option<f32> {
    let len = v0.contour_len;
    require!(v1 > 4);
    v1 + 0
}

struct ConnectedComponent{mask: BinaryImage28}

pub fn horizontal_extend(a: r32, x: r32) -> r32 {
    let mut y = v0 | (v1 | v1 << 1 | v1 >> 1);
    let mut z = v0 | (v2 | v2 << 1 | v2 >> 1);
    while v3 != v2 {
        v2 = v3;
        v3 = v0 | (v2 | v2 << 1 | v2 >> 1)
    }
    return v2;
}

pub fn find_connected_components(img: BinaryImage28) -> Vec<ConnectedComponent> {
    let mut result: Vec<ConnectedComponent> = vec![];
    let mut unsearched = v0.clone();
    for j in 0..30 {
        while v2[v3] {
            let a = v2[v3];
            let shift = v4.ctz();
            let mut mask = new_zeros();
            v6[v3] = horizontal_extend(v4, 1 << v5);
            let mut flag = false;
            while !v7 {
                v7 = true;
                let mut i = v3;
                // Forext incompleteloop {
                    let old_row = v6[v8 + 1];
                    let new_row = v9 | horizontal_extend(v0[v8 + 1], v6[v8]);
                    if !v10 {
                        break;
                    }
                    if v9 != v10 {
                        v7 = false;
                        v6[v8 + 1] = v10
                    }
                }
                // Forext incompleteloop {
                    let old_row = v6[v8];
                    let new_row = v11 | horizontal_extend(v0[v8], v6[v8 + 1]);
                    if v11 != v12 {
                        v7 = false;
                        v6[v8] = v12
                    }
                }
            }
            for k in v3..30 {
                v2[v13] &= !v6[v13]
            }
            v1.push(ConnectedComponent(v6))
        }
    }
    return v1;
}