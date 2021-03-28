use rand::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::*;
use num::*;
use num_derive::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

const STAT_N: usize = 18;
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
enum STAT {
        HP
    ,   HP_FLAT
    ,   ATK
    ,   ATK_FLAT
    ,   CR
    ,   CD
    ,   DEF
    ,   DEF_FLAT
    ,   ER
    ,   EM
    ,   PYRO
    ,   ELECTRO
    ,   CRYO
    ,   HYDRO
    ,   ANEMO
    ,   GEO
    ,   PHYS
    ,   HEAL

    ,   END
}

const TYPE_N: usize = 5;
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
enum TYPE {
        FLOWER
    ,   PLUME
    ,   SANDS
    ,   GOBLET
    ,   CIRCLET
}

#[derive(Clone, Debug, Default)]
struct WeightContainer {
        weight:     [f64; STAT_N]
    ,   tot_weight: f64
}

#[derive(Clone, Debug)]
struct ArtifactBase {
        tag:            TYPE
    ,   primary_wc:     WeightContainer
    ,   secondary_wc:   WeightContainer
}

#[derive(Clone, Debug)]
struct ArtifactBases {
        flower: ArtifactBase
    ,   plume:  ArtifactBase
    ,   sands:  ArtifactBase
    ,   goblet: ArtifactBase
    ,   circlet:ArtifactBase
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ArtifactJSON {
    tag:                TYPE
    ,   primary_weights:    Vec<(STAT, f64)>
    ,   secondary_weights:  Vec<(STAT, f64)>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JSON {
        artifacts:  Vec<ArtifactJSON>
}

impl WeightContainer {
    fn set(&mut self, stat: STAT, weight: f64) {
        self.tot_weight += weight - self.weight[stat as usize];
        self.weight[stat as usize] = weight;
    }

    fn remove(&mut self, stat: STAT) {
        self.set(stat, 0.0);
    }

    fn get_weight(&self, stat: STAT) -> f64 {
        self.weight[stat as usize]
    }

    fn get_tot_weight(&self) -> f64 {
        self.tot_weight
    }

    fn choose(&self, f: f64) -> STAT {
        if f > 1.0 { return STAT::END }
        else {
            let f_adj = f * self.tot_weight;
            let mut cumsum = 0.0;
            STAT::from_usize(
                self.weight.iter()
                    .enumerate()
                    .take_while (|(i, x)| {
                        cumsum += **x;
                        (cumsum - **x) < f_adj
                    }).last()
                    .unwrap_or_else(|| (STAT::END as usize, &0.0))
                    .0).unwrap()
        }
    }

    fn choose_rand(&self) -> STAT {
        let mut rng = rand::thread_rng();
        self.choose(rng.gen())
    }
}

impl ArtifactBase {
    fn instance(&self) -> Self {
        self.clone()
    }
}

impl Default for ArtifactBases {
    fn default() -> Self {
        Self {
            flower: ArtifactBase {
                tag: TYPE::FLOWER,
                primary_wc: Default::default(),
                secondary_wc: Default::default() },
            plume: ArtifactBase {
                tag: TYPE::PLUME,
                primary_wc: Default::default(),
                secondary_wc: Default::default()
            },
            sands: ArtifactBase {
                tag: TYPE::SANDS,
                primary_wc: Default::default(),
                secondary_wc: Default::default()
            },
            goblet: ArtifactBase {
                tag: TYPE::GOBLET,
                primary_wc: Default::default(),
                secondary_wc: Default::default()
            },
            circlet: ArtifactBase {
                tag: TYPE::CIRCLET,
                primary_wc: Default::default(),
                secondary_wc: Default::default()
            }
        }
    }
}

fn import_json(path: &str) -> std::result::Result<ArtifactBases, Box<std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json: JSON = serde_json::from_reader(reader)?;
    let mut bases: ArtifactBases = Default::default();
    for artifact in json.artifacts {
        let base_artifact_ref;
        match artifact.tag {
            TYPE::PLUME =>  base_artifact_ref = &mut bases.plume,
            TYPE::FLOWER => base_artifact_ref = &mut bases.flower,
            TYPE::SANDS =>  base_artifact_ref = &mut bases.sands,
            TYPE::GOBLET => base_artifact_ref = &mut bases.goblet,
            TYPE::CIRCLET => base_artifact_ref = &mut bases.circlet,
        }
        for (stat, p) in artifact.primary_weights {
            base_artifact_ref.primary_wc.set(stat, p);
        }
        for (stat, p) in artifact.secondary_weights {
            base_artifact_ref.secondary_wc.set(stat, p);
        }
    }
    Ok(bases)
}

fn main() {
    let bases = import_json("json/artifacts.json").unwrap();
    for i in 0..100
    {
        assert_eq!(STAT::HP, bases.flower.primary_wc.choose_rand());
    }
    println!("Passed primary assertion!");

    let mut stats = vec![0; 18];
    for i in 0..1000000
    {
        stats[bases.flower.secondary_wc.choose_rand() as usize] += 1;
    }
    println!("{:?}", stats.iter().enumerate().map(|(i, x)| (STAT::from_usize(i).unwrap(), *x)).collect::<Vec<(STAT, usize)>>());
}