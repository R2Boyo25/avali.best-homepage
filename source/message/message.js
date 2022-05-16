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

function createMessage(content, type = "message") {
    if (type == "success") {
        mbox.innerHTML = ('<div class="message success"><span class="close-message" onclick="remove(this);">X</span>' + content + "</div>") + mbox.innerHTML;
    } else if (type == "error") {
        mbox.innerHTML = ('<div class="message error"><span class="close-message" onclick="remove(this);">X</span>' + content + "</div>") + mbox.innerHTML;
    } else {
        mbox.innerHTML = ('<div class="message"><span class="close-message" onclick="remove(this);">X</span>' + content + "</div>") + mbox.innerHTML;
    }
}