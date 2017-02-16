use super::*;

use openmp_runtime::*;

thread_local!(static OMP_EXEC_CTX_STACK: Rc<ExecCtxStack<OmpExecCtx>> = Rc::new(ExecCtxStack::new()));

pub struct OmpExecCtxGuard;

impl Drop for OmpExecCtxGuard {
  fn drop(&mut self) {
  }
}

#[derive(Default)]
pub struct OmpExecCtx {
  pub max_team_size:    Option<usize>,
}

impl ExecutionContext for OmpExecCtx {
  type Guard = OmpExecCtxGuard;

  fn implicit() -> Rc<OmpExecCtx> {
    OMP_EXEC_CTX_STACK.with(|stack| {
      let stack = stack.borrow();
      stack.top()
    })
  }

  fn max_depth() -> Option<usize> {
    Some(1)
  }

  fn push(ctx: Rc<OmpExecCtx>) -> OmpExecCtxGuard {
    if let Some(max_team_size) = ctx.max_team_size {
      assert!(max_team_size >= 1);
      unsafe { omp_set_num_threads(max_team_size as _) };
    }
    OMP_EXEC_CTX_STACK.with(|stack| {
      let mut stack = stack.borrow_mut();
      stack.push(ctx);
    });
  }
}
