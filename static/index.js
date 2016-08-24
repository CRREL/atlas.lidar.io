$(".camera-picker-thumbnail").click(function() {
  var name = $(this).data("name");
  $(".camera-picker-image").addClass("hidden");
  $(".camera-picker-image[data-name=" + name + "]").removeClass("hidden");
});
$(".camera-picker-thumbnail")[0].click();
