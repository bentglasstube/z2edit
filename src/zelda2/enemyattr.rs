use ron;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::zelda2::palette::PaletteGui;
use crate::gui::zelda2::Gui;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Enemy {
        pub id: String,
        pub offset: u8,
        pub name: String,
        pub chr: Address,
        pub palette: u8,
        pub size: (i32, i32),
        pub tiles: Vec<i32>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EnemyGroup {
        pub id: String,
        pub name: String,
        pub world: u8,
        pub overworld: u8,
        pub hp: Address,
        pub xp: Address,
        pub table_len: u8,
        pub enemy: Vec<Enemy>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config(pub Vec<EnemyGroup>);

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<(&EnemyGroup, &Enemy)> {
            path.check_len("enemy", 2)?;
            for group in self.0.iter() {
                if path.at(0) == group.id {
                    for enemy in group.enemy.iter() {
                        if path.at(1) == enemy.id {
                            return Ok((group, enemy));
                        }
                    }
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }
    }

    impl Default for Config {
        fn default() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/enemies.ron")).unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub id: IdPath,
    pub hp: i32,
    pub palette: usize,
    pub steal_xp: bool,
    pub need_fire: bool,
    pub xp: usize,
    pub drop_group: usize,
    pub no_beam: bool,
    pub unknown1: bool,
    pub damage: usize,
    pub no_thunder: bool,
    pub regenerate: bool,
    pub unknown2: bool,
    pub no_sword: bool,
    pub unknown3: usize,
}

#[typetag::serde]
impl RomData for Enemy {
    fn name(&self) -> String {
        "Enemy".to_owned()
    }

    fn unpack(&mut self, edit: &Edit) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let (egrp, ecfg) = config.enemy.find(&self.id)?;
        let rom = edit.rom.borrow();
        let hp = rom.read(egrp.hp + ecfg.offset)?;
        let xp0 = rom.read(egrp.xp + ecfg.offset)?;
        let xp1 = rom.read(egrp.xp + ecfg.offset + egrp.table_len * 1)?;
        let xp3 = rom.read(egrp.xp + ecfg.offset + egrp.table_len * 3)?;

        self.hp = hp as i32;
        self.palette = (xp0 >> 6) as usize;
        self.need_fire = (xp0 & 0x20) != 0;
        self.steal_xp = (xp0 & 0x10) != 0;
        self.xp = (xp0 & 0x0F) as usize;

        self.drop_group = (xp1 >> 6) as usize;
        self.no_beam = (xp1 & 0x20) != 0;
        self.unknown1 = (xp1 & 0x10) != 0;
        self.damage = (xp1 & 0x0F) as usize;

        self.no_thunder = (xp3 & 0x80) != 0;
        self.regenerate = (xp3 & 0x40) != 0;
        self.no_sword = (xp3 & 0x20) != 0;
        self.unknown2 = (xp3 & 0x10) != 0;
        self.unknown3 = (xp3 & 0x0F) as usize;
        Ok(())
    }

    fn pack(&self, edit: &Edit) -> Result<()> {
        let config = Config::get(&edit.meta.borrow().config)?;
        let (egrp, ecfg) = config.enemy.find(&self.id)?;

        let xp0 = (self.palette << 6) as u8
            | if self.need_fire { 0x20 } else { 0x00 }
            | if self.steal_xp { 0x10 } else { 0x00 }
            | self.xp as u8;

        let xp1 = (self.drop_group << 6) as u8
            | if self.no_beam { 0x20 } else { 0x00 }
            | if self.unknown1 { 0x10 } else { 0x00 }
            | self.damage as u8;

        let xp3 = 0 as u8
            | if self.no_thunder { 0x80 } else { 0x00 }
            | if self.regenerate { 0x40 } else { 0x00 }
            | if self.no_sword { 0x20 } else { 0x00 }
            | if self.unknown2 { 0x10 } else { 0x00 }
            | self.unknown3 as u8;

        let mut rom = edit.rom.borrow_mut();
        rom.write(egrp.hp + ecfg.offset, self.hp as u8)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 0, xp0)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 1, xp1)?;
        rom.write(egrp.xp + ecfg.offset + egrp.table_len * 3, xp3)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyGroup {
    pub data: Vec<Enemy>,
}

#[typetag::serde]
impl RomData for EnemyGroup {
    fn name(&self) -> String {
        "EnemyGroup".to_owned()
    }

    fn unpack(&mut self, edit: &Edit) -> Result<()> {
        for d in self.data.iter_mut() {
            d.unpack(edit)?;
        }
        Ok(())
    }

    fn pack(&self, edit: &Edit) -> Result<()> {
        for d in self.data.iter() {
            d.pack(edit)?;
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        PaletteGui::new(project, commit_index)
    }
}
