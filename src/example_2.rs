pub struct Heap(Vec<u64>);

// Let h(j) = \floor{j/2}
// Invariant: K_1, K_2, ... K_n is a heap if:
//     K_h(j) >= K_j for 1 <= h(j) < j <= N
impl Heap {
    
}

pub fn heap_sort(elems: &mut [u64]) {
    let n = elems.len();
    if n <= 1 { return; } // (might be interesting to remove this condition and see what breaks
    let siftup = |elems: &mut [u64], record, l, r| {
        let mut j = l+1;
        loop {
            let i = j-1;
            j = 2*j;
            if j > r { elems[i] = record; return; }
            if j < r { if elems[j-1] < elems[j] { j = j + 1; } }
            if record >= elems[j-1] { elems[i] = record; return; }
            elems[i] = elems[j-1];
        }
    };
    for l in (0..(n/2)).rev() {
        siftup(elems, elems[l], l, n);
    }
    for r in (1..n).rev() {
        let record = elems[r];
        elems[r] = elems[0];
        siftup(elems, record, 0, r);
        if r == 1 { elems[0] = record; }
    }
}

fn fun_sort(input: &[u64]) -> Vec<u64> {
    let mut v: Vec<_> = input.iter().map(|e|*e).collect();
    heap_sort(&mut v);
    v
}

macro_rules! instance {
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            assert_eq!(fun_sort(& $input), vec![1, 3, 5, 7]);
        }
    }
}

#[cfg(test)]
mod heap_sort_basics {
    use super::fun_sort;

    instance!(i1357, [1, 3, 5, 7]);
    instance!(i3157, [3, 1, 5, 7]);
    instance!(i3517, [3, 5, 1, 7]);
    instance!(i3571, [3, 5, 7, 1]);

    instance!(i1537, [1, 5, 3, 7]);
    instance!(i5137, [5, 1, 3, 7]);
    instance!(i5317, [5, 3, 1, 7]);
    instance!(i5371, [5, 3, 7, 1]);

    instance!(i1573, [1, 5, 7, 3]);
    instance!(i5173, [5, 1, 7, 3]);
    instance!(i5713, [5, 7, 1, 3]);
    instance!(i5731, [5, 7, 3, 1]);

    instance!(i1375, [1, 3, 7, 5]);
    instance!(i3175, [3, 1, 7, 5]);
    instance!(i3715, [3, 7, 1, 5]);
    instance!(i3751, [3, 7, 5, 1]);

    instance!(i1735, [1, 7, 3, 5]);
    instance!(i7135, [7, 1, 3, 5]);
    instance!(i7315, [7, 3, 1, 5]);
    instance!(i7351, [7, 3, 5, 1]);

    instance!(i1753, [1, 7, 5, 3]);
    instance!(i7153, [7, 1, 5, 3]);
    instance!(i7513, [7, 5, 1, 3]);
    instance!(i7531, [7, 5, 3, 1]);
}
