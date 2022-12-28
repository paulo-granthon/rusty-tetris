use doryen_rs::Console;

pub const BUTTON_HEIGHT: u32 = 5; 

pub fn render_button (
    con: &mut Console,
    x: i32, y: i32, w: u32,
    text: &str, color: (u8, u8, u8, u8),
    fore: Option<(u8, u8, u8, u8)>, back: Option<(u8, u8, u8, u8)>
) {
    let width = w.max(text.len() as u32 + 4);
    con.rectangle(x - (width as i32 / 2), y - (BUTTON_HEIGHT as i32 / 2), width, BUTTON_HEIGHT, fore, back, Some('+' as u16));
    con.rectangle(x - (width as i32 / 2), y - (BUTTON_HEIGHT as i32 / 2), width, BUTTON_HEIGHT, Some(color), None, None);
    con.print(x, y, text, doryen_rs::TextAlign::Center, Some(color), back);
}