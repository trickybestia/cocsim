use arbitrary::Arbitrary;
use cocsim::{
    AttackPlan,
    AttackPlanExecutor,
    Game,
    Map,
};

#[derive(Arbitrary)]
struct FuzzInputs {
    pub map: Map,
    pub plan: AttackPlan,
}

fn main() {
    afl::fuzz!(|inputs: FuzzInputs| {
        let _ = fuzz(inputs);
    });
}

fn fuzz(inputs: FuzzInputs) -> anyhow::Result<()> {
    let mut game = Game::new(&inputs.map, false)?;
    let mut plan_executor = AttackPlanExecutor::new(inputs.plan.units());

    while !game.done() {
        plan_executor.tick(&mut game)?;
        game.tick(1.0 / 60.0 as f32)?;
    }

    Ok(())
}
