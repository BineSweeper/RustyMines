pub(crate) struct Slot {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub mine_count: i32,
}

impl Slot {
    pub(crate) fn new() -> Slot {
        Slot {
            is_mine: false,
            is_revealed: false,
            mine_count: 0,
        }
    }

    pub(crate) fn reveal(&mut self) {
        self.is_revealed = true;
    }

    pub fn set_mine(&mut self) {
        self.is_mine = true;
    }

    pub(crate) fn is_blank(&self) -> bool {
        self.mine_count == 0
    }

    pub(crate) fn description(&self) -> String {
        if self.is_revealed {
            if self.is_mine {
                "X".to_string()
            } else if self.is_blank() {
                " ".to_string()
            } else {
                self.mine_count.to_string()
            }
        } else {
            "■".to_string()
        }
    }
}
