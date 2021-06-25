mod scrapper;

use clap::Clap;

#[derive(Clap, Debug)]
struct Args {
    departement: u8,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let departements = scrapper::scrap().await;
    let dep = departements
        .get(&args.departement)
        .expect("Département non connu");

    println!(
        "Département : {} => Risque Global [{}/4]",
        dep.county_name, dep.risk_level
    );
    for risk in dep.risks.iter().filter(|risk| risk.level != 0) {
        println!("      {} : Risque [{}/4]", risk.pollen_name, risk.level);
    }
}
