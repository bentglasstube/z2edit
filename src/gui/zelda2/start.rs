use std::rc::Rc;

use imgui;
use imgui::im_str;

use crate::errors::*;
use crate::gui::zelda2::Gui;
use crate::gui::Visibility;
use crate::zelda2::project::{Edit, Project, RomData};
use crate::zelda2::start::Start;

pub struct StartGui {
    visible: Visibility,
    changed: bool,
    commit_index: isize,
    edit: Rc<Edit>,
    start: Start,
}

impl StartGui {
    pub fn new(project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        let edit = project.get_commit(commit_index)?;

        let mut start = Start::default();
        start.unpack(&edit)?;

        Ok(Box::new(StartGui {
            visible: Visibility::Visible,
            changed: false,
            commit_index: commit_index,
            edit: edit,
            start: start,
        }))
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        let edit = Box::new(self.start.clone());
        let i = project.commit(self.commit_index, edit)?;
        self.edit = project.get_commit(i)?;
        self.commit_index = i;
        Ok(())
    }
}

impl Gui for StartGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        imgui::Window::new(im_str!("Start Values"))
            .opened(&mut visible)
            .build(ui, || {
                if ui.button(im_str!("Commit"), [0.0, 0.0]) {
                    match self.commit(project) {
                        Err(e) => error!("EnemyGui: commit error {}", e),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                let mut changed = self.changed;
                let width = ui.push_item_width(100.0);
                changed |=
                    imgui::InputInt::new(ui, im_str!("Attack"), &mut self.start.level.attack)
                        .build();
                ui.same_line(0.0);
                changed |=
                    imgui::InputInt::new(ui, im_str!("Magic"), &mut self.start.level.magic).build();
                ui.same_line(0.0);
                changed |=
                    imgui::InputInt::new(ui, im_str!("Life"), &mut self.start.level.life).build();

                changed |= imgui::InputInt::new(
                    ui,
                    im_str!("Heart Containers"),
                    &mut self.start.inventory.heart,
                )
                .build();
                changed |= imgui::InputInt::new(
                    ui,
                    im_str!("Magic Containers"),
                    &mut self.start.inventory.magic,
                )
                .build();
                changed |= imgui::InputInt::new(
                    ui,
                    im_str!("Crystals"),
                    &mut self.start.inventory.crystals,
                )
                .build();
                changed |=
                    imgui::InputInt::new(ui, im_str!("Lives"), &mut self.start.inventory.lives)
                        .build();
                width.pop(ui);

                ui.separator();
                changed |= ui.checkbox(im_str!("Downstab"), &mut self.start.inventory.downstab);
                ui.same_line(0.0);
                changed |= ui.checkbox(im_str!("Upstab"), &mut self.start.inventory.upstab);

                ui.separator();
                ui.columns(2, im_str!("columns"), true);
                changed |= ui.checkbox(im_str!("Shield Spell"), &mut self.start.spell.shield);
                changed |= ui.checkbox(im_str!("Jump Spell"), &mut self.start.spell.jump);
                changed |= ui.checkbox(im_str!("Life Spell"), &mut self.start.spell.life);
                changed |= ui.checkbox(im_str!("Fairy Spell"), &mut self.start.spell.fairy);
                changed |= ui.checkbox(im_str!("Fire Spell"), &mut self.start.spell.fire);
                changed |= ui.checkbox(im_str!("Reflect Spell"), &mut self.start.spell.reflex);
                changed |= ui.checkbox(im_str!("Spell Spell"), &mut self.start.spell.spell);
                changed |= ui.checkbox(im_str!("Thunder Spell"), &mut self.start.spell.thunder);

                ui.next_column();

                changed |= ui.checkbox(im_str!("Candle"), &mut self.start.inventory.candle);
                changed |= ui.checkbox(im_str!("Glove"), &mut self.start.inventory.glove);
                changed |= ui.checkbox(im_str!("Raft"), &mut self.start.inventory.raft);
                changed |= ui.checkbox(im_str!("Boots"), &mut self.start.inventory.boots);
                changed |= ui.checkbox(im_str!("Flute"), &mut self.start.inventory.flute);
                changed |= ui.checkbox(im_str!("Cross"), &mut self.start.inventory.cross);
                changed |= ui.checkbox(im_str!("Hammer"), &mut self.start.inventory.hammer);
                changed |= ui.checkbox(im_str!("Magic Key"), &mut self.start.inventory.magickey);
                self.changed = changed;
            });
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("Start Values Changed"),
            "There are unsaved changes in the Start Values Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}