use anyhow::Result;
use diffres::Config;

fn main()->Result<()>{

    let content=include_str!("../fixtures/test.yml");
    
    let config = Config::from_yaml(content)?;
    
    println!("get config data is:{:#?}",config);


   Ok(())
   
    //get data-->caculate
    
}

