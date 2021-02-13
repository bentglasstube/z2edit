use std::any::Any;
use std::convert::From;
use std::path::PathBuf;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::nes::Buffer;
use crate::nes::IdPath;
use crate::util::relative_path::PathConverter;
use crate::zelda2::config::Config;
use crate::zelda2::project::{Edit, EditProxy, RomData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileResource {
    Unknown,
    Vanilla,
    Name(PathConverter),
}

impl Default for FileResource {
    fn default() -> Self {
        FileResource::Unknown
    }
}

impl FileResource {
    pub fn to_pathbuf(&self) -> Result<PathBuf> {
        let pref = AppContext::pref();
        match self {
            FileResource::Unknown => Err(ErrorKind::NotImplemented(
                "No path for FileResource::Unknown".to_string(),
            )
            .into()),
            FileResource::Vanilla => {
                if !(pref.vanilla_rom.exists() && pref.vanilla_rom.is_file()) {
                    Err(ErrorKind::ConfigError(
                        "Set the location of the vanilla ROM in Prefences".to_string(),
                    )
                    .into())
                } else {
                    Ok(PathBuf::from(&pref.vanilla_rom))
                }
            }
            FileResource::Name(name) => Ok(PathBuf::from(name)),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImportRom {
    pub id: IdPath,
    pub file: FileResource,
}

impl ImportRom {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
    pub fn from_file(filename: &str) -> Result<Box<ImportRom>> {
        ImportRom::new(FileResource::Name(PathConverter::from(filename)))
    }

    pub fn new(file: FileResource) -> Result<Box<ImportRom>> {
        match &file {
            FileResource::Name(filename) => {
                if !filename.as_ref().is_file() {
                    return Err(ErrorKind::NotFound(format!("{:?}", filename)).into());
                }
            }
            FileResource::Unknown => {
                return Err(ErrorKind::NotImplemented(format!("Cannot load {:?}", file)).into());
            }
            FileResource::Vanilla => {}
        };

        Ok(Box::new(ImportRom {
            id: IdPath::default(),
            file: file,
        }))
    }
}

const FIX_SCRIPT: &str = r#"from z2edit import fix
try:
    fix.fix_all(edit)
except Exception as e:
    print("ImportRom: ", e)
    raise e
"#;

#[typetag::serde]
impl RomData for ImportRom {
    fn name(&self) -> String {
        "ImportRom".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, _edit: &Rc<Edit>) -> Result<()> {
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let config = Config::get(&edit.config())?;
        let path = self.file.to_pathbuf()?;
        info!("ImportRom: loading file {:?}", path);
        let rom = Buffer::from_file(&path, Some(config.layout.clone()))?;
        edit.rom.replace(rom);

        info!("ImportRom: applying default fixes");
        let gil = Python::acquire_gil();
        let py = gil.python();
        let proxy = Py::new(py, EditProxy::new(Rc::clone(edit)))?;
        let locals = PyDict::new(py);
        locals.set_item("edit", proxy)?;
        py.run(FIX_SCRIPT, None, Some(locals))?;
        AppContext::app().borrow(py).process_python_output();
        Ok(())
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
