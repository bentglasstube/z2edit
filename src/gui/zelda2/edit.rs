use imgui;
use imgui::{im_str, ImString};
use std::rc::Rc;

use crate::errors::*;
use crate::gui::fa;
use crate::gui::util::tooltip;
use crate::gui::zelda2::Gui;
use crate::gui::Visibility;
use crate::zelda2::project::{Edit, EditAction, Project};

pub struct EditDetailsGui {
    visible: Visibility,
    changed: bool,
    edit: Rc<Edit>,
    label: ImString,
    user: ImString,
    comment: ImString,
    config: ImString,
    extra: Vec<(ImString, ImString)>,
}

impl EditDetailsGui {
    pub fn new(edit: Rc<Edit>) -> Result<Box<dyn Gui>> {
        let e = Rc::clone(&edit);
        let meta = e.meta.borrow();
        Ok(Box::new(EditDetailsGui {
            visible: Visibility::Visible,
            changed: false,
            edit: edit,
            label: ImString::new(&meta.label),
            user: ImString::new(&meta.user),
            comment: ImString::new(&meta.comment),
            config: ImString::new(&meta.config),
            extra: meta
                .extra
                .iter()
                .map(|(k, v)| (ImString::new(k), ImString::new(v)))
                .collect(),
        }))
    }

    fn draw_extra(&mut self, ui: &imgui::Ui) -> bool {
        let mut action = EditAction::None;
        ui.text("Extra Data:");
        ui.text("Key");
        ui.same_line(216.0);
        ui.text("Value");
        for i in 0..self.extra.len() {
            let id = ui.push_id(i as i32);
            let width = ui.push_item_width(200.0);
            if imgui::InputText::new(ui, im_str!("##key"), &mut self.extra[i].0)
                .resize_buffer(true)
                .build()
            {
                action.set(EditAction::Update);
            }
            width.pop(ui);
            ui.same_line(0.0);
            let width = ui.push_item_width(400.0);
            if imgui::InputText::new(ui, im_str!("##value"), &mut self.extra[i].1)
                .resize_buffer(true)
                .build()
            {
                action.set(EditAction::Update);
            }
            width.pop(ui);
            ui.same_line(0.0);
            if ui.button(&im_str!("{}", fa::ICON_TRASH), [0.0, 0.0]) {
                action.set(EditAction::Delete(i));
            }
            tooltip("Delete", ui);
            id.pop(ui);
        }
        if ui.button(&im_str!("{}", fa::ICON_COPY), [0.0, 0.0]) {
            action.set(EditAction::New);
        }
        tooltip("New Key/Value Pair", ui);

        match action {
            EditAction::New => {
                self.extra.push((ImString::new(""), ImString::new("")));
                true
            }
            EditAction::Delete(index) => {
                self.extra.remove(index);
                true
            }
            EditAction::Update => true,
            _ => false,
        }
    }
}

impl Gui for EditDetailsGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let title = im_str!("{} Details", self.edit.meta.borrow().label);
        imgui::Window::new(&title)
            .unsaved_document(self.changed)
            .build(ui, || {
                self.changed |= imgui::InputText::new(ui, im_str!("Label"), &mut self.label)
                    .resize_buffer(true)
                    .build();
                self.changed |= imgui::InputText::new(ui, im_str!("User"), &mut self.user)
                    .resize_buffer(true)
                    .build();
                self.changed |= imgui::InputText::new(ui, im_str!("Config"), &mut self.config)
                    .resize_buffer(true)
                    .build();
                self.changed |= imgui::InputTextMultiline::new(
                    ui,
                    im_str!("Comment"),
                    &mut self.comment,
                    [0.0, 120.0],
                )
                .resize_buffer(true)
                .build();
                ui.separator();
                self.changed |= self.draw_extra(ui);
                ui.separator();

                if ui.button(im_str!("  Ok  "), [0.0, 0.0]) {
                    let mut meta = self.edit.meta.borrow_mut();
                    meta.label = self.label.to_string();
                    meta.user = self.user.to_string();
                    meta.comment = self.comment.to_string();
                    meta.config = self.config.to_string();
                    meta.extra = self
                        .extra
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    project.changed.set(true);
                    self.visible = Visibility::Dispose;
                    self.changed = false;
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                    self.visible = Visibility::Dispose;
                }
            });
    }
    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
