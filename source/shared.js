function getGetParameters() {
    var result = [],
        tmp = [];
    var items = location.search.substr(1).split("&");
    for (var index = 0; index < items.length; index++) {
        tmp = items[index].split("=");
        result[result.length] = [tmp[0], decodeURIComponent(tmp[1])];
    }
    return result;
}

function findGetParameter(parameterName) {
    var result = null;

    if (parameterName in getGetParameters()) {
        result = getGetParameters()[parameterName];
    }

    return result;
}