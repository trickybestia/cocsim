use arbitrary::Arbitrary;
use cocsim::{
    AttackPlan,
    AttackPlanExecutor,
    Game,
    Map,
    validate_units,
};

#[derive(Arbitrary, Debug)]
struct FuzzInputs {
    pub map: Map,
    pub plan: AttackPlan,
}

fn main() {
    afl::fuzz!(|inputs: FuzzInputs| {
        if inputs.map.validate().is_ok()
            && validate_units(inputs.plan.units().iter().map(|unit| unit.unit_model())).is_ok()
        {
            // uncomment next line when debugging crash
            //println!("{:#?}", inputs);

            let mut game = Game::new(&inputs.map, false);
            let mut plan_executor = AttackPlanExecutor::new(inputs.plan.units());

            while !game.done() {
                plan_executor.tick(&mut game);
                game.tick(1.0 / 60.0 as f32);
            }
        }
    });
}
