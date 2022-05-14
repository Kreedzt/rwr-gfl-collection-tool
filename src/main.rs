// 移除控制台
#![windows_subsystem = "windows"]

use qt_widgets::cpp_core::{NullPtr, Ptr, Ref, StaticUpcast};
use qt_widgets::qt_core::{qs, QTime, QTimer, SlotNoArgs, TextFormat};
use qt_widgets::qt_gui::{QTextFormat, QGradient};
use qt_widgets::{
    QApplication, QLabel, QLineEdit, QMessageBox, QPushButton, QToolBar, QVBoxLayout, QWidget, QGroupBox,
};

use std::cell::RefCell;
use std::rc::Rc;
mod constant;
use rwr_gfl_armor_parser::export_to_file;

use constant::APPLICATION_NAME;

fn main() {
    QApplication::init(|app| unsafe {
        // 全局
        let widget = QWidget::new_0a();
        let layout = QVBoxLayout::new_1a(&widget);
        widget.set_window_title(&qs(APPLICATION_NAME));
        widget.set_minimum_width(400);

        // weapons 分组
        let weapon_group_box = QGroupBox::new();
        weapon_group_box.set_title(&qs("weapons"));
        let weapon_group_layout = QVBoxLayout::new_0a();

        let weapons_label = QLabel::new();
        weapons_label.set_text(&qs("输入weapons文件夹路径(提取武器文件需要)"));
        weapons_label.set_text_format(TextFormat::PlainText);
        weapon_group_layout.add_widget(&weapons_label);

        let weapons_line_edit = QLineEdit::new();
        weapon_group_layout.add_widget(&weapons_line_edit);

        let weapons_translation_label = QLabel::new();
        weapons_translation_label.set_text(&qs("输入weapons翻译文件路径(提取武器文件需要, 非必须)"));
        weapons_translation_label.set_text_format(TextFormat::PlainText);
        weapon_group_layout.add_widget(&weapons_translation_label);

        let weapons_translation_line_edit = QLineEdit::new();
        weapon_group_layout.add_widget(&weapons_translation_line_edit);

        let button = QPushButton::from_q_string(&qs("开始提取武器数据(.csv文件)"));
        weapon_group_layout.add_widget(&button);

        // weapons 布局添加完毕, 追加到总布局
        weapon_group_box.set_layout(&weapon_group_layout);
        layout.add_widget(&weapon_group_box);

        // armor 分组
        let armor_group_box = QGroupBox::new();
        armor_group_box.set_title(&qs("armor"));

        let armor_group_layout = QVBoxLayout::new_0a();

        let armor_label = QLabel::new();
        armor_label.set_text(&qs("输入items文件夹路径(提取护甲数据需要)"));
        armor_label.set_text_format(TextFormat::PlainText);
        armor_group_layout.add_widget(&armor_label);

        let armor_line_edit = QLineEdit::new();
        armor_group_layout.add_widget(&armor_line_edit);

        let button2 = QPushButton::from_q_string(&qs("开始提取护甲数据(.csv文件)"));
        armor_group_layout.add_widget(&button2);

        // armor 布局添加完毕, 追加到总布局
        armor_group_box.set_layout(&armor_group_layout);
        layout.add_widget(&armor_group_box);

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
            let text = armor_line_edit.text();
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
