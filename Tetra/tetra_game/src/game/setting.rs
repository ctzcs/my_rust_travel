use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct GameSetting{
    pub window_width:f32,
    pub window_height:f32,
    pub version:String,
}

impl GameSetting{
    pub fn new()->GameSetting{
        GameSetting{
            window_width:1920.0,
            window_height:1080.0,
            version:String::from("v0.1"),
        }
    }
}


lazy_static!{
    pub static ref GAME_SETTING:Mutex<GameSetting> = Mutex::new(GameSetting::new());
    pub static ref TIME_SETTING:Mutex<TimeSetting> = Mutex::new(TimeSetting::new());
}



pub struct TimeSetting{
    pub elapsed_time:f32,
    pub delta_time:f32,
    pub fixed_delta_time:f32,
}

impl TimeSetting{
    pub fn new()->TimeSetting{
        TimeSetting{
            elapsed_time:0.00,
            delta_time:0.02,
            fixed_delta_time:0.02,
        }
    }
}
