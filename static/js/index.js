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

    // get prediction from server
    $.ajax({
        url: 'http://127.0.0.1:3000/predict',
        headers: {
            "Access-Control-Allow-Origin": "*",
        },
        type: 'POST',
        ContentType: 'application/json',
        data: {data: input_text}
      }).done(function(prediction)
      {
        $(".messages").append('<div class="other-message message"><div class="chatbot-icon-background-border"><div class="chatbot-icon-background"><i class="fa-solid fa-robot chatbot-icon"></i></div></div><h3>' + prediction + '</h3></div>');
                
      }).fail(function(jqXHR, textStatus, errorThrown)
      { // if ajax POST request fails
        alert('Something went wrong. error: ' + errorThrown);
      });


    $(".type-message-box").val("");
}