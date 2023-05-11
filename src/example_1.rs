type Idx = u8;

// Given K and sorted array [a_0, a_1, ... a_n],
// If returns Ok(i), then exists some i such that a_i == K.
// Otherwise, returns Err(j) such that
//    i < j ==> a_i < K and j <= k ==> K < a_k.
// (i.e., j is the point you would insert K if all a_k were
// first shifted to the right one slot.)
fn search(key: u64, elems: &[u64]) -> Result<Idx, Idx>
{
    let mut below: Idx = 0;
    let mut limit: Idx = elems.len() as Idx;
    loop {
	if below == limit { return Err(below); }
	assert!(below < limit);
	let mid = below + (limit - below) / 2;
	assert!(0 <= mid && (mid as usize) < elems.len());
	let elem = elems[mid as usize];
	if elem == key { return Ok(mid); }
	if elem < key { below = mid+1; }
	else { assert!(key < elem); limit = mid; }
    }
}

#[test]
fn search_basic() {
    assert_eq!(search(1, &[1, 3, 5, 7]), Ok(0));
    assert_eq!(search(3, &[1, 3, 5, 7]), Ok(1));
    assert_eq!(search(5, &[1, 3, 5, 7]), Ok(2));
    assert_eq!(search(7, &[1, 3, 5, 7]), Ok(3));

    assert_eq!(search(0, &[1, 3, 5, 7]), Err(0));
    assert_eq!(search(2, &[1, 3, 5, 7]), Err(1));
    assert_eq!(search(4, &[1, 3, 5, 7]), Err(2));
    assert_eq!(search(6, &[1, 3, 5, 7]), Err(3));
    assert_eq!(search(8, &[1, 3, 5, 7]), Err(4));
}

#[test]
fn search_demo_overflo() {
    assert_eq!(search(1, &vec![0; 200]), Err(200));
}
