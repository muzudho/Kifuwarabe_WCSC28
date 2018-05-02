use teigi::conv::*;
use teigi::shogi_syugo::*;
use jotai::uchu::*;

impl Uchu {

    /**
     * らいおんの位置
     */
    pub fn get_ms_r( &self, jiai:&Jiai ) -> umasu {
        self.ky.ms_r[ sn_to_num(&self.get_teban(jiai)) ]
    }
}