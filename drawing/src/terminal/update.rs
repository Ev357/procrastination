use color_eyre::eyre::Result;

use crate::{message::Message, model::Model, terminal::Terminal};

impl Terminal {
    pub fn update<T>(&mut self, update: T) -> Result<Option<Message>>
    where
        T: FnOnce(&mut Model) -> Option<Message>,
    {
        let mut model = self.model.clone();
        update(&mut model);
        self.render(&model)?;
        self.model = model;

        Ok(None)
    }
}
