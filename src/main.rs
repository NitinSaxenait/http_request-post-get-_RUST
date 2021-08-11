use reqwest::Error;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://connect.cloud.kaaiot.com/kp1/c3s1v83efgnu0bisart0-v1/dcx/testToken/json";
    //let result = fetch(url).await?;
    let result = post(url).await?;

    println!("Status {}", result.status());
    println!("Headers \n{:#?}", result.headers());
    println!("Body \n{}", result.text().await?);
    Ok(())
}
/// to get the data from http;
async fn fetch(url: &str) -> Result<(reqwest::Response), Error> {
    let response = reqwest::get(url).await?;
    Ok(response)
}
/// to post in different form;
/*
async fn post(url:&str) -> Result<reqwest::Response,Error>{
    let params = [("temperature","21"), ("humidity","32")];
    let client =  reqwest::Client::new();
    let response = client.post(url)
        .form(&params)
        .send()
        .await?;

    Ok(response)
}
*/
/// to post the data to the http;
async fn post(url: &str) -> Result<(reqwest::Response), Error> {
    let mut map = HashMap::new();
    map.insert("temperature:", "26");
    map.insert("humidity:", "52");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&map).send().await?;
    Ok(response)
}
