///////////////////////////////////////////////////////////////////////////////

use pages::Page;
use std::path::PathBuf;

use self::{
    modals::Modal,
    pages::Project,
    widgets::{modal_view::modal_view, page_view::page_view, top_bar::top_bar},
};

///////////////////////////////////////////////////////////////////////////////

pub mod modals;
pub mod pages;
pub mod widgets;

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    active: Page,
    recent: Vec<PathBuf>,

    #[serde(skip)]
    modals: Vec<Modal>,
}

//---------------------------------------------------------------------------//

impl App {
    //-------------------------------------------------------------------------//

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        match cc.storage {
            Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            _ => Default::default(),
        }
    }

    //-------------------------------------------------------------------------//

    pub fn new_graph(&mut self) {
        self.active = Page::Project(Project::new());
    }

    //-------------------------------------------------------------------------//

    pub fn find_graph(&mut self) {
        todo!()
    }

    //-------------------------------------------------------------------------//

    pub fn open_graph(&mut self) {
        todo!()
    }

    //-------------------------------------------------------------------------//

    pub fn close_graph(&mut self) {
        self.active = Page::Blank;
    }

    //-------------------------------------------------------------------------//
}

//---------------------------------------------------------------------------//

impl Default for App {
    fn default() -> Self {
        Self {
            active: Page::Blank,
            recent: vec![],
            modals: vec![],
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        top_bar(self, ctx);
        page_view(self, ctx);
        modal_view(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

///////////////////////////////////////////////////////////////////////////////
