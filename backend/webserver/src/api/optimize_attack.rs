use anyhow::Context;
use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{
            Message,
            WebSocket,
        },
    },
    response::Response,
};
use cocsim::{
    AttackOptimizer,
    AttackPlanExecutor,
    DerivativeAttackOptimizer,
    Game,
    GeneticAttackOptimizer,
    Map,
    Units,
    consts::MAX_ARMY_HOUSING_SPACE,
};
use log::warn;
use serde_json::json;
use tokio::task::spawn_blocking;

use crate::{
    consts::{
        FPS,
        OPTIMIZE_ATTACK_ITERATIONS,
    },
    dto_game_renderer::DtoGameRenderer,
    utils::round_floats,
};

pub async fn optimize_attack(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(async |socket| {
        if let Err(err) = optimize_attack_internal(socket).await {
            warn!("optimize_attack_internal finished with error: {err:#?}");
        }
    })
}

async fn optimize_attack_internal(mut socket: WebSocket) -> anyhow::Result<()> {
    let map_message = socket.recv().await.context("Expected message")??;
    let map: Map = serde_json::from_str(map_message.to_text()?)?;

    map.validate()?;

    let units_message = socket.recv().await.context("Expected message")??;
    let units = serde_json::from_str::<Units<MAX_ARMY_HOUSING_SPACE>>(units_message.to_text()?)?;

    socket
        .send(Message::text(
            json!({
                "type": "progress",
                "progress": "Attack optimization process started...",
            })
            .to_string(),
        ))
        .await?;

    let mut optimizer: Box<dyn AttackOptimizer> = Box::new(GeneticAttackOptimizer::new(
        map.clone(),
        (*units).into(),
        0.02,
        0.05,
    ));

    for i in 0..OPTIMIZE_ATTACK_ITERATIONS {
        if i == OPTIMIZE_ATTACK_ITERATIONS / 2 {
            optimizer = Box::new(DerivativeAttackOptimizer::new(
                map.clone(),
                (*units).into(),
                optimizer.best().cloned(),
            ));
        }

        optimizer = spawn_blocking(move || {
            optimizer.step();

            optimizer
        })
        .await?;

        let (_, best_plan_stats) = optimizer.best().expect("Best plan exists here");

        let progress = format!(
            "Gen. #{i} best plan finished in {:.1} <= {:.1} <= {:.1} seconds",
            best_plan_stats.min_time_elapsed(),
            best_plan_stats.avg_time_elapsed,
            best_plan_stats.max_time_elapsed()
        );

        socket
            .send(Message::text(
                json!({
                    "type": "progress",
                    "progress": progress,
                })
                .to_string(),
            ))
            .await?;
    }

    socket
        .send(Message::text(
            json!({
                "type": "progress",
                "progress": "Attack optimization done, rendering result...",
            })
            .to_string(),
        ))
        .await?;

    let result = spawn_blocking(move || {
        let mut game = Game::new(&map, true, None);
        let mut plan_executor =
            AttackPlanExecutor::new(&optimizer.best().expect("Best plan exists here").0.units);

        let mut renderer = DtoGameRenderer::new(1);

        plan_executor.tick(&mut game);
        renderer.draw(&mut game);

        while !game.done() && (!plan_executor.is_empty() || game.is_attacker_team_present()) {
            plan_executor.tick(&mut game); // no problem calling it twice on first loop iteration
            game.tick(1.0 / FPS as f32);
            renderer.draw(&mut game);
        }

        let mut result: serde_json::Value =
            serde_json::to_value(renderer.finish(&mut game)).expect("Should not fail");

        round_floats(&mut result, 2);

        result
    })
    .await?;

    socket
        .send(Message::text(
            json!({
                "type": "result",
                "result": result,
            })
            .to_string(),
        ))
        .await?;

    socket.send(Message::Close(None)).await?;

    Ok(())
}
