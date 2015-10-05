$(function() {
    scan_code();
});


function scan_code() {
    var js_files = {};
    $("code").each(function (i, el) {

        var c = $(el).html();

        var lines = c.split("\n");

        var match = lines[0].match("#![a-z0-9]+");
        if (match) {
            var lang = match[0].substr(2);
            $(el).addClass(lang);
            $(el).html(c.substr(c.indexOf("\n") + 1));
        }
    });
}
