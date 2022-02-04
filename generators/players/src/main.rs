use std::collections::HashMap;

use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use ::reqwest::blocking::Client;
use anyhow::Error;

use rand;
use rand::distributions::{Distribution, Uniform};

use common::enums::{Class, Race, Gender, Trait};
use common::structs::{Abilities};

const SERVER: &str = "http://127.0.0.1:4000/";
const PER_PLAYER: i64 = 64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "graphql/leagues.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct Leagues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "graphql/create_player.graphql",
    variables_derive = "Debug",
    response_derive = "Debug",
)]
pub struct CreatePlayer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "graphql/set_league_state.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct SetLeagueState;


impl From<Class> for create_player::Class {
    fn from(item: Class) -> Self {
        match item {
            Class::Bard => create_player::Class::BARD,
            Class::Cleric => create_player::Class::CLERIC,
            Class::Druid => create_player::Class::DRUID,
            Class::Fighter => create_player::Class::FIGHTER,
            Class::Paladin => create_player::Class::PALADIN,
            Class::Ranger => create_player::Class::RANGER,
            Class::Rogue => create_player::Class::ROGUE,
            Class::Wizard => create_player::Class::WIZARD,
        }
    }
}


impl From<Race> for create_player::Race {
    fn from(item: Race) -> Self {
        match item {
            Race::Dwarf => create_player::Race::DWARF,
            Race::Elf => create_player::Race::ELF,
            Race::Goblin => create_player::Race::GOBLIN,
            Race::Halfling => create_player::Race::HALFLING,
            Race::Human => create_player::Race::HUMAN,
            Race::Orc => create_player::Race::ORC,
        }
    }
}

impl From<Gender> for create_player::Gender {
    fn from(item: Gender) -> Self {
        match item {
            Gender::Male => create_player::Gender::MALE,
            Gender::Female => create_player::Gender::FEMALE,
        }
    }
}

impl From<Trait> for create_player::Trait {
    fn from(item: Trait) -> Self {
        match item {
            Trait::Belligerent => create_player::Trait::BELLIGERENT,
            Trait::Boring => create_player::Trait::BORING,
            Trait::Cleptomaniac => create_player::Trait::CLEPTOMANIAC,
            Trait::Clumsy => create_player::Trait::CLUMSY,
            Trait::Dirty => create_player::Trait::DIRTY,
            Trait::Fat => create_player::Trait::FAT,
            Trait::Goon => create_player::Trait::GOON,
            Trait::Greedy => create_player::Trait::GREEDY,
            Trait::HotTemper => create_player::Trait::HOT_TEMPER,
            Trait::Lazy => create_player::Trait::LAZY,
            Trait::LightningArm => create_player::Trait::LIGHTNING_ARM,
            Trait::Lucky => create_player::Trait::LUCKY,
            Trait::Quick => create_player::Trait::QUICK,
            Trait::QuickWitted => create_player::Trait::QUICK_WITTED,
            Trait::SuckUp => create_player::Trait::SUCK_UP,
            Trait::SureShot => create_player::Trait::SURE_SHOT,
            Trait::Timid => create_player::Trait::TIMID,
            Trait::Tough => create_player::Trait::TOUGH,
        }
    }
}



fn fetch_name(race: &str, gender: &str) -> Result<String, Error> {
    let url = format!("http://localhost:9000/name?race={}&gender={}", race, gender);

    let response = reqwest::blocking::get(url)?
        .json::<HashMap<String, String>>()?;

    return Ok(response["name"].clone())
}

fn rand_ability_score() -> u16 {
    let mut results = Vec::<u16>::new();
    let mut rng = rand::thread_rng();

    let die = Uniform::from(1..7);

    for _ in 0..4 {
        let result = die.sample(&mut rng);
        results.push(result);
    }

    results.sort();

    return results[3] + results[2] + results[1]
}

fn set_abilities(class: Class, ability_scores: Vec<u16>) -> Abilities {
    match class {
        Class::Bard => {
            Abilities {
                strength: ability_scores[2] as i64,
                dexterity: ability_scores[4] as i64,
                constitution: ability_scores[3] as i64,
                intelligence: ability_scores[0] as i64,
                wisdom: ability_scores[1] as i64,
                charisma: ability_scores[5] as i64,
            }
        },
        Class::Cleric => {
            Abilities {
                strength: ability_scores[3] as i64,
                dexterity: ability_scores[2] as i64,
                constitution: ability_scores[4] as i64,
                intelligence: ability_scores[0] as i64,
                wisdom: ability_scores[5] as i64,
                charisma: ability_scores[1] as i64,
            }
        },
        Class::Druid => {
            Abilities {
                strength: ability_scores[2] as i64,
                dexterity: ability_scores[4] as i64,
                constitution: ability_scores[3] as i64,
                intelligence: ability_scores[1] as i64,
                wisdom: ability_scores[5] as i64,
                charisma: ability_scores[0] as i64,
            }
        },
        Class::Fighter => {
            Abilities {
                strength: ability_scores[5] as i64,
                dexterity: ability_scores[3] as i64,
                constitution: ability_scores[4] as i64,
                intelligence: ability_scores[0] as i64,
                wisdom: ability_scores[2] as i64,
                charisma: ability_scores[1] as i64,
            }
        },
        Class::Paladin => {
            Abilities {
                strength: ability_scores[5] as i64,
                dexterity: ability_scores[2] as i64,
                constitution: ability_scores[3] as i64,
                intelligence: ability_scores[0] as i64,
                wisdom: ability_scores[1] as i64,
                charisma: ability_scores[4] as i64,
            }
        },
        Class::Ranger => {
            Abilities {
                strength: ability_scores[1] as i64,
                dexterity: ability_scores[5] as i64,
                constitution: ability_scores[4] as i64,
                intelligence: ability_scores[2] as i64,
                wisdom: ability_scores[3] as i64,
                charisma: ability_scores[0] as i64,
            }
        },
        Class::Rogue => {
            Abilities {
                strength: ability_scores[0] as i64,
                dexterity: ability_scores[5] as i64,
                constitution: ability_scores[2] as i64,
                intelligence: ability_scores[4] as i64,
                wisdom: ability_scores[1] as i64,
                charisma: ability_scores[3] as i64,
            }
        },
        Class::Wizard => {
            Abilities {
                strength: ability_scores[0] as i64,
                dexterity: ability_scores[4] as i64,
                constitution: ability_scores[3] as i64,
                intelligence: ability_scores[5] as i64,
                wisdom: ability_scores[2] as i64,
                charisma: ability_scores[1] as i64,
            }
        },
    }
}

fn calc_max_health(class: Class, constitution: i64) -> i64 {
    let modifier = (constitution - 10) / 2;
    match class {
        Class::Bard => 8 + modifier,
        Class::Cleric => 8 + modifier,
        Class::Druid => 8 + modifier,
        Class::Fighter => 10 + modifier,
        Class::Paladin => 10 + modifier,
        Class::Ranger => 10 + modifier,
        Class::Rogue => 8 + modifier,
        Class::Wizard => 6 + modifier,
    }
}

const PREMIUM_MULT: i64 = 500;
const SECONDARY_MULT: i64 = 200;
const NORMAL_MULT: i64 = 100;
const DUMP_MULT: i64 = 25;

fn calc_cost(class: Class, abilities: &Abilities, trait_one: Trait, trait_two: Trait) -> i64 {
    let mut cost: i64 = 0;
    match class {
        Class::Bard => {
            cost += abilities.strength * DUMP_MULT;
            cost += abilities.dexterity * SECONDARY_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * NORMAL_MULT;
            cost += abilities.wisdom * NORMAL_MULT;
            cost += abilities.charisma * PREMIUM_MULT;
        },
        Class::Cleric => {
            cost += abilities.strength * NORMAL_MULT;
            cost += abilities.dexterity * NORMAL_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * DUMP_MULT;
            cost += abilities.wisdom * PREMIUM_MULT;
            cost += abilities.charisma * NORMAL_MULT;
        },
        Class::Druid => {
            cost += abilities.strength * DUMP_MULT;
            cost += abilities.dexterity * SECONDARY_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * NORMAL_MULT;
            cost += abilities.wisdom * PREMIUM_MULT;
            cost += abilities.charisma * DUMP_MULT;
        },
        Class::Fighter => {
            cost += abilities.strength * PREMIUM_MULT;
            cost += abilities.dexterity * DUMP_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * NORMAL_MULT;
            cost += abilities.wisdom * NORMAL_MULT;
            cost += abilities.charisma * NORMAL_MULT;
        },
        Class::Paladin => {
            cost += abilities.strength * PREMIUM_MULT;
            cost += abilities.dexterity * DUMP_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * DUMP_MULT;
            cost += abilities.wisdom * DUMP_MULT;
            cost += abilities.charisma * PREMIUM_MULT;
        },
        Class::Ranger => {
            cost += abilities.strength * DUMP_MULT;
            cost += abilities.dexterity * PREMIUM_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * NORMAL_MULT;
            cost += abilities.wisdom * SECONDARY_MULT;
            cost += abilities.charisma * DUMP_MULT;
        },
        Class::Rogue => {
            cost += abilities.strength * DUMP_MULT;
            cost += abilities.dexterity * PREMIUM_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * SECONDARY_MULT;
            cost += abilities.wisdom * NORMAL_MULT;
            cost += abilities.charisma * NORMAL_MULT;
        },
        Class::Wizard => {
            cost += abilities.strength * DUMP_MULT;
            cost += abilities.dexterity * SECONDARY_MULT;
            cost += abilities.constitution * SECONDARY_MULT;
            cost += abilities.intelligence * PREMIUM_MULT;
            cost += abilities.wisdom * SECONDARY_MULT;
            cost += abilities.charisma * DUMP_MULT;
        },
    }


    // AGENT COST
    cost += calc_trait_cost(trait_one);
    cost += calc_trait_cost(trait_two);

    let mut rng = rand::thread_rng();
    cost += Uniform::from(0..=2000).sample(&mut rng);

    cost
}

fn calc_trait_cost(t: Trait) -> i64{
    match t {
        Trait::Belligerent => -2000,
        Trait::Boring => 0,
        Trait::Cleptomaniac => 1000,
        Trait::Clumsy => -2000,
        Trait::Dirty => 1000,
        Trait::Fat => -1000,
        Trait::Goon => 0,
        Trait::Greedy => -1000,
        Trait::HotTemper => -1000,
        Trait::Lazy => -1000,
        Trait::LightningArm => 5000,
        Trait::Lucky => 5000,
        Trait::Quick => 2000,
        Trait::QuickWitted => 2000,
        Trait::SuckUp => 1000,
        Trait::SureShot => 10000,
        Trait::Timid => -500,
        Trait::Tough => 1000,
    }
}


fn gen_players_for_league(league_id: &str, count: i64) -> Result<(), Error> {
    for _ in 0..count {
        let race: Race = rand::random();
        let gender: Gender = rand::random();
        let class: Class = rand::random();

        let race_str = format!("{:?}", race).to_lowercase();
        let gender_str = format!("{:?}", gender).to_lowercase();
        let class_str = format!("{:?}", class).to_lowercase();

        let name = fetch_name(&race_str, &gender_str);

        if let Ok(name) = name {
            let output = format!("{} : {} {} {}", name, &gender_str, &race_str, &class_str);

            let mut ability_scores = Vec::<u16>::new();

            for _ in 0..6 {
                ability_scores.push(rand_ability_score());
            }

            ability_scores.sort();

            let abilities = set_abilities(class, ability_scores);

            let trait_one: Trait = rand::random();
            let trait_two: Trait = rand::random();

            let max_health = calc_max_health(class, abilities.constitution);

            let cost = calc_cost(class, &abilities, trait_one, trait_two);

            let variables = create_player::Variables{
                name: name,
                league: league_id.to_string(),
                race: race.into(),
                class: class.into(),
                gender: gender.into(),
                max_health: max_health,
                cost: cost,
                strength: abilities.strength,
                dexterity: abilities.dexterity,
                constitution: abilities.constitution,
                intelligence: abilities.intelligence,
                wisdom: abilities.wisdom,
                charisma: abilities.charisma,
                traits: vec![trait_one.into(), trait_two.into()],
            };

            /*let variables = create_player::Variables{
                name: "Chester Cheeto".to_string(),
                league: "sd2309ijdfl".to_string(),
                race: create_player::Race::DWARF,
                class: create_player::Class::WIZARD,
                gender: create_player::Gender::MALE,
                max_health: 10,
                cost: 100,
                strength: 10,
                dexterity: 10,
                wisdom: 10,
                constitution: 10,
                intelligence: 10,
                charisma: 10,
                traits: vec![create_player::Trait::LUCKY],
            };*/

            let client = Client::builder()
                .user_agent("afb/0.1.0")
                .default_headers(
                    std::iter::once((
                        reqwest::header::CONTENT_TYPE,
                        reqwest::header::HeaderValue::from_str("application/json").unwrap(),
                    ))
                    .collect()
                )
                .build()?;

            let response = post_graphql::<CreatePlayer, _>(&client, SERVER, variables)?;

            let data: create_player::ResponseData = response.data.unwrap();

            //println!("{:?}", data.create_player.id.to_string());

            //dbg!("{:?}", response);
            println!("{} : str: {} dex: {} con: {} int: {} wis: {} chr: {} : {:?} {:?} : {} {}", output,
                abilities.strength, abilities.dexterity, abilities.constitution, abilities.intelligence, abilities.wisdom, abilities.charisma,
                trait_one, trait_two, max_health, cost);
        } else {
            dbg!("{:?}", name);
        }
    }

    Ok(())
}



fn query_leagues() -> Result<leagues::ResponseData, Error> {
    let variables = leagues::Variables{};

    let client = Client::builder()
        .user_agent("afb/0.1.0")
        .default_headers(
            std::iter::once((
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str("application/json").unwrap(),
            ))
            .collect()
        )
        .build()?;

    let response = post_graphql::<Leagues, _>(&client, SERVER, variables)?;

    let data: leagues::ResponseData = response.data.expect("missing response data");

    Ok(data)
}

fn set_league_state(id: &str, state: set_league_state::LeagueState) -> Result<(), Error> {
    let variables = set_league_state::Variables{
        id: id.to_string(),
        state,
    };

    let client = Client::builder()
        .user_agent("afb/0.1.0")
        .default_headers(
            std::iter::once((
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str("application/json").unwrap(),
            ))
            .collect()
        )
        .build()?;

    let response = post_graphql::<SetLeagueState, _>(&client, SERVER, variables)?;

    let data: set_league_state::ResponseData = response.data.expect("missing response data");

    Ok(())
}


fn main() {
    let leagues_resp = query_leagues();

    match leagues_resp {
        Ok(data) => {
            for league in data.leagues.iter() {
                if league.status == leagues::LeagueStatus::PLAYER_GENERATION {
                    gen_players_for_league(&league.id, league.max_players * PER_PLAYER);
                    set_league_state(&league.id, set_league_state::                   
                }
            }
        },
        Err(err) => {
            dbg!{err};
        }
    }
}
