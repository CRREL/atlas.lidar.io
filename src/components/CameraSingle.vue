<template>
  <div class="camera-single">
    <b-row v-if="camera.latest_image">
      <b-col offset-md="3" md="6">
        <b-img :src="camera.latest_image.url" fluid />
        <p>
          {{ camera.latest_image.datetime | moment("from") }}
        </p>
      </b-col>
    </b-row>

    <div v-if="images">
      Images here
    </div>
  </div>
</template>

<script>
import api from '../api'

export default {
  props: ['camera', 'images'],
  mounted () {
    api.cameraImages(this.camera.id, images => { this.images = images })
  }
}
</script>
