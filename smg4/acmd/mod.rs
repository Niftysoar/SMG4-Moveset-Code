mod throw;
mod areals;
mod specials;
mod tilts;
mod smashes;
mod ground;
mod other;
// mod final_smash;

pub fn install() {
    throw::install();
    areals::install();
    specials::install();
    tilts::install();
    smashes::install();
    ground::install();
    other::install();
    // final_smash::install();
}