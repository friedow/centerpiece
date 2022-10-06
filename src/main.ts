import { createApp } from 'vue'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faMagnifyingGlass, faRocket, faWindowMaximize } from '@fortawesome/free-solid-svg-icons'

import './index.css'
import App from './App.vue'

library.add(faMagnifyingGlass, faRocket, faWindowMaximize)

createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .mount('#app')
