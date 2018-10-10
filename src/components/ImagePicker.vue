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
            @click="setActiveYear(year)"
            >
            {{ year }}
          </b-nav-item>
        </b-nav>
      </b-col>

      <b-col>
        <h4>Months</h4>
        <b-nav vertical pills v-if="months">
          <b-nav-item
            v-for="month in months"
            :key="month"
            :active="month === activeMonth"
            @click="setActiveMonth(month)"
            >
            {{ new Date(1986, month) | moment("MMM") }}
          </b-nav-item>
        </b-nav>
        <p class="text-muted" v-else>
          Choose a year...
        </p>
      </b-col>
    </b-row>
  </div>
</template>

<script>
export default {
  props: ['tree'],
  data () {
    return {
      activeYear: null,
      activeMonth: null
    }
  },
  computed: {
    years () {
      let years = Object.keys(this.tree)
      years.sort()
      return years
    },
    months () {
      if (this.activeYear) {
        let months = Object.keys(this.tree[this.activeYear]).map(Number)
        months.sort((a, b) => a - b)
        return months
      } else {
        return null
      }
    }
  },
  methods: {
    setActiveYear (year) {
      this.activeYear = year
      this.activeMonth = null
    },
    setActiveMonth (month) {
      this.activeMonth = month
    }
  }
}
</script>
