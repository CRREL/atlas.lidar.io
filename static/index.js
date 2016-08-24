$(".camera-picker-thumbnail").click(function() {
  var name = $(this).data("name");
  $(".camera-picker-image").addClass("hidden");
  $(".camera-picker-image[data-name=" + name + "]").removeClass("hidden");
});
$(".camera-picker-thumbnail")[0].click();

var soc_graph = new Dygraph(document.getElementById("soc-graph"), "/api/v1/soc", {
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
