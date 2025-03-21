use flowistry::{
  infoflow::mutation::{ModularMutationVisitor, Mutation},
  mir::placeinfo::PlaceInfo,
};
use indexical::impls::RustcIndexMatrix;
use rustc_middle::mir::{visit::Visitor, Body, Mutability, Place};
use rustc_utils::mir::location_or_arg::LocationOrArg;

pub struct DirectInfluence<'a, 'tcx> {
  place_info: &'a PlaceInfo<'tcx>,
  influence: RustcIndexMatrix<Place<'tcx>, LocationOrArg>,
}

impl<'a, 'tcx> DirectInfluence<'a, 'tcx> {
  pub fn build(body: &Body<'tcx>, place_info: &'a PlaceInfo<'tcx>) -> Self {
    let mut influence = RustcIndexMatrix::new(place_info.location_domain());

    ModularMutationVisitor::new(place_info, |location, mutations| {
      let mut add = |place: Place<'tcx>, mutability: Mutability| {
        for alias in place_info.reachable_values(place, mutability) {
          influence.insert(*alias, location);
        }
      };

      for Mutation {
        mutated, inputs, ..
      } in mutations
      {
        for input in inputs {
          add(input, Mutability::Not);
        }

        add(mutated, Mutability::Mut);
      }
    })
    .visit_body(body);

    DirectInfluence {
      place_info,
      influence,
    }
  }

  pub fn lookup(&self, target: Place<'tcx>) -> Vec<LocationOrArg> {
    let aliases = self.place_info.reachable_values(target, Mutability::Not);
    aliases
      .iter()
      .flat_map(|target_alias| {
        self
          .influence
          .row_set(target_alias)
          .iter()
          .copied()
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>()
  }
}
