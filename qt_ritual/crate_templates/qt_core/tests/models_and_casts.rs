use qt_core::{
    ItemDataRole, QAbstractItemModel, QAbstractTableModel, QListOfQString, QString,
    QStringListModel, QAnyStringView
};

#[test]
fn models_and_casts() {
    unsafe {
        let string_list = QListOfQString::new_0a();
        string_list.resize(string_list.size() + 2);
        string_list.index_mut(0).assign_1a(&QAnyStringView::from_q_string(&QString::from_std_str("text1")));
        string_list.index_mut(1).assign_1a(&QAnyStringView::from_q_string(&QString::from_std_str("text2")));
        let string_list_model = QStringListModel::from_q_list_of_q_string(&string_list);
        assert_eq!(string_list_model.row_count_0a(), 2);

        let index0 = string_list_model.index_2a(0, 0);
        assert_eq!(
            string_list_model
                .data_2a(&index0, ItemDataRole::DisplayRole.to_int())
                .to_string()
                .to_std_string(),
            "text1"
        );

        let index1 = string_list_model.index_2a(1, 0);
        assert_eq!(
            string_list_model
                .data_2a(&index1, ItemDataRole::DisplayRole.to_int())
                .to_string()
                .to_std_string(),
            "text2"
        );

        let abstract_model = string_list_model.static_upcast::<QAbstractItemModel>();
        assert_eq!(abstract_model.row_count_0a(), 2);

        let string_list_model_back = abstract_model.dynamic_cast::<QStringListModel>();
        assert!(
            !string_list_model_back.is_null(),
            "dynamic_cast should be successful"
        );
        assert_eq!(string_list_model_back.row_count_0a(), 2);

        assert!(abstract_model
            .dynamic_cast::<QAbstractTableModel>()
            .is_null());
    }
}
