use qt_web_engine_widgets::QWebEngineView;
use qt_core::{QUrl, QString};
use qt_widgets::QApplication;

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