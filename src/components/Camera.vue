<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
      {{ camera.description }}
    </p>
  </div>

  <div class="camera" v-else>
    Loading {{ id }}...
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
