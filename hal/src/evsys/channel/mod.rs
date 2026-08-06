use core::marker::PhantomData;
use crate::typelevel::Sealed;

pub trait Status: Sealed {}

pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}

pub enum Initialized {}
impl Sealed for Initialized {}
impl Status for Initialized {}

pub struct Channel<Id: ChId, S: Status> {
    regs: RegisterBlock<Id>,
    _status: PhantomData<S>,
}
