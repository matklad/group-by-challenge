pub fn group_by<I, F, K, T>(xs: I, key_fn: F) -> Vec<(K, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> K,
    K: Eq,
{
    let mut res = Vec::new();
    let mut xs = xs.into_iter();

    let mut key = None;
    let mut group = Vec::new();
    loop {
        let mut next_key = None;
        let mut next_group = Vec::new();
        for x in xs.by_ref() {
            let k = key_fn(&x);
            if key.as_ref() != Some(&k) {
                next_key = Some(k);
                next_group = vec![x];
                break;
            }
            group.push(x)
        }

        if let Some(key) = key {
            res.push((key, group));
        }
        key = next_key;
        group = next_group;
        if key.is_none() {
            break;
        }
    }
    res
}

#[test]
fn tests() {
    assert_eq!(
        group_by(0..5, |&x| x % 3 == 0),
        vec![
            (true, vec![0]),
            (false, vec![1, 2]),
            (true, vec![3]),
            (false, vec![4])
        ],
    );
    assert_eq!(
        group_by(0..5, |&x| x % 3 == 1),
        vec![
            (false, vec![0]),
            (true, vec![1]),
            (false, vec![2, 3]),
            (true, vec![4])
        ],
    );
    assert_eq!(
        group_by(0..5, |&x| x % 3 == 0),
        vec![
            (true, vec![0]),
            (false, vec![1, 2]),
            (true, vec![3]),
            (false, vec![4])
        ],
    );

    assert_eq!(
        group_by(0..5, |&x| x),
        vec![
            (0, vec![0]),
            (1, vec![1]),
            (2, vec![2]),
            (3, vec![3]),
            (4, vec![4])
        ],
    );

    assert_eq!(group_by(0..5, |_| ()), vec![((), vec![0, 1, 2, 3, 4])]);

    assert_eq!(group_by(0..0, |_| ()), vec![]);
}
