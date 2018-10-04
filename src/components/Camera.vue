<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
      {{ camera.description }}
    </p>

    <b-row v-if="camera.is_dual">
      Dual camera
    </b-row>
    <b-row v-else>
      Not a dual camera
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
