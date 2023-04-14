use serde_json::{json,Map,Value};
use tide::{Body ,Response};

#[derive(serde::Serialize)]
struct WebServiceResponse{
    status : String,
    info : String,
}

pub fn vec_to_string(vec : Vec<&str>) -> String {
    let mut res = String::new();
    
    for (i,el) in vec.into_iter().enumerate() {
        if i != 0 {res.push_str(",")}
        res.push_str("'");
        res.push_str(el);
        res.push_str("'");
    }

    res
}

pub fn to_json(data : impl serde::Serialize) -> tide::Result<serde_json::Value> {
    Ok(json!(data))
}

pub fn ws_response(status_val : &str,info_val : &str) -> tide::Result<Response>{
    let mut res = Response::new(200);
    let data = WebServiceResponse{
        status : status_val.into(), info : info_val.into() };
        res.set_body(Body::from_json(&data)?);
    Ok(res)
}

pub fn ok_with_data(info : &str,key :&str, value : Value) -> tide::Result<Response>{

    let mut res = Response::new(200);
    let mut data = Map::new();
    data.insert("status".into(), Value::from("Ok".to_string()));
    data.insert("info".into(), Value::from(info.to_string()));
    data.insert(key.into(),value);
    res.set_body(Body::from_json(&data)?);
    Ok(res)
    

}