use doryen_rs::Console;

use crate::components::*;

// holds a list of components to render
pub struct Screen {
    components: Vec<Box<dyn Component>>,
}

impl Screen {

    pub fn render (&self, con: &mut Console) {
        for component in &self.components {
            component.render(con);
        }
    }
}
