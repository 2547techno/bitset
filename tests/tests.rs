use bitset::BitSet;

#[test]
fn test_add() {
    let mut bitset = BitSet::new(6);
    bitset.add(&0).unwrap();
    bitset.add(&2).unwrap();
    bitset.add(&3).unwrap();
    bitset.add(&5).unwrap();
    //println!("{:?}", bitset);
    assert_eq!(bitset.get_data().to_owned(), 0b101101u128);
}

#[test]
fn test_has() {
    let mut bitset = BitSet::new(8);

    bitset.add(&4).unwrap();
    bitset.add(&5).unwrap();
    bitset.add(&7).unwrap();

    assert!(bitset.has(&5).unwrap());
    assert!(!bitset.has(&3).unwrap());
}

#[test]
fn test_remove() {
    let mut bitset = BitSet::new(7);

    bitset.add(&0).unwrap();
    bitset.add(&1).unwrap();
    bitset.add(&2).unwrap();
    bitset.add(&6).unwrap();

    bitset.remove(&1).unwrap();
    bitset.remove(&2).unwrap();

    assert_eq!(bitset.get_data().to_owned(), 0b1000001u128);
}

#[test]
fn test_size() {
    let mut bitset = BitSet::new(5);

    bitset.add(&0).unwrap();
    bitset.add(&1).unwrap();
    bitset.add(&3).unwrap();
    bitset.add(&4).unwrap();
    bitset.remove(&1).unwrap();
    

    assert_eq!(bitset.size(), 3);
}

#[test]
fn test_capacity() {
    let bitset = BitSet::new(65);

    assert_eq!(bitset.capacity(), 65)
}
