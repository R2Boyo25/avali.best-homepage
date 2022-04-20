function updateURL(location) {
    history.pushState({
        state: 1
    }, "State 1", location);
}