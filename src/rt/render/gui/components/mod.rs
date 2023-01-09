use doryen_rs::Console;

pub trait Component {
    fn render (&self, con: &mut Console);
}