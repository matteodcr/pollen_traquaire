#[macro_use]
extern crate rocket;

mod scrapper;

use crate::scrapper::{DepartementNum, Risk};
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("index.html"))
}

#[derive(FromForm)]
struct PollenParams {
    county: u8,
    cupressacees: bool,
    saule: bool,
    frene: bool,
    peuplier: bool,
    charme: bool,
    bouleau: bool,
    platane: bool,
    chene: bool,
    graminees: bool,
    oseille: bool,
    urticacees: bool,
    aulne: bool,
    noisetier: bool,
    plantain: bool,
    olivier: bool,
    ambroisie: bool,
    tilleul: bool,
}

impl PollenParams {
    pub fn contains_pollen(&self, pollen_name: &str) -> bool {
        match pollen_name {
            "Cupressacées" => self.cupressacees,
            "Saule" => self.saule,
            "Frêne" => self.frene,
            "Peuplier" => self.peuplier,
            "Charme" => self.charme,
            "Bouleau" => self.bouleau,
            "Platane" => self.platane,
            "Chêne" => self.chene,
            "Graminées" => self.graminees,
            "Oseille" => self.oseille,
            "Urticacées" => self.urticacees,
            "Aulne" => self.aulne,
            "Noisetier" => self.noisetier,
            "Plantain" => self.plantain,
            "Olivier" => self.olivier,
            "Ambroisies" => self.ambroisie,
            "Tilleul" => self.tilleul,
            _ => false,
        }
    }
}

#[get("/resultat?<form..>")]
async fn resultat(form: PollenParams) -> Template {
    #[derive(serde::Serialize)]
    struct Context<'a> {
        county: &'a str,
        county_risk: u8,
        risks: &'a [&'a Risk],
    }

    let departements = scrapper::scrap().await;
    let dep = departements
        .get(&DepartementNum(form.county))
        .expect("Département non connu");

    println!("{:?}", dep);

    let pollen = dep
        .risks
        .iter()
        .filter(|risk| form.contains_pollen(&risk.pollen_name))
        .collect::<Vec<_>>();

    Template::render(
        "index",
        &Context {
            county: &*dep.county_name,
            county_risk: dep.risk_level,
            risks: &pollen[..],
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, resultat])
        .mount("/static", FileServer::from("src/static"))
        .attach(Template::fairing())
}
