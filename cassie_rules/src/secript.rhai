// Implementation of the Event Handler With State Pattern - Map Style

/// Initialize user-provided state.
/// State is stored inside an object map bound to 'state'.
fn init() {
    // Add 'bool_state' as new state variable if one does not exist
    if !("bool_state" in state) {
        state.bool_state = false;
    }
    // Add 'obj_state' as new state variable (overwrites any existing)
    state.value = 0;

    // Can also add OOP-style functions!
    state.log = |x| print(`State = ${this.value}, data = ${x}`);
}

/// 'start' event handler
fn start(data) {
    // Can detect system-provided default states!
    // Access state variables in 'state'
    if state.bool_state {
        throw "Already started!";
    }

    // New values can be added to the state
    state.start_mode = data;

    if state.value <= 0 {
        throw "Conditions not yet ready to start!";
    }
    state.bool_state = true;
    state.value = parse_int(data);

    // Constant 'MY_CONSTANT' in custom scope is also visible!
    print(`MY_CONSTANT = ${MY_CONSTANT}`);
}

/// 'end' event handler
fn end(data) {
    if !state.bool_state || !("start_mode" in state) {
        throw "Not yet started!";
    }
    if state.value > 0 {
        throw "Conditions not yet ready to end!";
    }
    state.bool_state = false;
    state.value = parse_int(data);
}

/// 'update' event handler
fn update(data) {
    let data = parse_int(data);
    state.value += data;

    // Call user-defined function OOP-style!
    state.log(data);
}
