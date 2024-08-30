<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-100">
    <div class="w-full max-w-md p-8 space-y-8 bg-white rounded-xl shadow-2xl">
      <h1 class="text-3xl font-bold text-center text-gray-800">Create Your Account</h1>
      <p class="text-center text-gray-600">Join us and start collaborating</p>
      <form @submit.prevent="register" class="mt-8 space-y-6">
        <div>
          <label for="fullName" class="block text-sm font-medium text-gray-700">Full Name</label>
          <input type="text" id="fullName" v-model="fullName" placeholder="Enter your full name" required
                 class="mt-1 block w-full px-3 py-2 bg-gray-50 border border-gray-300 rounded-md text-sm shadow-sm placeholder-gray-400
                        focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">Email</label>
          <input type="email" id="email" v-model="email" placeholder="Enter your email" required
                 class="mt-1 block w-full px-3 py-2 bg-gray-50 border border-gray-300 rounded-md text-sm shadow-sm placeholder-gray-400
                        focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label for="workspaceName" class="block text-sm font-medium text-gray-700">Workspace Name</label>
          <input type="text" id="workspaceName" v-model="workspaceName" placeholder="Enter your workspace name" required
                 class="mt-1 block w-full px-3 py-2 bg-gray-50 border border-gray-300 rounded-md text-sm shadow-sm placeholder-gray-400
                        focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
          <input type="password" id="password" v-model="password" placeholder="Enter your password" required
                 class="mt-1 block w-full px-3 py-2 bg-gray-50 border border-gray-300 rounded-md text-sm shadow-sm placeholder-gray-400
                        focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500" />
        </div>

        <button type="submit"
                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150 ease-in-out">
          Register
        </button>
      </form>

      <p class="mt-2 text-center text-sm text-gray-600">
        Already have an account?
        <router-link to="/login" class="font-medium text-blue-600 hover:text-blue-500">
          Login here
        </router-link>.
      </p>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      fullName: '',
      email: '',
      workspaceName: '',
      password: '',
    };
  },
  methods: {
    async register() {
      try {
        const user = await this.$store.dispatch('signup', {
          email: this.email,
          fullname: this.fullName,
          password: this.password,
          workspace: this.workspaceName
        });

        console.log('Signup successful, user:', user);
        this.$router.push('/'); // Redirect to chat after successful signup
      } catch (error) {
        console.error('Signup failed:', error);
        // Handle signup failure, show error message to user, etc.
      }
    },
  },
};
</script>
