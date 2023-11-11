<script setup lang="ts">
import { ref } from 'vue'

interface User {
  username: string
  email: string
  password: string
  confirm: string
}

const user = ref<User>({
  username: '',
  email: '',
  password: '',
  confirm: '',
})

const verg = ref(null)

verg.value = JSON.parse("{\n  \"id\": \"a1b2c3d4-1234-5678-abcd-1234567890ab\",\n  \"name\": \"Rahmenvertrag Fahrzeugservice und Reparatur\",\n  \"description\": \"Diese Ausschreibung betrifft einen Rahmenvertrag für den Fahrzeugservice und die Reparatur von Fahrzeugen im öffentlichen Dienst. Wir suchen qualifizierte Anbieter, die umfassende Dienstleistungen im Bereich Wartung, Reparatur und Inspektion von Fuhrparks anbieten können. Die Ausschreibung umfasst verschiedene Fahrzeugtypen, einschließlich PKWs, Nutzfahrzeugen und Spezialfahrzeugen. Die Vertragslaufzeit beträgt drei Jahre mit der Möglichkeit einer Verlängerung um zwei Jahre.\",\n  \"flags\": [\n    \"fahrzeuge\",\n    \"wartungsvertrag\",\n    \"pkw\",\n    \"rahmenvertrag\",\n    \"lkw\"\n  ],\n  \"files\": [\n    {\n      \"id\": \"f1a2b3c4-5678-4321-dcba-0987654321ab\",\n      \"name\": \"Selbsterklärung russische Sanktionen.pdf\",\n      \"percent_done\": 100,\n      \"is_form\": \"true\",\n      \"mime\": \"application/pdf\"\n    },\n    {\n      \"id\": \"d4e5f6g7-8765-4321-dcba-567890abcdef\",\n      \"name\": \"Allgemeine Bieterinformationen.pdf\",\n      \"percent_done\": 93,\n      \"is_form\": \"true\",\n      \"mime\": \"application/pdf\"\n    },\n    {\n      \"id\": \"h8i9j0k1-2345-6789-abcd-3210987654ef\",\n      \"name\": \"Angebotsdetails.pdf\",\n      \"percent_done\": 100,\n      \"is_form\": \"false\",\n      \"mime\": \"application/pdf\"\n    }\n  ]\n}\n")

function fetch_details() {
    return verg
}

function register() {
  const data = JSON.parse(JSON.stringify(user.value))
  // eslint-disable-next-line no-console
  console.log('Registered: ', data)
}
</script>

<template>
  <div>
    <h3 class="text-3xl font-semibold text-gray-700">
      <!-- {{$route.params.id}} -->
      {{ verg.name }}
    </h3>

    <!--<div class="mt-4">
      <h4 class="text-gray-600">
        Model Form
      </h4>

      <div class="mt-4">
        <div
          class="w-full max-w-sm overflow-hidden bg-white border rounded-md shadow-md"
        >
          <form>
            <div
              class="flex items-center justify-between px-5 py-3 text-gray-700 border-b"
            >
              <h3 class="text-sm">
                Add Category
              </h3>
              <button>
                <svg
                  class="w-4 h-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>

            <div class="px-5 py-6 text-gray-700 bg-gray-200 border-b">
              <label class="text-xs">Name</label>

              <div class="relative mt-2 rounded-md shadow-sm">
                <span
                  class="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-600"
                >
                  <svg
                    class="w-6 h-6"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z"
                    />
                  </svg>
                </span>

                <input
                  type="text"
                  class="w-full px-12 py-2 border-transparent rounded-md appearance-none focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500"
                >
              </div>
            </div>

            <div class="flex items-center justify-between px-5 py-3">
              <button
                class="px-3 py-1 text-sm text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 focus:outline-none"
              >
                Cancel
              </button>
              <button
                class="px-3 py-1 text-sm text-white bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none"
              >
                Save
              </button>
            </div>
          </form>
        </div>
      </div>
    </div> -->

    <div class="mt-8">
      <h4 class="text-gray-600">
        Grundlagen Information
      </h4>

      <div class="mt-4">
        <div class="p-6 bg-white rounded-md shadow-md">
          <h2 class="text-lg font-semibold text-gray-700 capitalize">
            Beschreibung
          </h2>
          <div>
            {{ verg.description }}
          </div>

          <!-- <form @submit.prevent="register">
            <div class="grid grid-cols-1 gap-6 mt-4 sm:grid-cols-2">
              <div>
                <label class="text-gray-700" for="username">Username</label>
                <input
                  v-model="user.username"
                  class="w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500"
                  type="text"
                >
              </div>

              <div>
                <label class="text-gray-700" for="emailAddress">Email Address</label>
                <input
                  v-model="user.email"
                  class="w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500"
                  type="email"
                >
              </div>

              <div>
                <label class="text-gray-700" for="password">Password</label>
                <input
                  v-model="user.password"
                  class="w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500"
                  type="password"
                >
              </div>

              <div>
                <label class="text-gray-700" for="passwordConfirmation">Password Confirmation</label>
                <input
                  v-model="user.confirm"
                  class="w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500"
                  type="password"
                >
              </div>
            </div>

            <div class="flex justify-end mt-4">
              <button
                class="px-4 py-2 text-gray-200 bg-gray-800 rounded-md hover:bg-gray-700 focus:outline-none focus:bg-gray-700"
              >
                Save
              </button>
            </div>
          </form> -->
        </div>
      </div>
    </div>

    <div class="mt-4">
      <h2 class="text-gray-600">
        Dateien
      </h2>

      <div class="mt-6">
        <div class="my-6 overflow-hidden bg-white rounded-md shadow">
          <table class="w-full text-left border-collapse">
            <thead class="border-b">
              <tr>
                <th
                  class="px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800"
                >
                  Dateiname
                </th>
                <!-- <th
                  class="px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800"
                >
                  Dateityp
                </th> -->
                <th
                  class="px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800"
                >
                  Status Befüllung
                </th>
                <th
                  class="px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800"
                >
                  Download/Bearbeiten
                </th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="file in verg.files"
                :key="file.id"
                class="hover:bg-gray-200"
              >
                <td class="px-6 py-4 text-lg text-gray-700 border-b">
                  {{ file.name }}
                </td>
                <td class="px-6 py-4 text-gray-500 border-b">
                  {{ file.percent_done }}
                </td>
                <td class="px-6 py-4 text-gray-500 border-b">
                  <button>
                    <img src="/src/assets/downloads.png" class="w-8 h-8">
                  </button>
                  <button class="mx-4">
                    <img src="/src/assets/edit.png" class="w-8 h-8">
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
