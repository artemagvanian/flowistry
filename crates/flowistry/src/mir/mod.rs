//! Infrastructure for analyzing MIR that supports the information flow analysis.

use polonius_engine::FactTypes;
use rustc_ast::token::TokenKind::FatArrow;
use rustc_borrowck::consumers::{BodyWithBorrowckFacts, RustcFacts};
use rustc_middle::mir::Body;

pub mod aliases;
pub mod engine;
pub mod placeinfo;
pub mod utils;

/// The per-procedure information the analysis needs. Most of the time this is
/// going to be
/// [BodyWithBorrowckFacts]
pub trait FlowistryInput<'tcx>: Copy {
  fn body(self) -> &'tcx Body<'tcx>;
  fn input_facts_subset_base(
    self,
  ) -> &'tcx [(
    <RustcFacts as FactTypes>::Origin,
    <RustcFacts as FactTypes>::Origin,
    <RustcFacts as FactTypes>::Point,
  )];
}

impl<'tcx> FlowistryInput<'tcx> for &'tcx BodyWithBorrowckFacts<'tcx> {
  fn body(self) -> &'tcx Body<'tcx> {
    &self.body
  }

  fn input_facts_subset_base(
    self,
  ) -> &'tcx [(
    <RustcFacts as FactTypes>::Origin,
    <RustcFacts as FactTypes>::Origin,
    <RustcFacts as FactTypes>::Point,
  )] {
    &self.input_facts.as_ref().unwrap().subset_base
  }
}
