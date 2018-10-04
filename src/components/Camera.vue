<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
      {{ camera.description }}
    </p>

    <b-row>
      <b-col>
        This image was taken {{ camera.latest_image.datetime | moment("from") }}.
      </b-col>

      <b-col>
        <b-img :src="camera.latest_image.url" fluid />
      </b-col>
    </b-row>
  </div>

  <div class="camera" v-else>
    Loading {{ camera.id }}...
  </div>
</template>

<script>
export default {
  props: ['id'],
  computed: {
    camera () {
      return this.$store.getters.camera(this.id)
    }
  }
}
</script>
