#[derive(PartialEq, Debug)] pub struct FoundAt(pub usize);
#[derive(PartialEq, Debug)] pub struct Unfound { pub insert_at: usize }

// Given K and sorted array [a_0, a_1, ... a_n],
// If returns Ok(i), then exists some i such that a_i == K.
// Otherwise, returns Err(j) such that
//    i < j ==> a_i < K and j <= k ==> K < a_k.
// (i.e., j is the point you would insert K if all a_k were
// first shifted to the right one slot.)
pub fn search(key: u64, elems: &[u64]) -> Result<FoundAt, Unfound>
{
    let mut below = 0_usize;
    let mut limit = elems.len();
    loop {
        assert!(below <= limit);
        if below == limit { return Err(Unfound { insert_at: below }); }
        let mid = (limit + below) / 2;
        let elem = elems[mid];
        if elem == key { return Ok(FoundAt(mid)); }
        if elem < key {
            below = mid + 1;
        } else {
            limit = mid;
        }
    }
}

#[test]
fn search_basic_found() {
    assert_eq!(search(1, &[1, 3, 5, 7]), Ok(FoundAt(0)));
    assert_eq!(search(3, &[1, 3, 5, 7]), Ok(FoundAt(1)));
    assert_eq!(search(5, &[1, 3, 5, 7]), Ok(FoundAt(2)));
    assert_eq!(search(7, &[1, 3, 5, 7]), Ok(FoundAt(3)));
}

#[test]
fn search_basic_unfound() {
    assert_eq!(search(0, &[1, 3, 5, 7]), Err(Unfound { insert_at: 0 }));
    assert_eq!(search(2, &[1, 3, 5, 7]), Err(Unfound { insert_at: 1 }));
    assert_eq!(search(4, &[1, 3, 5, 7]), Err(Unfound { insert_at: 2 }));
    assert_eq!(search(6, &[1, 3, 5, 7]), Err(Unfound { insert_at: 3 }));
    assert_eq!(search(8, &[1, 3, 5, 7]), Err(Unfound { insert_at: 4 }));
}

#[cfg(not_now)]
#[test]
fn search_demo_overflo() {
    const OFLO_LEN: usize = 1 << 31;
    assert_eq!(search(1, &vec![0; OFLO_LEN]), Err(OFLO_LEN));
}
