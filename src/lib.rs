use std::cell::{RefCell};
use std::rc::{Rc};

#[cfg(feature = "openmp")] extern crate openmp_runtime;

#[cfg(feature = "openmp")] pub mod openmp;

pub trait ExecutionContext {
  type Guard;

  fn implicit() -> Rc<Self> where Self: Sized;
  fn max_depth() -> Option<usize>;
  fn push(ctx: Rc<Self>) -> Self::Guard where Self: 'static + Sized;
}

pub struct ExecCtxStack<Ctx> {
  pub active:   RefCell<Option<Rc<Ctx>>>,
  //pub implicit: RefCell<Vec<Rc<Ctx>>>,
}

impl<Ctx> ExecCtxStack<Ctx> where Ctx: ExecutionContext {
  pub fn new() -> Self {
    ExecCtxStack{
      active:   RefCell::new(None),
      //implicit: ...
    }
  }

  pub fn push(&mut self, ctx: Rc<Ctx>) {
    unimplemented!();
  }

  pub fn pop(&mut self) {
    unimplemented!();
  }

  pub fn top(&self) -> Rc<Ctx> {
    unimplemented!();
  }
}
