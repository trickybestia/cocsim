use cocsim::{
    Game,
    Map,
    ValidatedMap,
    WithCount,
    WithMaxHousingSpace,
    attack_optimizer::{
        Army,
        AttackPlanExecutor,
        v1::{
            AttackOptimizer,
            RandomAttackOptimizer,
            SimulatedAnnealingAttackOptimizer,
        },
    },
    consts::{
        MAX_ARMY_HOUSING_SPACE,
        MAX_SPELLS_HOUSING_SPACE,
    },
    spells::SpellModelEnum,
    units::UnitModelEnum,
};
use serde_json::{
    json,
    to_value,
};
use thiserror::Error;

use crate::{
    consts::{
        FPS,
        OPTIMIZE_ATTACK_ITERATIONS,
        OPTIMIZE_ATTACK_ITERATIONS_PER_STEP,
        OPTIMIZE_ATTACK_STEPS,
    },
    dto_game_renderer::DtoGameRenderer,
};

#[derive(Error, Debug)]
pub enum SendRecvError {
    #[error("request was cancelled")]
    Cancel,
}

pub fn optimize_attack(
    mut send: impl FnMut(String) -> Result<(), SendRecvError>,
    mut recv: impl FnMut() -> Result<String, SendRecvError>,
) -> anyhow::Result<()> {
    let map: Map = serde_json::from_str(&recv()?)?;
    let map = ValidatedMap::try_from(map)?;

    let units = serde_json::from_str::<
        WithMaxHousingSpace<MAX_ARMY_HOUSING_SPACE, WithCount<UnitModelEnum>>,
    >(&recv()?)?;
    let spells = serde_json::from_str::<
        WithMaxHousingSpace<MAX_SPELLS_HOUSING_SPACE, WithCount<SpellModelEnum>>,
    >(&recv()?)?;

    let army = Army {
        units: units.to_vec(),
        spells: spells.to_vec(),
    };

    send(
        json!({
            "type": "progress",
            "progress": "Attack optimization process started...",
        })
        .to_string(),
    )?;

    let mut optimizer = RandomAttackOptimizer::new(map.clone(), army.clone(), 100);

    for i in 0..10 {
        optimizer.step();

        let (_, best_plan_stats) = optimizer.best().expect("Best plan exists here");

        let progress = format!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed
        );

        send(
            json!({
                "type": "progress",
                "progress": progress,
            })
            .to_string(),
        )?;
    }

    let mut optimizer = SimulatedAnnealingAttackOptimizer::new(
        map.clone(),
        army.clone(),
        optimizer.best().cloned(),
        OPTIMIZE_ATTACK_ITERATIONS,
        OPTIMIZE_ATTACK_ITERATIONS_PER_STEP,
    );

    for i in 0..OPTIMIZE_ATTACK_STEPS {
        optimizer.step();

        let (_, best_plan_stats) = optimizer.best().expect("Best plan exists here");

        let progress = format!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds",
            best_plan_stats.min_time_elapsed,
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed
        );

        send(
            json!({
                "type": "progress",
                "progress": progress,
            })
            .to_string(),
        )?;
    }

    send(
        json!({
            "type": "progress",
            "progress": "Attack optimization done, rendering result...",
        })
        .to_string(),
    )?;

    let mut game = Game::new(&map, true, None);
    let mut plan_executor = AttackPlanExecutor::new(
        optimizer
            .best()
            .expect("Best plan exists here")
            .0
            .executor_actions(&map),
    );

    let mut renderer = DtoGameRenderer::new(1);

    plan_executor.tick(&mut game);
    renderer.draw(&mut game);

    while !game.done() && (!plan_executor.is_empty() || game.is_attacker_team_present()) {
        plan_executor.tick(&mut game); // no problem calling it twice on first loop iteration
        game.tick(1.0 / FPS as f32);
        renderer.draw(&mut game);
    }

    let result = to_value(renderer.finish(&mut game)).expect("Should not fail");

    send(
        json!({
            "type": "result",
            "result": result,
        })
        .to_string(),
    )?;

    Ok(())
}
