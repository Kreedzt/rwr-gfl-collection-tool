// SPDX-License-Identifier: GPL-3.0-only
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
use rwr_gfl_armor_parser;
use rwr_gfl_weapon_parser;

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
        weapon_group_box.set_title(&qs("武器数据"));
        let weapon_group_layout = QVBoxLayout::new_0a();

        let weapons_label = QLabel::new();
        weapons_label.set_text(&qs("输入weapons文件夹路径"));
        weapons_label.set_text_format(TextFormat::PlainText);
        weapon_group_layout.add_widget(&weapons_label);

        let weapons_line_edit = QLineEdit::new();
        weapon_group_layout.add_widget(&weapons_line_edit);

        let weapons_translation_label = QLabel::new();
        weapons_translation_label.set_text(&qs("输入weapons翻译文件路径"));
        weapons_translation_label.set_text_format(TextFormat::PlainText);
        weapon_group_layout.add_widget(&weapons_translation_label);

        let weapons_translation_line_edit = QLineEdit::new();
        weapon_group_layout.add_widget(&weapons_translation_line_edit);

        let weapon_exec_button = QPushButton::from_q_string(&qs("开始提取武器数据(.csv文件)"));
        weapon_group_layout.add_widget(&weapon_exec_button);

        // weapons 布局添加完毕, 追加到总布局
        weapon_group_box.set_layout(&weapon_group_layout);
        layout.add_widget(&weapon_group_box);

        // armor 分组
        let armor_group_box = QGroupBox::new();
        armor_group_box.set_title(&qs("护甲数据"));

        let armor_group_layout = QVBoxLayout::new_0a();

        let armor_label = QLabel::new();
        armor_label.set_text(&qs("输入items文件夹路径"));
        armor_label.set_text_format(TextFormat::PlainText);
        armor_group_layout.add_widget(&armor_label);

        let armor_line_edit = QLineEdit::new();
        armor_group_layout.add_widget(&armor_line_edit);

        let armor_translation_label = QLabel::new();
        armor_translation_label.set_text(&qs("输入items翻译文件路径"));
        armor_translation_label.set_text_format(TextFormat::PlainText);
        armor_group_layout.add_widget(&armor_translation_label);

        let armor_translation_line_edit = QLineEdit::new();
        armor_group_layout.add_widget(&armor_translation_line_edit);

        let armor_exec_button = QPushButton::from_q_string(&qs("开始提取护甲数据(.csv文件)"));
        armor_group_layout.add_widget(&armor_exec_button);

        // armor 布局添加完毕, 追加到总布局
        armor_group_box.set_layout(&armor_group_layout);
        layout.add_widget(&armor_group_box);

        // 著名信息
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


        // 绑定事件并处理逻辑
        // let widget_ref = Rc::new(widget);

        // 绑定 weapons 执行按钮
        // let widget = widget
        let weapon_exec_button_ref = Rc::new(weapon_exec_button);
        let weapon_exec_button_cloned = weapon_exec_button_ref.clone();

        let widget_ptr = widget.as_ptr();

        let weapon_buton_clicked = SlotNoArgs::new(NullPtr, move || {
            let folder_path = weapons_line_edit.text();
            let translation_path = weapons_translation_line_edit.text();

            let res = rwr_gfl_weapon_parser::export_to_file(&folder_path.to_std_string(), &translation_path.to_std_string());

            weapon_exec_button_cloned.set_text(&qs("正在处理中..."));
            weapon_exec_button_cloned.set_disabled(true);

            match res {
                Ok(path) => {
                    QMessageBox::information_q_widget2_q_string(
                        widget_ptr,
                        &qs("执行成功"),
                        &qs("生成文件名为: \"%1\".")
                            .arg_q_string(&qs(path))
                    );
                }
                Err(err) => {
                    QMessageBox::critical_q_widget2_q_string(
                        widget_ptr,
                        &qs("执行失败"),
                        &qs("\"%1\".")
                            .arg_q_string(&qs(format!("{:?}", err)))
                    );
                }
            }

            weapon_exec_button_cloned.set_text(&qs("开始提取武器数据(.csv文件)"));
            weapon_exec_button_cloned.set_disabled(false);
        });

        let weapon_exec_button_cloned = weapon_exec_button_ref.clone();
        weapon_exec_button_cloned.clicked().connect(&weapon_buton_clicked);

        // 绑定 armor 执行按钮
        let armor_exec_button_ref = Rc::new(armor_exec_button);
        let armor_exec_button_cloned = armor_exec_button_ref.clone();

        let armor_button_clicked = SlotNoArgs::new(NullPtr, move || {
            let folder_path = armor_line_edit.text();
            let translation_path = armor_translation_line_edit.text();

            let res = rwr_gfl_armor_parser::export_to_file(&folder_path.to_std_string(), &translation_path.to_std_string());

            armor_exec_button_cloned.set_text(&qs("正在处理中..."));
            armor_exec_button_cloned.set_disabled(true);

            match res {
                Ok(path) => {
                    QMessageBox::information_q_widget2_q_string(
                        widget_ptr,
                        &qs("执行成功"),
                        &qs("生成文件名为: \"%1\".")
                            .arg_q_string(&qs(path))
                    );
                }
                Err(err) => {
                    QMessageBox::critical_q_widget2_q_string(
                        widget_ptr,
                        &qs("执行失败"),
                        &qs("\"%1\".")
                            .arg_q_string(&qs(format!("{:?}", err)))
                    );
                }
            }

            armor_exec_button_cloned.set_text(&qs("开始提取护甲数据(.csv文件)"));
            armor_exec_button_cloned.set_disabled(false);
        });

        let armor_exec_button_cloned = armor_exec_button_ref.clone();
        armor_exec_button_cloned.clicked().connect(&armor_button_clicked);

        QApplication::exec()
    });
}
