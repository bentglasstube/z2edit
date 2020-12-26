use std::any::Any;
use std::cell::{Cell, RefCell};
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::vec::Vec;

use whoami;

use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::zelda2::Gui;
use crate::nes::freespace::FreeSpace;
use crate::nes::IdPath;
use crate::nes::{Address, Buffer, MemoryAccess};
use crate::util::pyaddress::PyAddress;
use crate::util::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::connectivity::Connectivity;
use crate::zelda2::edit_factory;
use crate::zelda2::import::{FileResource, ImportRom};

#[pyclass(unsendable)]
#[derive(Default, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub edits: Vec<Rc<Edit>>,
    #[serde(skip)]
    pub changed: Cell<bool>,
}

impl Project {
    pub fn new(name: &str, file: FileResource, config: &str, fix: bool) -> Result<Self> {
        let mut extra = HashMap::new();
        extra.insert("project".to_owned(), name.to_owned());
        extra.insert("fix".to_owned(), fix.to_string());
        let meta = Metadata {
            label: "ImportRom".to_owned(),
            user: whoami::username(),
            timestamp: UTime::now(),
            comment: String::default(),
            config: config.to_owned(),
            extra: extra,
        };
        let config = Config::get(&meta.config)?;
        let commit = Rc::new(Edit {
            meta: RefCell::new(meta),
            edit: RefCell::new(ImportRom::new(file)?),
            rom: RefCell::default(),
            memory: RefCell::new(FreeSpace::new(&config.misc.freespace)?),
            connectivity: RefCell::default(),
            action: RefCell::default(),
            error: RefCell::default(),
        });
        let project = Project {
            name: name.to_owned(),
            edits: vec![commit],
            ..Default::default()
        };
        project.replay(0, -1)?;
        Ok(project)
    }

    pub fn from_rom(filename: &str) -> Result<Self> {
        let now = UTime::now();
        Project::new(
            &format!("Project-{}", UTime::format(now, "%Y%m%d-%H%M")),
            FileResource::Name(filename.to_owned()),
            "vanilla",
            false,
        )
    }

    pub fn from_reader(r: impl Read) -> Result<Self> {
        let project: Project = serde_json::from_reader(r)?;
        match project.replay(0, -1) {
            Ok(_) => {}
            Err(e) => error!("Replay error during loading: {:?}", e),
        }
        Ok(project)
    }

    pub fn from_file(filepath: &Path) -> Result<Self> {
        let file = File::open(filepath)?;
        Project::from_reader(file)
    }

    pub fn to_writer(&self, w: &mut impl Write) -> Result<()> {
        serde_json::to_writer_pretty(w, self)?;
        self.changed.set(false);
        Ok(())
    }

    pub fn to_file(&self, filepath: &Path) -> Result<()> {
        let mut file = File::create(filepath)?;
        self.to_writer(&mut file)
    }

    fn normalized_index(&self, index: isize) -> Result<usize> {
        let len = self.edits.len() as isize;
        if index == -1 {
            Ok((len - 1) as usize)
        } else if index < len {
            Ok(index as usize)
        } else {
            Err(ErrorKind::CommitIndexError(index).into())
        }
    }

    pub fn replay(&self, start: isize, end: isize) -> Result<()> {
        let start = self.normalized_index(start)?;
        let end = self.normalized_index(end)? + 1;
        for i in start..end {
            let result = self.replay_one_commit(i);
            match result {
                Ok(_) => self.edits[i].error.borrow_mut().clear(),
                Err(e) => {
                    self.edits[i].error.replace(e.to_string());
                    return Err(e);
                }
            };
        }
        Ok(())
    }

    fn replay_one_commit(&self, index: usize) -> Result<()> {
        let commit = &self.edits[index];
        let config = Config::get(&commit.meta.borrow().config)?;
        if index == 0 {
            commit
                .memory
                .replace(FreeSpace::new(&config.misc.freespace)?);
        } else {
            let last = &self.edits[index - 1];
            //info!("Project::replay: {}.unpack", commit.edit.borrow().name());
            //commit.edit.borrow_mut().unpack(last)?;
            commit.rom.replace(last.rom.borrow().clone());
            commit.memory.replace(last.memory.borrow().clone());
        }
        info!("Project::replay: {}.pack", commit.edit.borrow().name());
        commit.edit.borrow().pack(&commit)?;
        commit.connectivity.borrow_mut().scan(&commit)?;
        Ok(())
    }

    pub fn last_commit(&self) -> Rc<Edit> {
        self.get_commit(-1).unwrap()
    }

    pub fn get_commit(&self, index: isize) -> Result<Rc<Edit>> {
        let index = self.normalized_index(index)?;
        Ok(Rc::clone(&self.edits[index]))
    }

    pub fn commit(
        &mut self,
        index: isize,
        edit: Box<dyn RomData>,
        suffix: Option<&str>,
    ) -> Result<isize> {
        let len = self.edits.len() as isize;
        if index == -1 {
            let last = self.get_commit(index)?;
            let label = if let Some(suffix) = suffix {
                format!("{}: {}", edit.name(), suffix)
            } else {
                edit.name()
            };
            let meta = Metadata {
                label: label,
                user: whoami::username(),
                timestamp: UTime::now(),
                comment: String::default(),
                config: last.next_config(),
                extra: HashMap::new(),
            };

            let commit = Rc::new(Edit {
                meta: RefCell::new(meta),
                edit: RefCell::new(edit),
                rom: last.rom.clone(),
                memory: last.memory.clone(),
                connectivity: RefCell::default(),
                action: RefCell::default(),
                error: RefCell::default(),
            });
            commit.edit.borrow().pack(&commit)?;
            commit.connectivity.borrow_mut().scan(&commit)?;
            self.edits.push(commit);
            self.changed.set(true);
            Ok(len)
        } else if index < len {
            let last = self.get_commit(index - 1)?;
            let commit = self.get_commit(index)?;
            {
                let mut meta = commit.meta.borrow_mut();
                meta.user = whoami::username();
                meta.timestamp = UTime::now();
            }
            commit.edit.replace(edit);
            commit.rom.replace(last.rom.borrow().clone());
            self.replay(index, -1)?;
            self.changed.set(true);
            Ok(index)
        } else {
            Err(ErrorKind::CommitIndexError(index).into())
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait RomData
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> String;
    fn unpack(&mut self, edit: &Rc<Edit>) -> Result<()>;
    fn pack(&self, edit: &Rc<Edit>) -> Result<()>;
    fn gui(&self, _project: &Project, _commit_index: isize) -> Result<Box<dyn Gui>> {
        Err(ErrorKind::NotImplemented(self.name()).into())
    }
    fn to_text(&self) -> Result<String> {
        Err(ErrorKind::NotImplemented(format!("{}::to_text", self.name())).into())
    }
    fn from_text(&mut self, _text: &str) -> Result<()> {
        Err(ErrorKind::NotImplemented(format!("{}::from_text", self.name())).into())
    }
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Serialize, Deserialize, FromPyObject, IntoPyObject)]
pub struct Metadata {
    pub label: String,
    pub user: String,
    pub timestamp: u64,
    pub comment: String,
    pub config: String,
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

// TODO(cfrantz): should probably move this to gui
#[derive(Debug, PartialEq)]
pub enum EditAction {
    None,
    New,
    NewAt(usize),
    MoveTo(usize),
    CopyAt(usize),
    Delete(usize),
    Swap(usize, usize),
    Drag,
    Update,
    PaletteChanged,
    CacheInvalidate,
}

impl EditAction {
    pub fn set(&mut self, action: EditAction) {
        if action != EditAction::None {
            if *self != EditAction::None {
                error!("EditAction {:?} replaced by {:?}", self, action);
            }
            *self = action;
        }
    }
}

impl Default for EditAction {
    fn default() -> Self {
        EditAction::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edit {
    pub meta: RefCell<Metadata>,
    pub edit: RefCell<Box<dyn RomData>>,
    #[serde(skip)]
    pub rom: RefCell<Buffer>,
    #[serde(skip)]
    pub memory: RefCell<FreeSpace>,
    #[serde(skip)]
    pub connectivity: RefCell<Connectivity>,
    #[serde(skip)]
    pub action: RefCell<EditAction>,
    #[serde(skip)]
    pub error: RefCell<String>,
}

#[derive(Debug, Default)]
pub struct EmulateAt {
    pub bank: u8,
    pub region: u8,
    pub world: u8,
    pub town_code: u8,
    pub palace_code: u8,
    pub connector: u8,
    pub room: u8,
    pub page: u8,
    pub prev_region: u8,
}

impl EmulateAt {
    fn patch(&self, buffer: &Buffer) -> Result<Buffer> {
        let mut buffer = buffer.clone();
        let facing = if self.page < 3 { 0 } else { 1 };
        #[rustfmt::skip]
        let code = [
            0xa9, self.bank,        // LDA #bank
            0x8d, 0x69, 0x07,       // STA $0769
            0xa9, self.region,      // LDA #region
            0x8d, 0x06, 0x07,       // STA $0706
            0xa9, self.world,       // LDA #world
            0x8d, 0x07, 0x07,       // STA $0707
            0xa9, self.town_code,   // LDA #town_code
            0x8d, 0x6b, 0x05,       // STA $056b
            0xa9, self.palace_code, // LDA #palace_code
            0x8d, 0x6c, 0x05,       // STA $056c
            0xa9, self.connector,   // LDA #connector
            0x8d, 0x48, 0x07,       // STA $0748
            0xa9, self.room,        // LDA #room
            0x8d, 0x61, 0x05,       // STA $0561
            0xa9, self.page,        // LDA #page
            0x8d, 0x5c, 0x07,       // STA $075c
            0xa9, facing,           // LDA #facing
            0x8d, 0x01, 0x07,       // STA $0701
            0xa9, self.prev_region, // LDA #prev_region
            0x8d, 0x0a, 0x07,       // STA $070a
            0x60,                   // RTS
        ];
        let addr = Address::Prg(0, 0xAA3F);
        buffer.write_bytes(addr, &code)?;
        Ok(buffer)
    }
}

impl Edit {
    pub fn next_config(&self) -> String {
        let meta = self.meta.borrow();
        if let Some(config) = meta.extra.get("next_config") {
            config.to_owned()
        } else {
            meta.config.to_owned()
        }
    }

    pub fn export(&self, filename: &Path) -> Result<String> {
        let rom = self.rom.borrow();
        rom.save(filename)?;
        Ok(rom.sha256())
    }

    pub fn emulate(&self, at: Option<EmulateAt>) -> Result<()> {
        let mut tmp = std::env::temp_dir();
        tmp.push("zelda2-test.nes");

        if let Some(at) = at {
            info!("{:?}", at);
            let rom = self.rom.borrow();
            let rom = at.patch(&rom)?;
            rom.save(&tmp)?;
        } else {
            self.export(&tmp)?;
        }

        let mut emulator = shellwords::split(&AppContext::emulator())?;
        emulator.push(tmp.to_str().unwrap().to_owned());

        Command::new(&emulator[0]).args(&emulator[1..]).spawn()?;
        Ok(())
    }

    pub fn win_id(&self, index: isize) -> u64 {
        if index == -1 {
            UTime::now()
        } else {
            self.meta.borrow().timestamp
        }
    }

    pub fn overworld_connector(&self, id: &IdPath) -> Option<IdPath> {
        let conn = self.connectivity.borrow();
        conn.overworld_connector(id).map(|x| x.clone())
    }
}

#[pyclass(unsendable)]
pub struct EditProxy {
    edit: Rc<Edit>,
}

impl EditProxy {
    pub fn new(edit: Rc<Edit>) -> EditProxy {
        EditProxy { edit: edit }
    }
}

#[pymethods]
impl EditProxy {
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.edit.edit.borrow().name())
    }

    #[getter]
    fn get_meta(&self) -> PyResult<Metadata> {
        Ok(self.edit.meta.borrow().clone())
    }

    #[setter]
    fn set_meta(&self, meta: Metadata) -> PyResult<()> {
        self.edit.meta.replace(meta);
        Ok(())
    }

    #[getter]
    fn get_text(&self) -> PyResult<String> {
        self.edit.edit.borrow().to_text().map_err(|e| e.into())
    }

    #[setter]
    fn set_text(&self, text: &str) -> PyResult<()> {
        match self.edit.edit.borrow_mut().from_text(text) {
            Ok(()) => {
                self.edit.action.replace(EditAction::Update);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    fn create(&self, kind: &str, id: Option<&str>) -> PyResult<EditProxy> {
        let meta = Metadata {
            label: kind.to_owned(),
            user: whoami::username(),
            timestamp: UTime::now(),
            comment: String::default(),
            config: self.edit.next_config(),
            extra: HashMap::new(),
        };
        let commit = Rc::new(Edit {
            meta: RefCell::new(meta),
            edit: RefCell::new(edit_factory(kind, id)?),
            rom: self.edit.rom.clone(),
            memory: self.edit.memory.clone(),
            connectivity: self.edit.connectivity.clone(),
            action: RefCell::default(),
            error: RefCell::default(),
        });
        Ok(EditProxy::new(commit))
    }

    fn unpack(&self, proxy: Option<&EditProxy>) -> PyResult<()> {
        let proxy = proxy.unwrap_or(self);
        let mut obj = proxy.edit.edit.borrow_mut();
        obj.unpack(&proxy.edit)?;
        Ok(())
    }

    fn pack(&self) -> PyResult<()> {
        let obj = self.edit.edit.borrow();
        obj.pack(&self.edit)?;
        Ok(())
    }

    fn read(&self, addr: PyAddress) -> Result<u8> {
        self.edit.rom.borrow().read(addr.address)
    }

    fn read_word(&self, addr: PyAddress) -> Result<u16> {
        self.edit.rom.borrow().read_word(addr.address)
    }
    fn read_pointer(&self, addr: PyAddress) -> Result<PyAddress> {
        let ptr = self.edit.rom.borrow().read_pointer(addr.address)?;
        Ok(PyAddress::from(ptr))
    }

    fn read_bytes<'p>(&self, py: Python<'p>, addr: PyAddress, len: usize) -> Result<&'p PyBytes> {
        let rom = self.edit.rom.borrow();
        let bytes = rom.read_bytes(addr.address, len)?;
        Ok(PyBytes::new(py, bytes))
    }

    fn write(&self, addr: PyAddress, val: u8) -> Result<()> {
        self.edit.rom.borrow_mut().write(addr.address, val)
    }

    fn write_word(&self, addr: PyAddress, val: u16) -> Result<()> {
        self.edit.rom.borrow_mut().write_word(addr.address, val)
    }

    fn write_pointer(&self, addr: PyAddress, ptr: PyAddress) -> Result<()> {
        self.edit
            .rom
            .borrow_mut()
            .write_pointer(addr.address, ptr.address)
    }

    fn write_bytes(&self, addr: PyAddress, val: &[u8]) -> Result<()> {
        self.edit.rom.borrow_mut().write_bytes(addr.address, val)
    }
    fn write_terminated(&mut self, addr: PyAddress, val: &[u8], terminator: u8) -> Result<()> {
        self.edit
            .rom
            .borrow_mut()
            .write_terminated(addr.address, val, terminator)
    }

    fn alloc(&mut self, addr: PyAddress, length: u16) -> Result<PyAddress> {
        let mut mem = self.edit.memory.borrow_mut();
        let addr = mem.alloc(addr.address, length)?;
        Ok(PyAddress::from(addr))
    }

    fn alloc_near(&mut self, addr: PyAddress, length: u16) -> Result<PyAddress> {
        let mut mem = self.edit.memory.borrow_mut();
        let addr = mem.alloc_near(addr.address, length)?;
        Ok(PyAddress::from(addr))
    }

    fn alloc_at(&mut self, addr: PyAddress, length: u16) -> Result<PyAddress> {
        let mut mem = self.edit.memory.borrow_mut();
        let addr = mem.alloc_at(addr.address, length)?;
        Ok(PyAddress::from(addr))
    }

    fn free(&mut self, addr: PyAddress, length: u16) {
        let mut mem = self.edit.memory.borrow_mut();
        mem.free(addr.address, length);
    }

    fn report(&self, addr: PyAddress) -> Result<(usize, u16)> {
        let mem = self.edit.memory.borrow();
        mem.report(addr.address)
    }

    fn overworld_connector(&self, id: &str) -> Option<String> {
        let id = IdPath::from(id);
        self.edit
            .connectivity
            .borrow()
            .overworld_connector(&id)
            .map(|s| s.to_string())
    }
}

#[pymethods]
impl Project {
    fn save(&self, filename: &str) -> PyResult<()> {
        self.to_file(&Path::new(filename)).map_err(|e| e.into())
    }
}

#[pyproto]
impl PySequenceProtocol for Project {
    fn __len__(&self) -> usize {
        self.edits.len()
    }

    fn __getitem__(&self, index: isize) -> PyResult<EditProxy> {
        match self.get_commit(index) {
            Ok(edit) => Ok(EditProxy { edit: edit }),
            Err(_) => Err(PyIndexError::new_err("list index out of range")),
        }
    }
}
