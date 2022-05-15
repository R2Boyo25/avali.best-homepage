let closeHTML = '<span class="close-message" onclick="remove(this);">X</span>';

function remove(p) {
    p.parentNode.parentNode.removeChild(p.parentNode);
}

function updateMessage(el) {
    if (el.getElementsByClassName("close-message").length == 0) {
        el.innerHTML = closeHTML + el.innerHTML;
    }
}

function updateMessages() {
    Array.from(document.getElementsByClassName("message")).forEach(updateMessage);
}

updateMessages();
var t = setInterval(updateMessages, 1000);