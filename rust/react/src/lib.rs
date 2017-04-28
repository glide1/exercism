#[allow(unused_variables)]

use std::collections::BTreeMap;

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

pub struct Reactor<'a, T: 'a> {
    current_id: CellID,
    cells: BTreeMap<CellID, Cell<'a, T>>,
    current_callback_id: CallbackID,
}

struct Cell<'a, T> {
    cell_data: CellData<'a, T>,
    callbacks: BTreeMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
    dependents: Vec<CellID>,
}

enum CellData<'a, T> {
    Value(T),
    Computation(Computation<'a, T>),
}

struct Computation<'a, T> {
    dependencies: Vec<CellID>,
    computation: Box<Fn(&[T]) -> T + 'a>,
    current_value: Option<T>,
}


impl<'a, T> Computation<'a, T> {
    fn new<F: Fn(&[T]) -> T + 'a>(dependencies: &[CellID], computation: F) -> Self {
        let cmp = Computation {
            computation: Box::new(computation),
            dependencies: Vec::from(dependencies),
            current_value: None,
        };

        cmp
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            current_id: 0,
            current_callback_id: 0,
            cells: BTreeMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        let id = self.current_id;
        self.cells
            .insert(self.current_id, Cell::new_input(initial));
        self.current_id += 1;
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self,
                                                      dependencies: &[CellID],
                                                      compute_func: F)
                                                      -> Result<CellID, ()> {
        if dependencies
               .into_iter()
               .any(|id| !self.cells.contains_key(id)) {
            return Err(());
        }
        let id = self.current_id;
        self.current_id += 1;

        let mut new_cell = Cell::new_computation(compute_func, dependencies);
        new_cell.update_current_value(&self.cells);
        self.cells.insert(id, new_cell);

        for dep in dependencies {
            if let Some(parent_cell) = self.cells.get_mut(dep) {
                parent_cell.add_dependent(id);
            }
        }
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match self.cells.get(&id) {
            Some(ref cell) => Some(cell.value(&self.cells)),
            _ => None,
        }
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        let res = if let Some(c) = self.cells.get_mut(&id) {
            c.set_value(new_value)
        } else {
            Err(())
        };
        if res.is_ok() {
            Reactor::call_dependent_callbacks(&mut self.cells, id)
        }
        res
    }

    fn call_dependent_callbacks(mut cell_map: &mut BTreeMap<CellID, Cell<T>>, id: CellID) {
        let mut ids = vec![];
        let mut val: Option<T> = None;
        if let Some(cell) = cell_map.get(&id) {
            val = Some(cell.value(cell_map));
        }
        if let Some(cell) = cell_map.get_mut(&id) {
            if !cell.changed_value(val.unwrap()) {
                return;
            }
            cell.trigger_callbacks(val.unwrap());
            ids = cell.dependents.clone();
        } else {
            return;
        }
        for id in ids {
            Reactor::call_dependent_callbacks(&mut cell_map, id)
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self,
                                                id: CellID,
                                                callback: F)
                                                -> Result<CallbackID, ()> {
        if !self.cells.contains_key(&id) {
            return Err(());
        }
        let cbid = self.current_callback_id;
        self.current_callback_id += 1;

        if let Some(cell) = self.cells.get_mut(&id) {
            cell.add_callback(cbid, callback);
        }

        Ok((cbid))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        if let Some(cell) = self.cells.get_mut(&cell) {
            cell.remove_callback(callback)
        } else {
            Err(())
        }
    }
}

impl<'a, T: Copy + PartialEq> Cell<'a, T> {
    fn new_input(initial: T) -> Self {
        Cell {
            callbacks: BTreeMap::new(),
            cell_data: CellData::Value(initial),
            dependents: Vec::new(),
        }
    }

    fn new_computation<F: Fn(&[T]) -> T + 'static>(computation: F,
                                                   dependencies: &[CellID])
                                                   -> Self {
        Cell {
            callbacks: BTreeMap::new(),
            cell_data: CellData::Computation(Computation::new(dependencies, computation)),
            dependents: Vec::new(),
        }
    }

    fn update_current_value(&mut self, cell_map: &BTreeMap<CellID, Cell<T>>) {
        let value = self.value(cell_map);
        if let CellData::Computation(ref mut compute) = self.cell_data {
            compute.current_value = Some(value);
        }
    }

    fn value(&self, cell_map: &BTreeMap<CellID, Cell<T>>) -> T {
        match self.cell_data {
            CellData::Value(value) => value,
            CellData::Computation(ref compute) => {
                let args: Vec<T> = compute
                    .dependencies
                    .iter()
                    .map(|d| cell_map[d].value(cell_map))
                    .collect();
                (compute.computation)(&args)
            }
        }
    }

    fn add_dependent(&mut self, id: CellID) {
        self.dependents.push(id)
    }

    fn trigger_callbacks(&mut self, value: T) {

        for cb in self.callbacks.values_mut() {
            cb(value)
        }
    }

    fn set_value(&mut self, new_value: T) -> Result<(), ()> {
        if let CellData::Value(_) = self.cell_data {
            self.cell_data = CellData::Value(new_value);
            self.trigger_callbacks(new_value);
        } else {
            return Err(());
        }
        Ok(())
    }

    fn add_callback<F: FnMut(T) -> () + 'a>(&mut self, callback_id: CallbackID, callback: F) {
        self.callbacks.insert(callback_id, Box::new(callback));
    }

    fn changed_value(&mut self, new_value: T) -> bool {
        if let CellData::Computation(ref mut compute) = self.cell_data {
            if let Some(current_val) = compute.current_value {
                if current_val != new_value {
                    compute.current_value = Some(new_value);
                    return true;
                }
            }
        } else if let CellData::Value(_) = self.cell_data {
            return true;
        }
        false
    }

    fn remove_callback(&mut self, callback: CallbackID) -> Result<(), ()> {
        if let Some(_) = self.callbacks.remove(&callback) {
            Ok(())
        } else {
            Err(())
        }
    }
}

