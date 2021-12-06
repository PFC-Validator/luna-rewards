use chrono::Utc;
use rand_core::RngCore;
use rand_seeder::{Seeder, SipRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
pub struct DelegationShares {
    pub delegator_address: String,
    pub validator_address: String,
    pub shares: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Winner {
    pub delegator_address: String,
    pub shares: String,
    pub date: String,
}
#[derive(Deserialize, Serialize)]
pub struct WinnerFile {
    pub winners: Vec<Winner>,
}

#[derive(Deserialize)]
pub struct Delegation {
    pub delegation: DelegationShares,
}
#[derive(Deserialize)]
pub struct Delegations {
    pub delegation_responses: Vec<Delegation>,
}

fn main() {
    let min_luna = 1_000_000_f64; // 1 luna
    let max_luna = 10_000_000_000_f64; // 10k luna
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <hash> <delegation_file> <winners_file>", args[0]);
        return;
    }
    let hash = &args[1];
    let mut rng: SipRng = Seeder::from(hash).make_rng();

    let delegation_file = &args[2];
    let winner_file = &args[3];
    let delegations: Delegations =
        serde_json::from_reader(std::fs::File::open(delegation_file).unwrap()).unwrap();
    let mut winners: WinnerFile =
        serde_json::from_reader(std::fs::File::open(winner_file).unwrap()).unwrap();

    let viable = delegations
        .delegation_responses
        .iter()
        .filter(|d| {
            let shares = d.delegation.shares.parse::<f64>().unwrap();
            shares >= min_luna && shares <= max_luna
        })
        .collect::<Vec<_>>();

    let winner_map: HashMap<String, Winner> = winners
        .winners
        .iter()
        .map(|w| (w.delegator_address.clone(), w.clone()))
        .collect::<HashMap<_, _>>();

    let mut cnt = 0;
    let now = Utc::now();
    let mut found: bool = false;
    while !found {
        let choice = rng.next_u32() % viable.len() as u32;

        let potential_winner = viable[choice as usize];
        cnt += 1;
        if let Some(already_won) = winner_map.get(&potential_winner.delegation.delegator_address) {
            println!(
                "{} Skipped as they have already won on {}",
                already_won.delegator_address, already_won.date
            );
            continue;
        }
        if cnt > 100 {
            panic!("Tried {} times, giving up", cnt);
        }
        found = true;
        let winner = Winner {
            delegator_address: potential_winner.delegation.delegator_address.clone(),
            shares: potential_winner.delegation.shares.clone(),
            date: format!("{}", now.format("%Y-%m-%d %T")),
        };
        winners.winners.push(winner.clone());
        serde_json::to_writer(std::fs::File::create(winner_file).unwrap(), &winners).unwrap();
        println!(
            "{} - #Delegations {} - #Viable {} - {} {:10.0}",
            delegation_file,
            delegations.delegation_responses.len(),
            viable.len(),
            winner.delegator_address,
            winner.shares.parse::<f64>().unwrap() / 1_000_000_f64,
        );
    }
}
