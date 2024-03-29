#![warn(unused_extern_crates)]
#![feature(
    rustc_private,
    min_specialization,
    once_cell,
    if_let_guard,
    let_chains,
    type_alias_impl_trait,
    box_patterns,
    drain_filter,
    result_option_inspect
)]
///! Refinement type checking
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_middle;
extern crate rustc_serialize;
extern crate rustc_span;

mod checker;
mod constraint_gen;
mod fixpoint_encoding;
pub mod invariants;
mod param_infer;
mod refine_tree;
mod sigs;
mod type_env;

use checker::Checker;
use constraint_gen::{ConstrReason, Tag};
use flux_common::{cache::QueryCache, dbg};
use flux_config as config;
use flux_errors::ResultExt;
use flux_macros::fluent_messages;
use flux_middle::{global_env::GlobalEnv, rty};
use itertools::Itertools;
use rustc_errors::{DiagnosticMessage, ErrorGuaranteed, SubdiagnosticMessage};
use rustc_hir::def_id::LocalDefId;

fluent_messages! { "../locales/en-US.ftl" }

pub fn check_fn(
    genv: &GlobalEnv,
    cache: &mut QueryCache,
    local_id: LocalDefId,
) -> Result<(), ErrorGuaranteed> {
    let def_id = local_id.to_def_id();
    dbg::check_fn_span!(genv.tcx, def_id).in_scope(|| {
        if genv.map().is_trusted(local_id) {
            return Ok(());
        }
        // HACK(nilehmann) this will ignore any code generated by a macro. This is
        // a temporary workaround to allow `#[derive(PartialEq, Eq)]` and should be
        // removed.
        if genv.tcx.def_span(def_id).ctxt() > rustc_span::SyntaxContext::root() {
            return Ok(());
        }

        // PHASE 1: infer shape of basic blocks
        let shape_result = Checker::run_in_shape_mode(genv, def_id).emit(genv.sess)?;
        tracing::info!("check_fn::shape");

        // PHASE 2: generate refinement tree constraint
        let (mut refine_tree, kvars) =
            Checker::run_in_refine_mode(genv, def_id, shape_result).emit(genv.sess)?;
        tracing::info!("check_fn::refine");

        // PHASE 3: invoke fixpoint on the constraints
        refine_tree.simplify();
        if config::dump_constraint() {
            dbg::dump_item_info(genv.tcx, def_id, "fluxc", &refine_tree).unwrap();
        }
        let mut fcx = fixpoint_encoding::FixpointCtxt::new(genv, def_id, kvars);
        let constraint = refine_tree.into_fixpoint(&mut fcx);
        let errors = fcx.check(cache, constraint).emit(genv.sess)?;

        tracing::info!("check_fn::fixpoint");
        if errors.is_empty() {
            Ok(())
        } else {
            report_errors(genv, errors)
        }
    })
}

fn report_errors(genv: &GlobalEnv, errors: Vec<Tag>) -> Result<(), ErrorGuaranteed> {
    let mut e = None;
    for err in errors {
        let span = err.span;
        e = Some(match err.reason {
            ConstrReason::Call => genv.sess.emit_err(errors::CallError { span }),
            ConstrReason::Assign => genv.sess.emit_err(errors::AssignError { span }),
            ConstrReason::Ret => genv.sess.emit_err(errors::RetError { span }),
            ConstrReason::Div => genv.sess.emit_err(errors::DivError { span }),
            ConstrReason::Rem => genv.sess.emit_err(errors::RemError { span }),
            ConstrReason::Goto(_) => genv.sess.emit_err(errors::GotoError { span }),
            ConstrReason::Assert(msg) => genv.sess.emit_err(errors::AssertError { span, msg }),
            ConstrReason::Fold => genv.sess.emit_err(errors::FoldError { span }),
            ConstrReason::Overflow => genv.sess.emit_err(errors::OverflowError { span }),
            ConstrReason::Other => genv.sess.emit_err(errors::UnknownError { span }),
        });
    }

    if let Some(e) = e {
        Err(e)
    } else {
        Ok(())
    }
}

mod errors {
    use flux_macros::Diagnostic;
    use rustc_span::Span;

    #[derive(Diagnostic)]
    #[diag(refineck_goto_error, code = "FLUX")]
    pub struct GotoError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_call_error, code = "FLUX")]
    pub struct CallError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_assign_error, code = "FLUX")]
    pub struct AssignError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_ret_error, code = "FLUX")]
    pub struct RetError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_div_error, code = "FLUX")]
    pub struct DivError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_rem_error, code = "FLUX")]
    pub struct RemError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_assert_error, code = "FLUX")]
    pub struct AssertError {
        #[primary_span]
        pub span: Span,
        pub msg: &'static str,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_fold_error, code = "FLUX")]
    pub struct FoldError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_overflow_error, code = "FLUX")]
    pub struct OverflowError {
        #[primary_span]
        pub span: Span,
    }

    #[derive(Diagnostic)]
    #[diag(refineck_unknown_error, code = "FLUX")]
    pub struct UnknownError {
        #[primary_span]
        pub span: Span,
    }
}
