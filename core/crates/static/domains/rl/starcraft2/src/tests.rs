use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct WorkerRush;
impl Player for WorkerRush {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Protoss)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        for worker in &self.units.my.workers {
            worker.attack(Target::Pos(self.enemy_start), false);
        }
        Ok(())
    }
}

#[test]
fn test() -> SC2Result<()> {
    let mut bot = WorkerRush::default();
    run_vs_computer(
        &mut bot,
        Computer::new(Race::Random, Difficulty::Medium, None),
        "EternalEmpireLE",
        Default::default(),
    )
}
