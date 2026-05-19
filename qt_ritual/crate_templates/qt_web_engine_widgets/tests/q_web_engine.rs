use qt_core::{QString, QUrl};
use qt_gui::SlotOfQUrl;
use qt_widgets::QApplication;
use qt_web_engine_widgets::QWebEngineView;

#[test]
fn web1() {
    QApplication::init(|_| unsafe {
        let web = QWebEngineView::new();
        web.load_q_url(&QUrl::from_user_input_1a(&QString::from_std_str(
            "https://www.rust-lang.org",
        )));
        let slot = SlotOfQUrl::new(&web, move |url| {
            let url = url.url_0a().to_std_string();
            assert_eq!(&url, "https://www.rust-lang.org");
        });
        let c = web.url_changed().connect(&slot);
        assert!(c.is_valid());
        0
    })
}
