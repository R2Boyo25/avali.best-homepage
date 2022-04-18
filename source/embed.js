function updateURL(location) {
    console.log(location);
    history.pushState({state:1}, "State 1", location);
}