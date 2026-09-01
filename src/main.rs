use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Hero {
    ename: u16,
    cname: String,
    id_name: String,
    title: String,
    skin_name: String,
}

type HeroList = Vec<Hero>;

#[derive(Parser)]
enum Cli {
    List,
    Download { name: String },
}

const HEROLIST_URL: &str = "https://pvp.qq.com/web201605/js/herolist.json";

fn main() {
    use Cli::*;
    let cmd = Cli::parse();

    let resp = tinyget::get(HEROLIST_URL).send().unwrap();
    let herolist = serde_json::from_slice::<HeroList>(resp.as_bytes()).unwrap();

    match cmd {
        List => {
            dbg!(herolist);
        }
        Download { name } => {
            let hero = herolist.iter().find(|h| h.cname == name).unwrap();
            dbg!(hero);
        }
    }
}
