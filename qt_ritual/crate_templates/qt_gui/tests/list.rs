use qt_gui::{QListOfQStandardItem, QStandardItem};

#[test]
fn list_of_pointers_append() {
    unsafe {
        let list = QListOfQStandardItem::new_0a();

        let item = QStandardItem::new();
        item.set_enabled(true);

        list.resize(list.size() + 1);
        list.index_mut(0).replace(item.as_mut_raw_ptr());

        let item2 = QStandardItem::new();
        item2.set_enabled(false);

        list.resize(list.size() + 1);
        list.index_mut(1).replace(item2.as_mut_raw_ptr());

        assert!((**list.at(0)).is_enabled());
        assert!(!(**list.at(1)).is_enabled());
    }
}
