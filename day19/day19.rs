use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct BluePrint {
    ore: i32,
    clay: i32,
    obs: (i32, i32),
    geo: (i32, i32),
}

struct State {
    n_ore: i32,
    n_clay: i32,
    n_obs: i32,
    n_geo: i32,
    ore_r: i32,
    clay_r: i32,
    obs_r: i32,
    geo_r: i32,
}

fn mine(state: &mut State, bp: &BluePrint, minutes: i32) -> i32 {
    if minutes == 0 {
        return state.geo_r;
    }
    let mut max_ego = 0;
    state.n_ore += state.ore_r;
    state.n_clay += state.clay_r;
    state.n_obs += state.obs_r;
    state.n_geo += state.geo_r;

    if state.n_ore >= bp.ore {
        let geo = mine(
            &mut State {
                n_ore: state.n_ore - bp.ore,
                n_clay: state.n_clay,
                n_obs: state.n_obs,
                n_geo: state.n_geo,
                ore_r: state.ore_r + 1,
                clay_r: state.clay_r,
                obs_r: state.obs_r,
                geo_r: state.geo_r,
            },
            bp,
            minutes - 1,
        );
        if geo > max_ego {
            max_ego = geo;
        }
    }
    if state.n_ore >= bp.clay {
        let geo = mine(
            &mut State {
                n_ore: state.n_ore - bp.clay,
                n_clay: state.n_clay,
                n_obs: state.n_obs,
                n_geo: state.n_geo,
                ore_r: state.ore_r,
                clay_r: state.clay_r + 1,
                obs_r: state.obs_r,
                geo_r: state.geo_r,
            },
            bp,
            minutes - 1,
        );
        if geo > max_ego {
            max_ego = geo;
        }
    }
    if state.n_ore >= bp.obs.0 && state.n_clay >= bp.obs.1 {
        let geo = mine(
            &mut State {
                n_ore: state.n_ore - bp.obs.0,
                n_clay: state.n_clay - bp.obs.1,
                n_obs: state.n_obs,
                n_geo: state.n_geo,
                ore_r: state.ore_r,
                clay_r: state.clay_r,
                obs_r: state.obs_r + 1,
                geo_r: state.geo_r,
            },
            bp,
            minutes - 1,
        );
        if geo > max_ego {
            max_ego = geo;
        }
    }

    if state.n_ore >= bp.geo.0 && state.n_obs >= bp.geo.1 {
        let geo = mine(
            &mut State {
                n_ore: state.n_ore - bp.obs.0,
                n_clay: state.n_clay,
                n_obs: state.n_obs - bp.geo.1,
                n_geo: state.n_geo + 1,
                ore_r: state.ore_r,
                clay_r: state.clay_r,
                obs_r: state.obs_r,
                geo_r: state.geo_r,
            },
            bp,
            minutes - 1,
        );
        if geo > max_ego {
            max_ego = geo;
        }
    }
    let geo = mine(state, bp, minutes - 1);
    if geo > max_ego {
        max_ego = geo;
    }

    return max_ego;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut bps: Vec<BluePrint> = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let words = row.split(" ").collect::<Vec<&str>>();
        bps.push(BluePrint {
            ore: words[6].parse().unwrap(),
            clay: words[12].parse().unwrap(),
            obs: (words[18].parse().unwrap(), words[21].parse().unwrap()),
            geo: (words[27].parse().unwrap(), words[30].parse().unwrap()),
        });
    }

    let mut ans = 0;
    for i in 0..bps.len() {
        let mut state = State {
            n_ore: 0,
            n_clay: 0,
            n_obs: 0,
            n_geo: 0,
            ore_r: 1,
            clay_r: 0,
            obs_r: 0,
            geo_r: 0,
        };
        ans += (i + 1) * mine(&mut state, &bps[i], 24) as usize;
    }

    println!("{}", ans);
    Ok(())
}
