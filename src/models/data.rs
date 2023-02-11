use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct DataDevice{
    #[serde(skip_deserializing)]
    pub id:i32,
    pub device:String,
    pub status :bool,
    #[serde(skip_deserializing)]
    pub date: String,
}