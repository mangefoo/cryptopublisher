use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use reqwest::blocking;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coin {
    pub id: String,
    pub current_price: f32,
    pub ath: f32
}

fn main() {
    //let json = r#"[{"id":"bitcoin","symbol":"btc","name":"Bitcoin","image":"https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579","current_price":50247,"market_cap":944678319215,"market_cap_rank":1,"fully_diluted_valuation":1054877223734,"total_volume":38996706542,"high_24h":50913,"low_24h":49624,"price_change_24h":-558.38518070073,"price_change_percentage_24h":-1.09907,"market_cap_change_24h":-9401083448.186401,"market_cap_change_percentage_24h":-0.98536,"circulating_supply":18806212.0,"total_supply":21000000.0,"max_supply":21000000.0,"ath":64805,"ath_change_percentage":-22.48675,"ath_date":"2021-04-14T11:54:46.763Z","atl":67.81,"atl_change_percentage":73979.02896,"atl_date":"2013-07-06T00:00:00.000Z","roi":null,"last_updated":"2021-09-04T13:05:10.764Z"},{"id":"ethereum","symbol":"eth","name":"Ethereum","image":"https://assets.coingecko.com/coins/images/279/large/ethereum.png?1595348880","current_price":3919.03,"market_cap":459820933068,"market_cap_rank":2,"fully_diluted_valuation":null,"total_volume":27486736610,"high_24h":3995.2,"low_24h":3878.05,"price_change_24h":-41.960263343612,"price_change_percentage_24h":-1.05934,"market_cap_change_24h":-4778874327.082397,"market_cap_change_percentage_24h":-1.0286,"circulating_supply":117385448.624,"total_supply":null,"max_supply":null,"ath":4356.99,"ath_change_percentage":-10.11152,"ath_date":"2021-05-12T14:41:48.623Z","atl":0.432979,"atl_change_percentage":904432.03734,"atl_date":"2015-10-20T00:00:00.000Z","roi":{"times":103.26347399424486,"currency":"btc","percentage":10326.347399424485},"last_updated":"2021-09-04T13:05:11.193Z"}]"#;

    loop {
        get_crypto_rates();
        thread::sleep(Duration::from_secs(60));
    }
}

fn get_crypto_rates() {
    let request_url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=ethereum,bitcoin,terra-luna";

    let response = blocking::Client::new()
        .get(request_url)
        .send();

    match response {
        Err(error) => { println!("Request failed: {}", error); },
        Ok(response) => { handle_response(response) }
    };

}

fn handle_response(response: Response) {
    match response.json::<Vec<Coin>>() {
        Ok(coins) => {
            println!("Coins{:?}", coins);
            publish_rates(coins);
        }
        Err(error) => {
            println!("Parse json failed: {:?}", error)
        }
    }
}

fn publish_rates(coins: Vec<Coin>) {
    let request_url = format!("http://sensor-relay.int.mindphaser.se/publish");
//    let request_url = format!("http://localhost:8967/publish");

    let mut request_body = HashMap::<String, String>::new();

    let bitcoin = coins.iter()
        .find(|c| { c.id == "bitcoin" });

    if bitcoin.is_some() {
        request_body.insert("bitcoin_price".to_string(), format!("{}", bitcoin.unwrap().current_price));
    }

    let ethereum = coins.iter()
        .find(|c| { c.id == "ethereum" });

    if ethereum.is_some() {
        request_body.insert("ethereum_price".to_string(), format!("{}", ethereum.unwrap().current_price));
    }

    let luna = coins.iter()
        .find(|c| { c.id == "terra-luna" });
    if luna.is_some() {
        request_body.insert("luna_price".to_string(), format!("{}", luna.unwrap().current_price));
    }

    let body = json!({
            "reporter": "crypto-publisher",
            "sensors": request_body,
            "topic": "sensors"
        });

    let post_response = blocking::Client::new()
        .post(request_url)
        .json(&body)
        .send();

    if post_response.is_err() {
        println!("Failed to send update to server: {}", post_response.unwrap_err())
    } else {
        println!("Published rates OK {:?}", request_body);
    }
}
