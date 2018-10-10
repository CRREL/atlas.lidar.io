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
  </div>
</template>

<script>
export default {
  props: ['camera']
}
</script>
