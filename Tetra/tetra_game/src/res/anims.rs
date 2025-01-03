use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};
//从json库中加载数据

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureAtlas {
    frames: Vec<Frame>,
    meta: Meta,
}


#[derive(Debug, Serialize, Deserialize)]
struct Frame {
    filename: String,
    frame: Rectangle,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: Rectangle,
    #[serde(rename = "sourceSize")]
    source_size: Size,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Size {
    w: i32,
    h: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: Size,
    scale: String,
    #[serde(rename = "smartupdate")]
    smart_update: String,
}

pub fn get_anim() -> Result<TextureAtlas,Box<dyn Error>>{

    //编译后相对路径改变
    let json_str = include_str!("../../resources/anim/anim.json");
    // let file = File::open("../../resources/anim/anim.json")?;
    // let reader = BufReader::new(file);
    let anim = serde_json::from_str(json_str)?;
    Ok(anim)
}