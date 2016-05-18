use std::cell::Cell;
use std::cell::RefCell;


pub enum Promise<T: ?Sized, E: ?Sized> {
    Finished(T),
    Failed(E),
    Working { whenFinished: Box<Fn(Result<T, E>) -> ()> }
}
pub struct Future<T: ?Sized, E: ?Sized> {
    cell: RefCell<Promise<T, E>>
}

pub struct FutureError;

impl<T: Copy, E: Copy> Future<T, E> {
    fn new(promise: Promise<T, E>) -> Future<T, E> {
        Future { cell: RefCell::new(promise) }
    }

    pub fn blank() -> Future<T, E> {
        let doNothing = |result: Result<T, E>| -> (){};
        Future::new(Promise::Working { whenFinished: Future::doNothing })
    }

    pub fn finish(t: T) -> Future<T, E> {
        Future::new(Promise::Finished(t))
    }
    pub fn fail(e: E) -> Future<T, E> {
        Future::new(Promise::Failed(e))
    }

    pub fn finished(&self, t: T) -> () {
        self.cell.set(Promise::Finished::<T, E>(t))
    }

    pub fn failed(&self, e: E) -> () {
        self.cell.set(Promise::Failed::<T, E>(e))
    }
    pub fn unwrap(&self) -> Result<T, E> {
        match self.cell.get() {
            Promise::Finished(t) => Ok(t),
            Promise::Failed(e) => Err(e),
            Promise::Working{whenFinished: _} => panic!("unwrap called on future which is still working")
        }
    }

    pub fn map<T1: Copy>(&self, f: fn(T) -> T1) -> Future<T1, E> {
        match self.cell.get() {
            Promise::Finished(t) => Future::new(Promise::Finished::<T1, E>(f(t))),
            Promise::Failed(e) => Future::new(Promise::Failed::<T1, E>(e)),
            Promise::Working{whenFinished: wf} => {
                let newFuture = Future::<T1, E>::blank();

                let updateFutureWithResult = |r: Result<T, E>| -> () {
                    wf(r);
                    match r {
                        Ok(t) => newFuture.finished(f(t)),
                        Err(e) => newFuture.failed(e)
                    }
                };
                let b = Box::new(updateFutureWithResult);
                self.cell.borrow_mut() = Promise::Working::<T, E> { whenFinished: updateFutureWithResult }
            }
        }
    }
}
