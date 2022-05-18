// might want to do all the ML querying here for live update

$(document).on('keypress',function(e) {
    if(e.which == 13) {
        sendMessage();
    }
});

$(".send-message-button").on("click", function() {
    sendMessage();
})

function sendMessage() {
    let input_text = $(".type-message-box").val();

    $(".messages").append('<div class="my-message message"><h3>' + input_text + '</h3></div>');
    $(".messages").append('<div class="other-message message"><div class="chatbot-icon-background-border"><div class="chatbot-icon-background"><i class="fa-solid fa-robot chatbot-icon"></i></div></div><h3>Chatbot Message!</h3></div>');

    $(".type-message-box").val("");
}