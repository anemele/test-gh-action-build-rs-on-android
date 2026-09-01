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

fn visit(hero: &Hero) -> String {
    format!(
        r#"\
found: {}-{}
visit by id_name:{} or ename:{}
skins: {}"#,
        hero.title, hero.cname, hero.id_name, hero.ename, hero.skin_name,
    )
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

    let bs = ureq::get(HEROLIST_URL)
        .call()
        .unwrap()
        .body_mut()
        .read_to_vec()
        .unwrap();
    let herolist = serde_json::from_slice::<HeroList>(bs.as_slice()).unwrap();

    match cmd {
        List => {
            println!("{:#?}", herolist);
        }
        Download { name } => {
            let hero = herolist
                .iter()
                .find(|h| h.cname == name || h.title == name)
                .unwrap();
            println!("{}", visit(hero))
        }
    }
}
