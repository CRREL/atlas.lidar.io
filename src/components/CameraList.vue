<template>
  <div id="camera-list">
    <b-card-group deck>
      <camera-card id="ATLAS_CAM2"></camera-card>
      <camera-card id="HEL_ATLASNORTH"></camera-card>
    </b-card-group>
    <b-card-group deck>
      <camera-card id="ATLAS_CAM"></camera-card>
      <camera-card id="HEL_BERGCAM3"></camera-card>
    </b-card-group>
    <b-card-group deck>
      <camera-card id="HEL_DUAL"></camera-card>
      <camera-card id="HEL_BERGCAM1"></camera-card>
      <camera-card id="HEL_BERGCAM2"></camera-card>
    </b-card-group>
  </div>
</template>

<script>
import CameraCard from './CameraCard.vue'

export default {
  components: {
    CameraCard
  }
}
</script>
