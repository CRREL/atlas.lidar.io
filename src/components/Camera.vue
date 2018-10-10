<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
      {{ camera.description }}
    </p>

    <camera-dual v-if="camera.is_dual" />
    <camera-single v-else :camera="camera" />
  </div>

  <div class="camera" v-else>
    Loading {{ id }}...
  </div>
</template>

<script>
import CameraDual from './CameraDual.vue'
import CameraSingle from './CameraSingle.vue'

export default {
  props: ['id'],
  components: {
    CameraDual,
    CameraSingle
  },
  computed: {
    camera () {
      return this.$store.getters.camera(this.id)
    }
  }
}
</script>
