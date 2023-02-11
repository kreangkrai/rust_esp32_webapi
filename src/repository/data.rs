use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{DataDevice};
use chrono::{DateTime,Utc,Local};

pub async fn gets()->Result<Vec<DataDevice>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut datas :Vec<DataDevice> = vec![];
    let mut id:i32;
    let mut device:String;
    let mut status:bool;
    let mut date:DateTime<Local>;
    let rows = client.query("select * from mee order by id", &[]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        status = row.get(2);
        date = row.get(3);
                        
        datas.push(DataDevice{id:id,device:device,status:status,date:date.format("%Y-%m-%d %H:%M:%S").to_string()});
    } 
    Ok(datas)      
}
pub async fn getbydevice(_device:String)->Result<DataDevice,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut datas :Vec<DataDevice> = vec![];
    let mut id:i32;
    let mut device:String;
    let mut status:bool;
    let mut date:DateTime<Local>;
    let rows = client.query("select * from mee where device = $1", &[&_device]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        status = row.get(2);
        date = row.get(3);
                        
        datas.push(DataDevice{id:id,device:device,status:status,date:date.format("%Y-%m-%d %H:%M:%S").to_string()});
    }
    
    Ok(datas[0].to_owned())      
}
pub async fn insert(p : DataDevice)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let datetime = Utc::now();
    let command = format!("insert into mee (device,status,date) SELECT $1,$2,$3 WHERE NOT EXISTS (SELECT 1 FROM mee WHERE device = $1)");
    
    let rows = client.execute(&command, &[&p.device,&p.status,&datetime]).await?;
    Ok(rows)      
}
pub async fn update(d: DataDevice)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let datetime = Utc::now();
    let command = format!("update mee set status=$1,date=$2 WHERE device = $3");
    
    let rows = client.execute(&command, &[&d.status,&datetime,&d.device]).await?;
    Ok(rows)    
}