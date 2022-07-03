use std::env;
use std::io::Read;
use std::fs::{self, File};
use std::path::PathBuf;
use std::{thread, time};
use serenity::model::id::ChannelId;
use serenity::http::client::Http;

async fn get_ip() -> String {
    for _i in 0..3 {
        if let Some(ip) = public_ip::addr_v4().await {
            return ip.to_string();
        }
        let ten_millis = time::Duration::from_millis(3000);
        thread::sleep(ten_millis);
    }
    "couldn't get an IP address".to_string()
}
fn get_file(dir : PathBuf) -> String {
    let mut file = File::open(dir).unwrap();
    let mut result = String::new();
    file.read_to_string(&mut result).unwrap();
    result
}
fn get_random_message() -> String {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); dir.pop(); dir.pop();
    dir.push("replique_chat_potte");
    
    let all_rep = get_file(dir);
    let repliques = all_rep.split('\n');
    let count_rep  = all_rep.split('\n').count() as f32;

    let rand: i32 = (rand::random::<f32>() * count_rep) as i32;
    let mut i  = 0;
    let mut final_string = String::new();
    for x in repliques {
        if i == rand {
            final_string = String::from(x);
        }
        i+=1;
    }
    final_string
}
fn build_message(ip : String) -> String {
    let mut replique = get_random_message();
    replique.push_str("\n```");
    replique.push_str(ip.as_str());
    replique.push_str("```");
    replique
}
#[tokio::main]
async fn main() {
    //Get the current ip address
    let ip = get_ip().await;
    
    let mut dir = env::current_exe().unwrap();
    dir.pop(); dir.pop(); dir.pop();
    let mut dir_ip = dir.clone();
    dir.push("token");
    dir_ip.push("last_ip");
    let last_ip = get_file(dir_ip.clone());

    if !ip.eq(last_ip.as_str()) {
        let token = get_file(dir);
        let message = build_message(ip.clone());
        
        let client_http = Http::new(token.as_str());
        let channel_id = ChannelId(992922304185122848);
        channel_id.say(client_http, message).await.unwrap();
        fs::write(dir_ip, ip).unwrap();
    }
    
}