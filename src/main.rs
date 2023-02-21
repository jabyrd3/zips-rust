use http_types::mime::JSON;
use tide::{Request, Response};

use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Zip {
    zip: u32,
    city: String,
    state: String,
    abbr: String,
    long: f32,
    lat: f32
}

#[derive(Clone,Debug)]
struct State {
    zips: Arc<HashMap<String,Zip>>
}

impl State {
    fn new(hm: HashMap<String, Zip>) -> Self {
        Self {
            zips: Arc::new(hm),
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut zip_map: HashMap<String, Zip> = HashMap::<String, Zip>::new();
    let mut rdr = csv::Reader::from_path("./zips.csv")?;
    for result in rdr.deserialize() {
        let record: Zip = result?;
        zip_map.insert(record.zip.to_string(), record);
    }
    let mut app = tide::with_state(State::new(zip_map));
    app.at("/zips/:zip").get(zips);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn zips(req: Request<State>) -> tide::Result {
    let zip = req.param("zip").unwrap_or("11215");
    let zips = &req.state().zips;
    let zip_struct = &zips[zip];
    let ret_string = serde_json::to_string(&zip_struct).unwrap();
    let response = Response::builder(200).body(ret_string).content_type(JSON);

    Ok(response.into())
}