use qt_web_engine_widgets::{qt_core::{QUrl, QString}, qt_widgets::QApplication, QWebEngineView};

#[test]
fn web1() {
    QApplication::init(|_| unsafe {
        let web = QWebEngineView::new_0a();
        web.load(&QUrl::from_user_input_1a(&QString::from_std_str("https://www.rust-lang.org")));
        let url = web.url().url_0a().to_std_string();
        assert_eq!(&url, "https://www.rust-lang.org");
        0
    })
}