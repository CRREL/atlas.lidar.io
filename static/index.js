$('[data-toggle="tooltip"]').tooltip();

$(".camera-picker-thumbnail").click(function() {
    var name = $(this).data("name");
    $(".camera-picker-image").addClass("hidden");
    $(".camera-picker-image[data-name=" + name + "]").removeClass("hidden");
});
$(".camera-picker-thumbnail")[0].click();

$(".camera-picker-animate").click(function() {
    if ($(this).data("animate")) {
        deanimate(this);
    } else {
        animate(this);
    }
});
function animate(button) {
    $(".camera-picker-images").addClass("hidden");
    $(".camera-picker-gifs").removeClass("hidden");
    $(button).find(".glyphicon").removeClass("glyphicon-play").addClass("glyphicon-stop");
    $(button).data("animate", true);
}
function deanimate(button) {
    $(".camera-picker-images").removeClass("hidden");
    $(".camera-picker-gifs").addClass("hidden");
    $(button).find(".glyphicon").removeClass("glyphicon-stop").addClass("glyphicon-play");
    $(button).data("animate", false);
}
deanimate($(".camera-picker-animate"));

var soc_graph = new Dygraph(document.getElementById("soc-graph"), "/api/v1/csv/soc", {
    dateWindow: [Date.now() - 2 * 30 * 24 * 60 * 60 * 1000, Date.now()],
    height: 300,
    labelsUTC: true,
    rollPeriod: 6,
    showRangeSelector: true,
    showRoller: true,
    axes: {
        y: {
            axisLabelFormatter: function(y) {
                return y + "%";
            }
        }
    }
});

var temperature_graph = new Dygraph(document.getElementById("temperature-graph"), "/api/v1/csv/temperature", {
    dateWindow: [Date.now() - 2 * 30 * 24 * 60 * 60 * 1000, Date.now()],
    height: 300,
    labelsUTC: true,
    rollPeriod: 24,
    showRangeSelector: true,
    showRoller: true,
    axes: {
        y: {
            axisLabelFormatter: function(y) {
                return y + "&deg;C";
            }
        }
    }
});

var temperature_graph = new Dygraph(document.getElementById("pressure-rh-graph"), "/api/v1/csv/pressure-rh", {
    dateWindow: [Date.now() - 2 * 30 * 24 * 60 * 60 * 1000, Date.now()],
    height: 300,
    labelsUTC: true,
    rollPeriod: 24,
    series: {
        "Relative humidity": {
            axis: "y2"
        }
    },
    showRangeSelector: true,
    showRoller: true,
    ylabel: "Pressure (mBar)",
    y2label: "Relative humidity (%)"
});

var _ = Dygraph.synchronize(soc_graph, temperature_graph, { range: false });
