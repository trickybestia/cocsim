use rand_pcg::Pcg64Mcg;
use shipyard::Unique;

#[derive(Unique)]
pub struct RngUnique(pub Pcg64Mcg);
