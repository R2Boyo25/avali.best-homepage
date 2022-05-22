// Modified from https://stackoverflow.com/a/5448595
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

// https://www.kirupa.com/html5/get_element_position_using_javascript.htm
function getPosition(el) {
    var xPos = 0;
    var yPos = 0;

    while (el) {
        if (el.tagName == "BODY") {
            // deal with browser quirks with body/window/document and page scroll
            var xScroll = el.scrollLeft || document.documentElement.scrollLeft;
            var yScroll = el.scrollTop || document.documentElement.scrollTop;

            xPos += (el.offsetLeft - xScroll + el.clientLeft);
            yPos += (el.offsetTop - yScroll + el.clientTop);
        } else {
            // for all other non-BODY elements
            xPos += (el.offsetLeft - el.scrollLeft + el.clientLeft);
            yPos += (el.offsetTop - el.scrollTop + el.clientTop);
        }

        el = el.offsetParent;
    }
    return {
        x: xPos,
        y: yPos
    };
}

function ctToRGB(cta) {
    return "rgb(" + cta[0] + ", " + cta[1] + ", " + cta[2] + ")";
}