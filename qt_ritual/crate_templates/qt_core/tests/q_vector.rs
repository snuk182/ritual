use qt_core::QListOfInt;

#[test]
fn vector1() {
    unsafe {
        let vec = QListOfInt::new_0a();
        vec.resize(vec.size() + 3);
        vec.index_mut(0).replace(1);
        vec.index_mut(1).replace(2);
        vec.index_mut(2).replace(4);
        assert_eq!(vec.count(), 3);
        assert_eq!(*vec.at(0), 1);
        assert_eq!(*vec.at(1), 2);
        assert_eq!(*vec.at(2), 4);
    }
}
