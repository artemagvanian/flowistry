//! Compute program dependence graphs (PDG) for a function call graph.

pub use utils::FnResolution;

use self::graph::DepGraph;
use crate::pdg::construct::GraphConstructor;
pub use crate::pdg::construct::{
  is_async_trait_fn, CallChanges, CallInfo, FakeEffect, FakeEffectKind, PdgParams,
  SkipCall,
};

mod construct;
pub mod graph;
mod utils;

/// Computes a global program dependence graph (PDG) starting from the root function specified by `def_id`.
pub fn compute_pdg<'tcx>(params: PdgParams<'tcx>) -> DepGraph<'tcx> {
  let constructor = GraphConstructor::root(params);
  constructor.construct()
}
