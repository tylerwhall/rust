// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(non_camel_case_types, non_snake_case)]

use rustc::ty::{self, Ty, TyCtxt};
use syntax_pos::DUMMY_SP;


pub fn type_needs_drop<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>, ty: Ty<'tcx>) -> bool {
    ty.needs_drop(tcx, ty::ParamEnv::reveal_all())
}

pub fn type_is_sized<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_sized(tcx.at(DUMMY_SP), ty::ParamEnv::reveal_all())
}

pub fn type_is_freeze<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_freeze(tcx, ty::ParamEnv::reveal_all(), DUMMY_SP)
}

pub enum IntPredicate {
    IntEQ,
    IntNE,
    IntUGT,
    IntUGE,
    IntULT,
    IntULE,
    IntSGT,
    IntSGE,
    IntSLT,
    IntSLE
}


#[allow(dead_code)]
pub enum RealPredicate {
    RealPredicateFalse,
    RealOEQ,
    RealOGT,
    RealOGE,
    RealOLT,
    RealOLE,
    RealONE,
    RealORD,
    RealUNO,
    RealUEQ,
    RealUGT,
    RealUGE,
    RealULT,
    RealULE,
    RealUNE,
    RealPredicateTrue
}

pub enum AtomicRmwBinOp {
    AtomicXchg,
    AtomicAdd,
    AtomicSub,
    AtomicAnd,
    AtomicNand,
    AtomicOr,
    AtomicXor,
    AtomicMax,
    AtomicMin,
    AtomicUMax,
    AtomicUMin
}

pub enum AtomicOrdering {
    #[allow(dead_code)]
    NotAtomic,
    Unordered,
    Monotonic,
    // Consume,  // Not specified yet.
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

pub enum SynchronizationScope {
    // FIXME: figure out if this variant is needed at all.
    #[allow(dead_code)]
    Other,
    SingleThread,
    CrossThread,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TypeKind {
    Void,
    Half,
    Float,
    Double,
    X86_FP80,
    FP128,
    PPC_FP128,
    Label,
    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
    Metadata,
    X86_MMX,
    Token,
}

// FIXME(mw): Anything that is produced via DepGraph::with_task() must implement
//            the HashStable trait. Normally DepGraph::with_task() calls are
//            hidden behind queries, but CGU creation is a special case in two
//            ways: (1) it's not a query and (2) CGU are output nodes, so their
//            Fingerprints are not actually needed. It remains to be clarified
//            how exactly this case will be handled in the red/green system but
//            for now we content ourselves with providing a no-op HashStable
//            implementation for CGUs.
mod temp_stable_hash_impls {
    use rustc_data_structures::stable_hasher::{StableHasherResult, StableHasher,
                                               HashStable};
    use ModuleCodegen;

    impl<HCX, M> HashStable<HCX> for ModuleCodegen<M> {
        fn hash_stable<W: StableHasherResult>(&self,
                                              _: &mut HCX,
                                              _: &mut StableHasher<W>) {
            // do nothing
        }
    }
}
