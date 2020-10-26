use imgui;
use imgui::{im_str, ImString, MenuItem};
use nfd;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::edit::EditDetailsGui;
use crate::gui::zelda2::enemyattr::EnemyGui;
use crate::gui::zelda2::palette::PaletteGui;
use crate::gui::zelda2::start::StartGui;
use crate::gui::zelda2::Gui;
use crate::util::UTime;
use crate::zelda2::project::{EditAction, Project};

pub struct ProjectGui {
    pub visible: bool,
    pub filename: Option<String>,
    pub widgets: Vec<Box<dyn Gui>>,
    pub project: RefCell<Project>,
}

impl ProjectGui {
    pub fn new(p: Project) -> Self {
        ProjectGui {
            visible: true,
            filename: None,
            widgets: Vec::new(),
            project: RefCell::new(p),
        }
    }
    pub fn from_file(filename: &str) -> Result<Self> {
        let mut project = ProjectGui::new(Project::from_file(&Path::new(filename))?);
        project.filename = Some(filename.to_owned());
        Ok(project)
    }

    fn save(&mut self) {
        if let Some(path) = &self.filename {
            match self.project.borrow().to_file(&Path::new(&path)) {
                Err(e) => error!("Could not save project as {:?}: {:?}", path, e),
                _ => {}
            }
        } else {
            self.save_dialog(false);
        }
    }
    fn save_dialog(&mut self, save_as: bool) {
        let result = nfd::open_save_dialog(Some("z2prj"), None).unwrap();
        match result {
            nfd::Response::Okay(path) => match self.project.borrow().to_file(&Path::new(&path)) {
                Err(e) => error!("Could not save project as {:?}: {:?}", path, e),
                Ok(_) => {
                    if !save_as {
                        self.filename = Some(path);
                    }
                }
            },
            _ => {}
        }
    }

    pub fn menu(&mut self, ui: &imgui::Ui) {
        ui.menu_bar(|| {
            ui.menu(im_str!("File"), true, || {
                if MenuItem::new(im_str!("Save")).build(ui) {
                    self.save();
                }
                if MenuItem::new(im_str!("Save As")).build(ui) {
                    self.save_dialog(true);
                }
            });
            ui.menu(im_str!("Edit"), true, || {
                if MenuItem::new(im_str!("Enemy Attributes")).build(ui) {
                    match EnemyGui::new(&self.project.borrow_mut(), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create EnemyGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Palette")).build(ui) {
                    match PaletteGui::new(&self.project.borrow_mut(), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create PaletteGui: {:?}", e),
                    };
                }
                if MenuItem::new(im_str!("Start Values")).build(ui) {
                    match StartGui::new(&self.project.borrow_mut(), -1) {
                        Ok(gui) => self.widgets.push(gui),
                        Err(e) => error!("Could not create StartGui: {:?}", e),
                    };
                }
            });
        });
    }

    fn dispose_widgets(&mut self) {
        let mut i = 0;
        while i != self.widgets.len() {
            if self.widgets[i].wants_dispose() {
                self.widgets.remove(i);
            } else {
                i += 1;
            }
        }
    }
    fn process_editactions(&self) {
        let mut project = self.project.borrow_mut();
        let mut i = 0;
        let mut first_action = 0;
        while i != project.edits.len() {
            let action = project.edits[i].action.replace(EditAction::None);
            match action {
                EditAction::None => {
                    i += 1;
                }
                EditAction::MoveTo(pos) => {
                    let pos = pos as usize;
                    project.edits.swap(i, pos);
                    if first_action == 0 {
                        first_action = if i < pos { i } else { pos };
                    }
                }
                EditAction::Delete => {
                    project.edits.remove(i);
                    if first_action == 0 {
                        first_action = i;
                    }
                }
            }
        }
        if first_action != 0 {
            match project.replay(first_action as isize, -1) {
                Err(e) => error!("EditActions: replay errror {:?}", e),
                _ => {}
            };
        }
    }

    fn draw_edit(&mut self, index: isize, ui: &imgui::Ui) {
        let id = ui.push_id(index as i32);
        let project = self.project.borrow_mut();
        let edit = project.get_commit(index).unwrap();
        let len = project.edits.len() as isize;
        let meta = edit.meta.borrow();
        let hdr = imgui::CollapsingHeader::new(&ImString::new(&meta.label)).build(ui);
        if ui.popup_context_item(im_str!("menu")) {
            if MenuItem::new(im_str!("Edit")).build(ui) {
                match edit.edit.borrow().gui(&project, index) {
                    Ok(gui) => self.widgets.push(gui),
                    Err(e) => error!("Error creating widget: {:?}", e),
                }
            }
            if MenuItem::new(im_str!("Details")).build(ui) {
                let gui = EditDetailsGui::new(Rc::clone(&edit)).unwrap();
                self.widgets.push(gui);
            }
            if MenuItem::new(im_str!("Move Up"))
                .enabled(index > 1)
                .build(ui)
            {
                edit.action.replace(EditAction::MoveTo(index - 1));
            }
            if MenuItem::new(im_str!("Move Down"))
                .enabled(index > 0 && index < len - 1)
                .build(ui)
            {
                edit.action.replace(EditAction::MoveTo(index + 1));
            }
            if MenuItem::new(im_str!("Delete"))
                .enabled(index > 0)
                .build(ui)
            {
                edit.action.replace(EditAction::Delete);
            }
            ui.end_popup();
        }
        if hdr {
            ui.bullet_text(&im_str!(
                "By {} on {}",
                meta.user,
                UTime::datetime(meta.timestamp)
            ));
            if !meta.comment.is_empty() {
                ui.text_wrapped(&ImString::new(&meta.comment));
            }
        }
        id.pop(ui);
    }

    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = self.visible;
        let title = if let Some(f) = &self.filename {
            im_str!("Project: {}", f)
        } else {
            imgui::ImString::new("Project: unnamed")
        };
        imgui::Window::new(&title)
            .opened(&mut visible)
            .menu_bar(true)
            .build(ui, || {
                self.menu(ui);

                let editlist = ui.push_id("editlist");
                let edits = self.project.borrow().edits.len();
                for i in 0..edits {
                    self.draw_edit(i as isize, ui);
                }
                editlist.pop(ui);

                let mut project = self.project.borrow_mut();
                let widgetlist = ui.push_id("widgetlist");
                for widget in self.widgets.iter_mut() {
                    widget.draw(&mut project, ui);
                }
                widgetlist.pop(ui);
            });
        self.visible = visible;
        self.dispose_widgets();
        self.process_editactions();
    }
}