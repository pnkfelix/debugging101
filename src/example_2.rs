pub struct Heap(Vec<u64>);

// Let h(j) = \floor{j/2}
// Invariant: K_1, K_2, ... K_n is a heap if:
//     K_h(j) >= K_j for 1 <= h(j) < j <= N
impl Heap {
    
}

pub fn heap_sort(elems: &mut [u64]) {
    let n = elems.len();
    if n <= 1 { return; } // (might be interesting to remove this condition and see what breaks
    let mut record = elems[0];
    let mut l = n/2 + 1;
    let mut r = n;
    let siftup = |elems: &mut [u64], record, l, r| {
        let mut j = l;
        loop {
            let i = j;
            j = 2*j;
            if j > r { elems[i-1] = record; return; }
            if j < r { if elems[j-1] < elems[j] { j = j + 1; } }
            if record >= elems[j-1] { elems[i] = record; return; }
            elems[i-1] = elems[j-1];
        }
    };
    while l > 1 {
        l -= 1;
        record = elems[l-1];
        siftup(elems, record, l, r);
    }
    while r > 1 {
        record = elems[r-1];
        elems[r-1] = elems[0];
        r -= 1;
        siftup(elems, record, l, r);
    }
    elems[0] = record;
}

fn fun_sort(input: &[u64]) -> Vec<u64> {
    let mut v: Vec<_> = input.iter().map(|e|*e).collect();
    heap_sort(&mut v);
    v
}

#[test]
fn heap_sort_basics() {
    assert_eq!(fun_sort(&[1, 3, 5, 7]), vec![1, 3, 5, 7]);
    assert_eq!(fun_sort(&[1, 5, 3, 7]), vec![1, 3, 5, 7]);
}
