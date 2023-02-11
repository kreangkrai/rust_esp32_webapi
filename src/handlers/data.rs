use actix_web::{http,web,HttpResponse,HttpRequest};
use crate::models::{DataDevice};
use crate::repository::{data};
use crate::errors::MyError;
pub async fn get_datas(req: HttpRequest) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::ACCEPT) {
        if let Ok(_s) = hdr.to_str() {
            let data = data::gets().await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn get_data(req: HttpRequest,device :web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::ACCEPT) {
        if let Ok(_s) = hdr.to_str() {
            let data = data::getbydevice(device.into_inner()).await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_data(req: HttpRequest,_data:web::Json<DataDevice>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::ACCEPT) {
        if let Ok(_s) = hdr.to_str() {
            let p:DataDevice = DataDevice{id:0,device:_data.device.to_string(),status:_data.status,date:String::from("")};
            let data = data::insert(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn update_data(req: HttpRequest,_data:web::Json<DataDevice>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::ACCEPT) {
        if let Ok(_s) = hdr.to_str() {
            let d:DataDevice = DataDevice{id:0,device:_data.device.to_string(),status:_data.status,date:String::from("")};
            let data = data::update(d).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}