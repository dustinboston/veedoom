use std::convert::TryFrom;

// TODO: The Diffler

/// Implements the most basic version of Myers diff with tracing
///
/// - The original paper is here http://xmailserver.org/diff2.pdf
/// - With help from https://docs.rs/crate/diffs/0.3.0/source/src/myers.rs
/// - And also https://docs.rs/crate/diffr-lib/0.1.3/source/src/lib.rs
/// - The backtrack fn from here https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/
pub fn shortest_edit(a: &Vec<String>, b: &Vec<String>) -> Vec<(isize, isize, isize, isize)> {
    let n = a.len();
    let m = b.len();
    let max = (m + n) as usize;

    let z = (2 * max + 2) as usize;
    let mut v = vec![0; z];
    let mut t: Vec<Vec<usize>> = vec![];

    for d in 0..max {
        t.push(v.clone());
        let d = to_isize(d);
        for k in (-d..=d).step_by(2) {
            let mut x: usize = if k == -d || k != d && v[i(k - 1, z)] < v[i(k + 1, z)] {
                v[i(k + 1, z)]
            } else {
                v[i(k - 1, z)] + 1
            };
            let mut y = (x as isize - k) as usize;
            while x < n && y < m && a[x] == b[y] {
                x += 1;
                y += 1;
            }
            v[i(k, z)] = x;
            if x >= n && y >= m {
                return backtrack(&t, n, m);
            }
        }
    }
    vec![]
}

pub fn backtrack(
    t: &Vec<Vec<usize>>,
    a_len: usize,
    b_len: usize,
) -> Vec<(isize, isize, isize, isize)> {
    let mut x = a_len.clone() as isize;
    let mut y = b_len.clone() as isize;
    let mut path: Vec<(isize, isize, isize, isize)> = vec![];

    for d in (0..t.len()).rev() {
        let d = to_isize(d);
        let v = &t[d as usize];
        let z = v.len();
        let k = x - y;

        let prev_k = if k == -d || k != d && v[i(k - 1, z)] < v[i(k + 1, z)] {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v[i(prev_k, z)] as isize;
        let prev_y = prev_x - prev_k;

        while x as isize > prev_x && y as isize > prev_y {
            x -= 1;
            y -= 1;
        }

        if d > 0 {
            path.push((prev_x, prev_y, x, y));
        }

        x = prev_x;
        y = prev_y;
    }

    path
}

fn i(a: isize, b: usize) -> usize {
    let b = b as isize;
    (((a % b) + b) % b) as usize
}

fn to_isize(input: usize) -> isize {
    isize::try_from(input).unwrap()
}

