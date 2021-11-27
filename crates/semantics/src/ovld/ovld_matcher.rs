use super::*;

pub struct OvldMatcher {
  ovlds: Vec<Ovld>,
}
impl OvldMatcher {
  pub fn match_ovlds(a: &Ovld, b: &Ovld) -> bool {
    if a.inputs.len() == b.inputs.len() {
      a.inputs.iter().zip(b.inputs.iter()).all(|(r, s)| r == s)
    } else {
      false
    }
  }
  pub fn new() -> OvldMatcher {
    OvldMatcher { ovlds: vec![] }
  }
  pub fn add_ovld(&mut self, ovld: Ovld) {
    for old in &self.ovlds {
      assert!(!Self::match_ovlds(old, &ovld));
    }
    self.ovlds.push(ovld);
  }
}

pub type OvldMatchers = HashMap<*const Entity, OvldMatcher>;
