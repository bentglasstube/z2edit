use ron;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::convert::{From, TryFrom};
use std::rc::Rc;

use crate::errors::*;
use crate::gui::zelda2::overworld::OverworldGui;
use crate::gui::zelda2::Gui;
use crate::idpath;
use crate::nes::{Address, IdPath, MemoryAccess};
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, Project, RomData};

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct PalaceDetail {
        pub to_stone_table: (Address, usize),
        pub chr_table: Address,
        pub palette_table: Address,
        pub length: usize,
    }
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Overworld {
        pub id: IdPath,
        pub name: String,
        pub overworld: usize,
        pub subworld: usize,
        pub pointer: Address,
        pub connector: Address,
        pub encounter: Address,
        pub objtable: Address,
        pub objtable_len: usize,
        pub tile_palette: Address,
        pub palette: Address,
        pub chr: Address,
        pub width: usize,
        pub height: usize,
        pub raft_connector: usize,
        pub raft_table: Address,
        pub palace: PalaceDetail,
    }

    #[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum HiddenKind {
        None,
        Palace,
        Town,
    }

    impl Default for HiddenKind {
        fn default() -> Self {
            HiddenKind::None
        }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HiddenSpot {
        pub id: IdPath,
        pub kind: HiddenKind,
        pub connector: Address,
        pub overworld: Address,
        pub x: Address,
        pub y: Address,
        pub return_y: Address,
        pub ppu_macro: Address,
        pub discriminator: Address,
    }

    #[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum TileSetKind {
        Vanilla,
        CF207Hackjam2020,
    }

    impl Default for TileSetKind {
        fn default() -> Self {
            TileSetKind::Vanilla
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub mason_dixon_line: Address,
        pub y_offset: i32,
        pub map: Vec<config::Overworld>,
        pub hidden: Vec<config::HiddenSpot>,
        pub terrain_name: Vec<String>,
        pub palace_connectors: Vec<usize>,
        pub town_connectors: Vec<usize>,
        pub overworld_ram: u16,
        pub overworld_len: usize,
        pub tileset: TileSetKind,
    }

    impl Config {
        pub fn to_string(&self) -> String {
            let pretty = ron::ser::PrettyConfig::new();
            ron::ser::to_string_pretty(&self, pretty).unwrap()
        }

        pub fn find(&self, path: &IdPath) -> Result<&config::Overworld> {
            path.check_range("overworld", 1..=3)?;
            for overworld in self.map.iter() {
                if path.prefix(&overworld.id) {
                    return Ok(overworld);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn find_hidden(&self, path: &IdPath) -> Result<&config::HiddenSpot> {
            path.check_len("hidden", 1)?;
            for hidden in self.hidden.iter() {
                if *path == hidden.id {
                    return Ok(hidden);
                }
            }
            Err(ErrorKind::IdPathNotFound(path.into()).into())
        }

        pub fn palace_code(&self, conn: usize) -> Option<usize> {
            if self.palace_connectors.contains(&conn) {
                Some(conn - self.palace_connectors[0])
            } else {
                None
            }
        }

        pub fn town_code(&self, conn: usize) -> Option<usize> {
            if self.town_connectors.contains(&conn) {
                Some((conn - self.town_connectors[0]) / 2)
            } else {
                None
            }
        }

        pub fn is_encounter(&self, edit: &Rc<Edit>, id: &IdPath) -> Result<bool> {
            let ocfg = match self.find(id) {
                Ok(v) => v,
                Err(_) => {
                    return Ok(false);
                }
            };
            let encounters = edit
                .rom
                .borrow()
                .read_bytes(ocfg.encounter, 14)?
                .iter()
                .map(|byte| Encounter::from(*byte).dest_map)
                .collect::<Vec<i32>>();

            let map = id.usize_last()? as i32;
            Ok(encounters.contains(&map))
        }

        pub fn vanilla() -> Self {
            ron::de::from_bytes(include_bytes!("../../config/vanilla/overworld.ron")).unwrap()
        }
    }
}

use config::HiddenKind;

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(try_from = "JsonMap")]
#[serde(into = "JsonMap")]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<u8>>,
}

struct CompressedMap {
    data: Vec<u8>,
    palace_offset: [u16; 4],
    hidden_palace: Option<u8>,
    hidden_town: Option<u8>,
}

impl Default for CompressedMap {
    fn default() -> Self {
        CompressedMap {
            data: Vec::new(),
            palace_offset: [0xffffu16; 4],
            hidden_palace: None,
            hidden_town: None,
        }
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Map {
            width: width,
            height: height,
            ..Default::default()
        }
    }

    fn decompress(&mut self, addr: Address, edit: &Rc<Edit>) -> Result<usize> {
        let config = Config::get(&edit.config())?;
        match config.overworld.tileset {
            config::TileSetKind::Vanilla => self.decompress_standard(addr, edit),
            config::TileSetKind::CF207Hackjam2020 => self.decompress_hackjam2020(addr, edit),
        }
    }

    fn compress(&self, overworld: &Overworld, edit: &Rc<Edit>) -> Result<CompressedMap> {
        let config = Config::get(&edit.config())?;
        match config.overworld.tileset {
            config::TileSetKind::Vanilla => self.compress_standard(overworld, edit),
            config::TileSetKind::CF207Hackjam2020 => self.compress_hackjam2020(overworld, edit),
        }
    }

    fn decompress_standard(&mut self, addr: Address, edit: &Rc<Edit>) -> Result<usize> {
        let rom = edit.rom.borrow();
        self.data = vec![vec![0xf as u8; self.width]; self.height];
        let mut y = 0;
        let mut index = 0;
        while y < self.height {
            let mut x = 0;
            while x < self.width {
                let val = rom.read(addr + index)?;
                let tile = val & 0x0F;
                let count = (val >> 4) as usize + 1;
                for _ in 0..count {
                    if x < self.width {
                        self.data[y][x] = tile;
                    }
                    x += 1;
                }
                index += 1;
            }
            y += 1;
        }
        Ok(index)
    }

    fn decompress_hackjam2020(&mut self, addr: Address, edit: &Rc<Edit>) -> Result<usize> {
        let rom = edit.rom.borrow();
        self.data = vec![vec![0xf as u8; self.width]; self.height];
        let mut y = 0;
        let mut index = 0;
        while y < self.height {
            let mut x = 0;
            while x < self.width {
                let val = rom.read(addr + index)?;
                let mut tile = val & 0x0F;
                let mut count = (val >> 4) as usize;
                if tile == 0x0F && count > 0 {
                    // Any tile F with a count > 0 represents an expansion tile
                    // encoded in the next byte.
                    index += 1;
                    tile = rom.read(addr + index)?;
                } else {
                    // Standard encoding for all tiles 0-E and for a single tile F.
                    count += 1;
                }
                for _ in 0..count {
                    if x < self.width {
                        self.data[y][x] = tile;
                    }
                    x += 1;
                }
                index += 1;
            }
            y += 1;
        }
        Ok(index)
    }

    fn compress_standard(&self, overworld: &Overworld, edit: &Rc<Edit>) -> Result<CompressedMap> {
        let config = Config::get(&edit.config())?;
        let comp_boulder = edit
            .meta
            .borrow()
            .extra
            .get("compress_boulder")
            .map(|v| v.as_str())
            .unwrap_or("true")
            .parse()?;
        let comp_spider = edit
            .meta
            .borrow()
            .extra
            .get("compress_spider")
            .map(|v| v.as_str())
            .unwrap_or("true")
            .parse()?;

        let mut map = CompressedMap::default();
        for (y, row) in self.data.iter().enumerate() {
            let mut x = 0;
            while x < self.width {
                let mut count = 0u8;
                let mut want_compress = true;
                let tile = row[x];
                if let Some(conn) = overworld.connector_at(x, y) {
                    let index = conn.id.usize_last().unwrap();
                    if let Some(palace) = config.overworld.palace_code(index) {
                        want_compress = false;
                        map.palace_offset[palace] = map.data.len() as u16;
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                map.hidden_palace = Some(tile);
                            }
                        }
                    } else {
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                want_compress = false;
                                map.hidden_town = Some(tile);
                            }
                        }
                    }
                }
                if tile == 0x0E {
                    want_compress = comp_boulder;
                }
                if tile == 0x0F {
                    want_compress = comp_spider;
                }

                while want_compress
                    && count < 15
                    && x + 1 < self.width
                    && tile == row[x + 1]
                    && !overworld.skip_compress(x + 1, y)
                {
                    x += 1;
                    count += 1;
                }
                map.data.push(tile | count << 4);
                x += 1;
            }
        }
        Ok(map)
    }

    fn compress_hackjam2020(
        &self,
        overworld: &Overworld,
        edit: &Rc<Edit>,
    ) -> Result<CompressedMap> {
        let config = Config::get(&edit.config())?;
        let mut map = CompressedMap::default();
        for (y, row) in self.data.iter().enumerate() {
            let mut x = 0;
            while x < self.width {
                let mut count = 0u8;
                let mut want_compress = true;
                let tile = row[x];
                if let Some(conn) = overworld.connector_at(x, y) {
                    let index = conn.id.usize_last().unwrap();
                    if let Some(palace) = config.overworld.palace_code(index) {
                        want_compress = false;
                        map.palace_offset[palace] = map.data.len() as u16;
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                map.hidden_palace = Some(tile);
                            }
                        }
                    } else {
                        if let Some(h) = &conn.hidden {
                            if h.hidden {
                                want_compress = false;
                                map.hidden_town = Some(tile);
                            }
                        }
                    }
                }
                if tile == 0x0E || tile == 0x0F {
                    want_compress = false;
                }
                if tile >= 0x10 {
                    // pre-increment the count for expansion tiles.
                    count += 1;
                }

                while want_compress
                    && count < 15
                    && x + 1 < self.width
                    && tile == row[x + 1]
                    && !overworld.skip_compress(x + 1, y)
                {
                    x += 1;
                    count += 1;
                }
                if tile < 0x10 {
                    // Vanilla tiles get written in the usual way.
                    map.data.push(tile | count << 4);
                } else {
                    // Expansion tiles are encoded as 0x_F, where the high
                    // nibble is the number of tiles followed by a byte
                    // with the tile ID.
                    map.data.push(0x0F | count << 4);
                    map.data.push(tile);
                }
                x += 1;
            }
        }
        Ok(map)
    }
}

const JSONMAP_TRANSFORM: &[u8] =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_*".as_bytes();

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<String>,
}

impl From<&Map> for JsonMap {
    fn from(a: &Map) -> Self {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = String::new();
            for &col in row.iter() {
                let len = JSONMAP_TRANSFORM.len();
                let col = col as usize;
                let col = if col < len { col } else { len - 1 };
                s.push(JSONMAP_TRANSFORM[col] as char);
            }
            map.push(s);
        }
        JsonMap {
            width: a.width,
            height: a.height,
            data: map,
        }
    }
}

impl From<Map> for JsonMap {
    fn from(a: Map) -> Self {
        JsonMap::from(&a)
    }
}

impl TryFrom<JsonMap> for Map {
    type Error = Error;
    fn try_from(a: JsonMap) -> Result<Self> {
        let mut map = Vec::new();
        for row in a.data.iter() {
            let mut s = Vec::new();
            for col in row.chars() {
                let v = JSONMAP_TRANSFORM
                    .iter()
                    .position(|&x| col as u8 == x)
                    .ok_or_else(|| -> Error {
                        ErrorKind::TransformationError(format!(
                            "Map transform could not convert '{}'",
                            col
                        ))
                        .into()
                    })?;
                s.push(v as u8);
            }
            map.push(s);
        }
        Ok(Map {
            width: a.width,
            height: a.height,
            data: map,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hidden {
    pub hidden: bool,
    pub id: IdPath,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PalaceDetail {
    pub index: usize,
    pub chr_bank: i32,
    pub palette: i32,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: IdPath,

    pub x: i32,
    pub y: i32,
    pub dest_map: i32,
    pub dest_world: usize,
    pub dest_overworld: usize,

    pub external: bool,
    pub second: bool,
    pub exit_2_lower: bool,
    pub entry: usize,
    pub entry_right: bool,
    pub passthru: bool,
    pub fall: bool,
    pub hidden: Option<Hidden>,
    pub palace: Option<PalaceDetail>,
}

impl Connector {
    pub fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut connector = Connector::default();
        connector.id = id;
        connector.unpack(edit)?;
        Ok(connector)
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let ocfg = config.overworld.find(&self.id)?;
        let index = self.id.usize_last()?;
        let rom = edit.rom.borrow();

        let y = rom.read(ocfg.connector + index + 0x00)?;
        let x = rom.read(ocfg.connector + index + 0x3f)?;
        let z = rom.read(ocfg.connector + index + 0x7e)?;
        let w = rom.read(ocfg.connector + index + 0xbd)?;

        self.set_y(y, &config.overworld);

        self.x = (x & 0x3f) as i32;
        self.second = (x & 0x40) != 0;
        self.exit_2_lower = (x & 0x80) != 0;

        self.dest_map = (z & 0x3f) as i32;
        self.entry = (z >> 6) as usize;

        self.dest_overworld = (w & 0x03) as usize;
        self.dest_world = (w & 0x1c) as usize >> 2;
        self.entry_right = (w & 0x20) != 0;
        self.passthru = (w & 0x40) != 0;
        self.fall = (w & 0x80) != 0;

        self.palace = if let Some(p) = config.overworld.palace_code(index) {
            if p < ocfg.palace.length {
                Some(PalaceDetail {
                    index: p,
                    chr_bank: rom.read(ocfg.palace.chr_table + p)? as i32 * 2,
                    palette: rom.read(ocfg.palace.palette_table + p)? as i32 / 16,
                })
            } else {
                None
            }
        } else {
            None
        };
        self.hidden = if let Some(spot) = self.hidden_index(edit, &config.overworld)? {
            self.set_y(rom.read(spot.connector + 2)?, &config.overworld);
            Some(Hidden {
                hidden: y == 0,
                id: idpath!(spot.id),
            })
        } else {
            None
        };

        Ok(())
    }

    fn hidden_index<'a>(
        &self,
        edit: &Rc<Edit>,
        config: &'a config::Config,
    ) -> Result<Option<&'a config::HiddenSpot>> {
        let ocfg = config.find(&self.id)?;
        let index = self.id.usize_last()?;
        let rom = edit.rom.borrow();
        for h in config.hidden.iter() {
            if rom.read(h.connector)? == index as u8
                && rom.read(h.overworld)? == ocfg.overworld as u8
                && ocfg.subworld == 0
            {
                // For now we assume a hidden spot cannot be on DM or MZ.
                return Ok(Some(h));
            }
        }
        Ok(None)
    }

    fn set_y(&mut self, y: u8, config: &config::Config) {
        self.y = (y & 0x7f) as i32 - config.y_offset;
        self.external = (y & 0x80) != 0;
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let ocfg = config.overworld.find(&self.id)?;
        let index = self.id.usize_last()?;
        let mut rom = edit.rom.borrow_mut();

        let y =
            if self.external { 0x80 } else { 0x00 } | (self.y + config.overworld.y_offset) as u8;
        let x = if self.second { 0x40 } else { 0x00 }
            | if self.exit_2_lower { 0x80 } else { 0x00 }
            | self.x as u8;
        let z = (self.entry << 6) as u8 | self.dest_map as u8;

        let w = if self.entry_right { 0x20 } else { 0x00 }
            | if self.passthru { 0x40 } else { 0x00 }
            | if self.fall { 0x80 } else { 0x00 }
            | self.dest_overworld as u8
            | (self.dest_world << 2) as u8;

        rom.write(ocfg.connector + index + 0x00, y)?;
        rom.write(ocfg.connector + index + 0x3f, x)?;
        rom.write(ocfg.connector + index + 0x7e, z)?;
        rom.write(ocfg.connector + index + 0xbd, w)?;

        if let Some(palace) = &self.palace {
            let p = palace.index;
            rom.write(ocfg.palace.chr_table + p, palace.chr_bank as u8 / 2)?;
            rom.write(ocfg.palace.palette_table + p, palace.palette as u8 * 16)?;
        }
        if let Some(hidden) = &self.hidden {
            let spot = config.overworld.find_hidden(&hidden.id)?;
            if spot.kind == HiddenKind::Palace {
                // Hidden palace Y coordinate includes the overworld_y_offset.
                // Furthermore, the "call" spot is 2 tiles north of the target
                // destination.
                if spot.ppu_macro.raw() == 0 {
                    return Err(ErrorKind::ConfigError(format!(
                        "No ppu_macro address defined for {}",
                        spot.id.to_string()
                    ))
                    .into());
                }
                rom.write(spot.connector + 2, y)?;
                rom.write(spot.y, (y & 0x7f) - 2)?;
                rom.write(spot.x, self.x as u8)?;
                rom.write(spot.return_y, y & 0x7f)?;

                // Compute the destination address in VRAM.
                let xx = self.x as u16;
                let yy = self.y as u16;
                let ppu_addr = 0x2000 + 2 * (32 * (yy % 15) + (xx % 16)) + 0x800 * (yy % 30 / 15);

                rom.write(spot.ppu_macro + 0, (ppu_addr >> 8) as u8)?;
                rom.write(spot.ppu_macro + 1, ppu_addr as u8)?;
                rom.write(spot.ppu_macro + 5, ((ppu_addr + 32) >> 8) as u8)?;
                rom.write(spot.ppu_macro + 6, (ppu_addr + 32) as u8)?;
                // Being lazy and not dealing with the color bits.
                rom.write(spot.ppu_macro + 10, 0xff)?;

                if hidden.hidden {
                    rom.write(ocfg.connector + index + 0x00, 0)?;
                }
            } else if spot.kind == HiddenKind::Town {
                // Hidden Town Y coordinate does not include the overworld_y_offset.
                // Furthermore, the x location seems to be 1 more than the actual
                // coordinate.
                if spot.discriminator.raw() == 0 {
                    return Err(ErrorKind::ConfigError(format!(
                        "No discriminator address defined for {}",
                        spot.id.to_string()
                    ))
                    .into());
                }
                rom.write(spot.connector + 2, y)?;
                rom.write(spot.y, self.y as u8)?;
                rom.write(spot.x, self.x as u8 + 1)?;
                rom.write(spot.return_y, y & 0x7f)?;
                rom.write(spot.discriminator, self.x as u8)?;
                if hidden.hidden {
                    rom.write(ocfg.connector + index + 0x00, 0)?;
                }
            } else {
                return Err(ErrorKind::ConfigError(format!(
                    "Don't know how to pack hidden spot {}",
                    spot.id.to_string()
                ))
                .into());
            }
        }
        if index == ocfg.raft_connector {
            // The raft table is stored as xpos, xpos, ypos, ypos.
            // The config stores a start location per overworld.
            rom.write(ocfg.raft_table + 0, x & 0x3f)?;
            rom.write(ocfg.raft_table + 2, y & 0x7f)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub dest_map: i32,
    pub entry: usize,
}

impl From<u8> for Encounter {
    fn from(a: u8) -> Encounter {
        Encounter {
            dest_map: (a & 0x3f) as i32,
            entry: (a >> 6) as usize,
        }
    }
}

impl From<&Encounter> for u8 {
    fn from(a: &Encounter) -> u8 {
        (a.dest_map & 0x3f) as u8 | (a.entry << 6) as u8
    }
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Overworld {
    pub id: IdPath,
    pub mason_dixon_line: i32,
    pub map: Map,
    pub encounter: Vec<Encounter>,
    pub connector: Vec<Connector>,
}

impl Overworld {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if let Some(id) = id {
            Ok(Box::new(Self {
                id: IdPath::from(id),
                ..Default::default()
            }))
        } else {
            Err(ErrorKind::IdPathError("id required".to_string()).into())
        }
    }

    pub fn from_rom(edit: &Rc<Edit>, id: IdPath) -> Result<Self> {
        let mut overworld = Overworld::default();
        overworld.id = id;
        overworld.unpack(edit)?;
        Ok(overworld)
    }

    fn skip_compress(&self, x: usize, y: usize) -> bool {
        let x = x as i32;
        let y = y as i32;
        for c in self.connector.iter() {
            if let Some(h) = &c.hidden {
                if h.hidden && c.x == x && c.y == y {
                    return true;
                }
            }
            if c.palace.is_some() && c.x == x && c.y == y {
                return true;
            }
        }
        false
    }

    fn connector_at(&self, x: usize, y: usize) -> Option<&Connector> {
        let x = x as i32;
        let y = y as i32;
        for c in self.connector.iter() {
            if c.x == x && c.y == y {
                return Some(c);
            }
        }
        None
    }

    fn compress(&self, edit: &Rc<Edit>) -> Result<CompressedMap> {
        self.map.compress(self, edit)
    }

    pub fn compressed_size(&self, edit: &Rc<Edit>) -> Result<usize> {
        let compressed = self.compress(edit)?;
        Ok(compressed.data.len())
    }
}

#[typetag::serde]
impl RomData for Overworld {
    fn name(&self) -> String {
        "Overworld".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let ocfg = config.overworld.find(&self.id)?;
        let addr = edit.rom.borrow().read_pointer(ocfg.pointer)?;

        self.mason_dixon_line = edit.rom.borrow().read(config.overworld.mason_dixon_line)? as i32
            - config.overworld.y_offset;
        self.map.width = ocfg.width;
        self.map.height = ocfg.height;
        self.map.decompress(addr, &edit)?;
        self.encounter = edit
            .rom
            .borrow()
            .read_bytes(ocfg.encounter, 14)?
            .iter()
            .map(|byte| Encounter::from(*byte))
            .collect();

        self.connector.clear();
        for index in 0..63 {
            let conn_id = ocfg.id.extend("connector").extend(index);
            self.connector.push(Connector::from_rom(edit, conn_id)?);
        }

        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let map = self.compress(&edit)?;
        let config = Config::get(&edit.config())?;
        let ocfg = config.overworld.find(&self.id)?;
        if map.data.len() >= config.overworld.overworld_len {
            return Err(ErrorKind::LengthError(format!(
                "Overworld too big: compressed size of {} bytes is larger than {} bytes",
                map.data.len(),
                config.overworld.overworld_len
            ))
            .into());
        }
        {
            let mut memory = edit.memory.borrow_mut();
            let addr = edit.rom.borrow().read_pointer(ocfg.pointer)?;
            let length = {
                let mut orig = Map::new(ocfg.width, ocfg.height);
                orig.decompress(addr, &edit)? as u16
            };

            memory.free(addr, length);
            let addr = memory.alloc_near(addr, map.data.len() as u16)?;
            info!(
                "Overworld::pack: storing {} at {:x?} using {} bytes",
                self.id,
                addr,
                map.data.len()
            );
            let mut rom = edit.rom.borrow_mut();
            rom.write_bytes(addr, &map.data)?;
            rom.write_pointer(ocfg.pointer, addr)?;

            let (addr, length) = ocfg.palace.to_stone_table;
            for i in 0..length {
                if map.palace_offset[i] != 0xFFFF {
                    rom.write_word(
                        addr + i * 2,
                        config.overworld.overworld_ram + map.palace_offset[i],
                    )?
                }
            }

            for (i, encounter) in self.encounter.iter().enumerate() {
                rom.write(ocfg.encounter + i, u8::from(encounter))?;
            }
        }
        for connector in self.connector.iter() {
            connector.pack(edit)?;
        }
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        OverworldGui::new(project, commit_index)
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
