#![no_main]

use arbitrary::Arbitrary;
use cocsim::{
    AttackPlan,
    AttackPlanExecutor,
    Game,
    Map,
    UnitModel,
    ValidatedMap,
    consts::MAX_ARMY_HOUSING_SPACE,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzInputs {
    pub map: Map,
    pub plan: AttackPlan,
}

fuzz_target!(|inputs: FuzzInputs| {
    if inputs
        .plan
        .units
        .iter()
        .map(|unit| unit.unit_model.r#type().housing_space)
        .sum::<usize>()
        <= MAX_ARMY_HOUSING_SPACE
        && let Ok(map) = ValidatedMap::try_from(inputs.map)
    {
        // uncomment next line when debugging crash
        //dbg!(inputs);

        let mut game = Game::new(&map, false, None);
        let mut plan_executor = AttackPlanExecutor::new(&inputs.plan, &map);

        while !game.done() && (game.is_attacker_team_present() || !plan_executor.is_empty()) {
            plan_executor.tick(&mut game);
            game.tick(1.0 / 60.0 as f32);
        }

        //println!("done!")
    }
});
