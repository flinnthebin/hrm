#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
  slint::android::init(app).unwrap();

  slint::slint! {
    export component MainWindow inherits Window {
      Text {
        text: "Hello World";
        font-size: 24px;
      }
    }
  }
  MainWindow::new().unwrap().run().unwrap();
}
