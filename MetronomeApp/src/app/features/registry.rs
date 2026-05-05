use std::collections::BTreeMap;

use crate::app::features::{
    accents::Accents,
    metronome::Home,
    shell::Menu,
    traits::Drawable,
};

pub struct Registry {
    map: BTreeMap<Menu, Box<dyn Drawable>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut map: BTreeMap<Menu, Box<dyn Drawable>> = BTreeMap::new();

        map.insert(Menu::Home, Box::new(Home) as Box<dyn Drawable>);
        map.insert(Menu::Accents, Box::new(Accents) as Box<dyn Drawable>);

        Self { map }
    }

    pub fn get_mut(&mut self, menu: Menu) -> &mut dyn Drawable {
        self.map
            .get_mut(&menu)
            .expect("menu registered")
            .as_mut()
    }

    pub fn get_all_keys(&self) -> Vec<Menu> {
        self.map.keys().copied().collect()
    }
}