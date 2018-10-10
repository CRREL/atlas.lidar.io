<template>
  <div class="image-picker">
    <b-row>
      <b-col>
        <h4>Years</h4>
        <b-nav vertical pills>
          <b-nav-item
            v-for="year in years"
            :key="year"
            :active="year === activeYear"
            @click="activeYear = year"
            >
            {{ year }}
          </b-nav-item>
        </b-nav>
      </b-col>
    </b-row>
  </div>
</template>

<script>
export default {
  props: ['tree'],
  data () {
    return {
      activeYear: null
    }
  },
  computed: {
    years () {
      let years = Object.keys(this.tree)
      years.sort()
      return years
    }
  }
}
</script>
