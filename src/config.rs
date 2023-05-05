use anyhow::Result;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use reqwest::{Method, header::HeaderMap};
use url::Url;
use tokio::fs;

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Config {
    #[serde(flatten)]
    pub profile:HashMap<String,Profile>
}



#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Profile {
    pub req1:ReqwestProfile,
    pub req2:ReqwestProfile,
    pub res:ResponseProfile,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct ReqwestProfile {
    #[serde(with = "http_serde::method",default)]
    pub method:Method,
    pub url:Url,
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub params:Option<serde_json::Value>,
    #[serde(skip_serializing_if="HeaderMap::is_empty",with = "http_serde::header_map",default)]
    pub headers:HeaderMap,
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub body:Option<serde_json::Value>,

}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if="Vec::is_empty",default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if="Vec::is_empty",default)]
    pub skip_body: Vec<String>,

}

impl Config {
    //从配置文件读
    pub async fn local_yaml(path:&str) -> Result<Self>{
        let con=fs::read_to_string(path).await?;
    
        Self::from_yaml(&con)
    
    }
    //从字符串读
    pub fn from_yaml(content:&str) -> anyhow::Result<Self>{
        
        Ok(serde_yaml::from_str(content).unwrap())
        
    }
}


