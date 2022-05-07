// 移除控制台
#![windows_subsystem = "windows"]

use qt_widgets::cpp_core::{NullPtr, Ptr, Ref, StaticUpcast};
use qt_widgets::qt_core::{qs, QTime, QTimer, SlotNoArgs, TextFormat};
use qt_widgets::qt_gui::QTextFormat;
use qt_widgets::{
    QApplication, QLabel, QLineEdit, QMessageBox, QPushButton, QToolBar, QVBoxLayout, QWidget,
};

use std::cell::RefCell;
use std::rc::Rc;
mod constant;

use constant::APPLICATION_NAME;

fn main() {
    QApplication::init(|app| unsafe {
        let widget = QWidget::new_0a();
        let layout = QVBoxLayout::new_1a(&widget);
        widget.set_window_title(&qs(APPLICATION_NAME));
        widget.set_minimum_width(400);

        // let toolbar = QToolBar::new();

        let label = QLabel::new();
        label.set_text(&qs("输入Mod文件夹路径(通常以 2513537759 为末尾)"));
        label.set_text_format(TextFormat::PlainText);
        layout.add_widget(&label);

        let line_edit = QLineEdit::new();
        layout.add_widget(&line_edit);

        let button = QPushButton::from_q_string(&qs("开始提取武器数据(.csv文件)"));
        layout.add_widget(&button);

        let button2 = QPushButton::from_q_string(&qs("开始提取护甲数据(.csv文件)"));
        layout.add_widget(&button2);

        let label = QLabel::new();
        label.set_text(&qs(format!(
            "{} v:{}",
            APPLICATION_NAME,
            env!("CARGO_PKG_VERSION")
        )));
        label.set_text_format(TextFormat::PlainText);
        layout.add_widget(&label);

        let label = QLabel::new();
        label.set_text(&qs(format!("作者: {}", env!("CARGO_PKG_AUTHORS"))));
        label.set_text_format(TextFormat::PlainText);
        layout.add_widget(&label);

        widget.show();

        let buton_clicked = SlotNoArgs::new(NullPtr, move || {
            let text = line_edit.text();
            // QMessageBox::information_q_widget2_q_string(
            //     &widget,
            //     &qs("Example"),
            //     &qs("Text: \"%1\".")
            //         .arg_q_string(&text)
            // );
        });
        button.clicked().connect(&buton_clicked);

        let button_ref = Rc::new(button);
        let button_cloned = button_ref.clone();

        let button2_clicked = SlotNoArgs::new(NullPtr, move || {
            button_cloned.set_text(&qs("禁用中!"));
            button_cloned.set_disabled(true);
        });
        button2.clicked().connect(&button2_clicked);

        QApplication::exec()
    });
}
