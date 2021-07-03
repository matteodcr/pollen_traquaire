#[macro_use]
extern crate rocket;

mod scrapper;

use rocket::fs::FileServer;
use rocket::response::content::Html;
use std::fmt::Write;

#[get("/")]
fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
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
            "Noisettier" => self.noisetier,
            "Plantain" => self.plantain,
            "Olivier" => self.olivier,
            "Ambroisie" => self.ambroisie,
            "Tilleul" => self.tilleul,
            _ => false,
        }
    }
}

#[get("/resultat?<form..>")]
async fn resultat(form: PollenParams) -> String {
    let mut response = String::new();
    let departements = scrapper::scrap().await;
    let dep = departements
        .get(&form.county)
        .expect("Département non connu");

    writeln!(
        response,
        "Département : {} => Risque Global [{}/4]",
        dep.county_name, dep.risk_level
    )
    .unwrap();
    for risk in dep
        .risks
        .iter()
        .filter(|risk| form.contains_pollen(&risk.pollen_name))
    {
        writeln!(
            response,
            "      {} : Risque [{}/4]",
            risk.pollen_name, risk.level
        )
        .unwrap();
    }
    response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, resultat])
        .mount("/static", FileServer::from("src/static"))
}
