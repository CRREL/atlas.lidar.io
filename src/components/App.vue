<template>
  <div id="app">
    <navbar></navbar>
    <b-container id="app">
      <router-view></router-view>
    </b-container>
  </div>
</template>

<script>
import Navbar from './Navbar.vue'

export default {
  components: {
    Navbar
  },
  mounted () {
    this.$store.dispatch('fetchCameras')
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
