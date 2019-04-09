use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

const TITLE: &str = " Batteries ";

pub struct BatteryWidget {}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {}
    }
}

impl Widget for BatteryWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(TITLE).draw(area, buf);
    }
}
