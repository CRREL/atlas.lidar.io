<template>
  <div class="image-picker">
    <b-row>
      <b-col cols="2">
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

      <b-col cols="2">
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

      <b-col>
        <div v-if="days">
          <b-row
            v-for="day in days"
            :key="day.day"
            align-v="center"
            class="my-2"
            >
            <b-col cols="2">
              <p>{{ new Date(1986, activeMonth, day.day) | moment("MMM DD") }}</p>
            </b-col>
            <b-col
              v-for="image in day.images"
              :key="image.datetime"
              >
              <b-img-lazy thumbnail fluid :src="image.url" />
            </b-col>
          </b-row>
        </div>
        <p class="text-muted" v-else-if="months">
          Choose a month...
        </p>
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
        let months = Object.keys(this.tree[this.activeYear])
        months.sort((a, b) => a - b)
        return months
      } else {
        return null
      }
    },
    days () {
      if (this.activeYear && this.activeMonth) {
        let days = Object.keys(this.tree[this.activeYear][this.activeMonth])
        days.sort((a, b) => a - b)
        let images = []
        for (let i = 0; i < days.length; i++) {
          images.push({
            day: days[i],
            images: this.tree[this.activeYear][this.activeMonth][days[i]]
          })
        }
        return images
      } else {
        return null
      }
    }
  },
  methods: {
    setActiveYear (year) {
      this.activeYear = year
      if (!this.tree[this.activeYear][this.activeMonth]) {
        this.activeMonth = null
      }
    },
    setActiveMonth (month) {
      this.activeMonth = month
    }
  }
}
</script>
