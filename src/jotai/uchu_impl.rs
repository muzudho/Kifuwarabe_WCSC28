use super::super::jotai::uchu::*;
use super::super::teigi::conv::*;
use super::super::teigi::shogi_syugo::*;

impl Uchu {
    /// らいおんの位置
    pub fn get_ms_r(&self, jiai: &Jiai) -> umasu {
        self.ky.ms_r[sn_to_num(&self.get_teban(jiai))]
    }
}
