pub fn get_iterator() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v_iterator = v.iter();

    // the Iterator Trait most notably requires implementation of the .next() associated function. it consumes the iterator, which is why it must be declared mutable, and returns an Option of Some reference to an item in the sequence or None when we've reached the end
    assert_eq!(Some(&1), v_iterator.next());
    assert_eq!(Some(&2), v_iterator.next());
    assert_eq!(Some(&3), v_iterator.next());
    assert_eq!(Some(&4), v_iterator.next());
    assert_eq!(Some(&5), v_iterator.next());
    assert_eq!(None, v_iterator.next());
    assert_eq!(None, v_iterator.next());

    // note: `v_iterator` has been fully consumed and is done
    // nothing will print since we've reached the end with the previous .next() invocations
    for figure in v_iterator {
        println!("{figure}");
    }

    // the for takes ownership of the iterator and makes it mutable
    for item in v.iter() {
        println!("{item}");
    }
}

pub fn mapping() {
    let v = vec!['a', 'b', 'c', 'd'];

    let v_iter = v.iter();

    // .map returns another iterator
    // iterators are lazy, this means nothing defined in the closure has happened until the iterator is consumed.
    let m_iter = v_iter.map(|char| format!("{char}Z"));

    println!("{:?}", v);
    println!("{:?}", m_iter);
    // collect consumes, and can change the collection type that is rebuilt, from the iterator
    println!("{:?}", m_iter.collect::<Vec<String>>());

    let encoded = vec!['g', 'd', 'p', 'p', 'k', 'k', 'n', 'y'];
    let decoded: String = encoded
        .iter()
        .filter(|&c| *c < 'p')
        .map(|c| *c as u8)
        .map(|val| (val + 1) as char)
        .map(|val| val.to_uppercase().to_string())
        .collect();
    println!("encoded message received!: {decoded}");
}
