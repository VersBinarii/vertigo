use crate::struct_mut::VecMut;

pub struct Hooks {
    callback_before_transaction: VecMut<Box<dyn Fn() + 'static>>,
    callback_after_transaction: VecMut<Box<dyn Fn() + 'static>>,
}

impl Hooks {
    pub fn new() -> Hooks {
        Hooks {
            callback_before_transaction: VecMut::new(),
            callback_after_transaction: VecMut::new(),
        }
    }

    pub fn on_before_transaction(&self, callback: impl Fn() + 'static) {
        self.callback_before_transaction.push(Box::new(callback));
    }

    pub fn on_after_transaction(&self, callback: impl Fn() + 'static) {
        self.callback_after_transaction.push(Box::new(callback));
    }

    pub fn fire_start(&self) {
        self.callback_before_transaction.for_each(|callback| {
            (callback)();
        });
    }

    pub fn fire_end(&self) {
        self.callback_after_transaction.for_each(|callback| {
            (callback)();
        });
    }
}
