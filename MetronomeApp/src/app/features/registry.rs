use std::collections::BTreeMap;

use crate::app::features::metronome::Home;
use crate::app::features::shell::Menu;
use crate::app::features::traits::Drawable;

pub struct Registry {
    map: BTreeMap<Menu, Box<dyn Drawable>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut map: BTreeMap<Menu, Box<dyn Drawable>> = BTreeMap::new();

        map.insert(Menu::Home, Box::new(Home) as Box<dyn Drawable>);

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