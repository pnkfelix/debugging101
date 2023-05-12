pub struct Heap(Vec<u64>);

// Let h(j) = \floor{j/2}
// Invariant: K_1, K_2, ... K_n is a heap if:
//     K_h(j) >= K_j for 1 <= h(j) < j <= N
//
// aka
//     K_{2i} <= K_i && K_{2i+1} <= K_i for 1 <= i < 2i+1 <= N
impl Heap {
    
}

pub fn heap_check(elems: &mut [u64], heap_len: usize) {
    assert!(heap_len <= elems.len());
    for j in 2..heap_len {
        let parent = j/2;
        assert!(elems[parent-1] >= elems[j-1], "{:?} broken at {} v {}", &elems[0..heap_len], parent, j);
    }
}

pub fn heap_sort(elems: &mut [u64]) {
    for i in 0..elems.len() {
        heap_insert(elems, i, elems[i]);
    }
    heap_check(elems, elems.len());
    for i in (0..elems.len()).rev() {
        let val = heap_pop(elems, i+1);
        heap_check(elems, i);
        elems[i] = val;
    }
}

pub fn heap_insert(elems: &mut [u64], heap_len: usize, new_elem: u64) {
    heap_check(elems, heap_len);
    assert!(heap_len < elems.len());
    elems[heap_len] = new_elem;
    let mut j = heap_len;
    while j > 0 {
        let i = j / 2;
        if elems[j] > elems[i] {
            elems.swap(j, i);
        }
        j = i;
    }
    heap_check(elems, heap_len + 1);
}

pub fn heap_pop(elems: &mut [u64], heap_len: usize) -> u64 {
    heap_check(elems, heap_len);
    assert!(heap_len > 0, "{heap_len} > 0");
    let ret_val = elems[0];
    if heap_len == 1 { return ret_val; }
    let moving = elems[heap_len-1];
    let mut i = 0;
    loop {
        let lft = 2*(i+1)-1;
        let rgt = 2*(i+1);
        if rgt < heap_len {
            // both children are available.
            // which is larger?
            let j = if elems[lft] > elems[rgt] { lft } else { rgt };
            if elems[j] > moving {
                // Move the larger into the gap (to maintain the heap invariant)
                // and then recur on that new gap.
                elems[i] = elems[j];
                i = j;
            } else {
                // we found the final location!
                elems[i] = moving;
                break;
            }
        } else if lft < heap_len {
            assert_eq!(lft, heap_len-1);
            // only the left is available.
            if elems[lft] > moving {
                elems[i] = elems[lft];
                i = lft;
            } else {
                elems[i] = moving;
                break;
            }
        } else {
            // no children available. `i` must be the final gap.
            elems[i] = moving;
            break;
        }
    }
    heap_check(elems, heap_len - 1);
    return ret_val;
}

pub fn heap_sort_taocp(elems: &mut [u64]) {
    let n = elems.len();
    if n <= 1 { return; }
    let siftup = |elems: &mut [u64], record, l, r| {
        let mut j = l;
        loop {
            let i = j;
            j = 2*(j+1)-1;
            if j > r-1 {
                elems[i] = record;
                return;
            }
            if j < r-1 {
                if elems[j] < elems[j+1] { j = j + 1; }
            }
            if record >= elems[j] {
                elems[i] = record;
                return;
            }
            elems[i] = elems[j];
        }
    };
    for l in (0..(n/2)).rev() {
        siftup(elems, elems[l], l, n);
    }
    for r in (1..n).rev() {
        let record = elems[r];
        elems[r] = elems[0];
        siftup(elems, record, 0, r);
        // if r == 1 { elems[0] = record; }
    }
}

#[cfg(test)]
fn fun_sort(input: &[u64]) -> Vec<u64> {
    let mut v: Vec<_> = input.iter().map(|e|*e).collect();
    heap_sort(&mut v);
    v
}

#[cfg(test)]
mod heap_sort_bigger {
    use super::fun_sort;
    #[test]
    fn bigger_2() {
        assert_eq!(fun_sort(&[15,10]),
                   vec![10,15]);
    }
    #[test]
    fn bigger_5() {
        assert_eq!(fun_sort(&[10,20,30,40,15]),
                   vec![10,15,20,30,40]);
    }
    #[test]
    fn bigger() {
        assert_eq!(fun_sort(&[10,20,30,40,15,
                              50,60,70,80,90]),
                   vec![10,15,20,30,40,
                        50,60,70,80,90]);
    }
}

#[cfg(test)]
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
