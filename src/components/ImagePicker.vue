<template>
  <div class="image-picker">
    {{ years }}
  </div>
</template>

<script>
export default {
  props: ['tree'],
  computed: {
    years () {
      let years = Object.keys(this.tree)
      years.sort()
      return years
    }
  }
}
</script>
