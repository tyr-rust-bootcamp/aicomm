<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-100">
    <div class="w-full max-w-md p-8 space-y-8 bg-white rounded-xl shadow-2xl">
      <h1 class="text-3xl font-bold text-center text-gray-800">Welcome Back</h1>
      <p class="text-center text-gray-600">Please login to your account</p>
      <form @submit.prevent="login" class="mt-8 space-y-6">
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">Email</label>
          <input type="email" id="email" v-model="email" placeholder="Enter your email" required
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
          Login
        </button>
      </form>

      <p class="mt-2 text-center text-sm text-gray-600">
        Don't have an account?
        <router-link to="/register" class="font-medium text-blue-600 hover:text-blue-500">
          Register here
        </router-link>.
      </p>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      email: '',
      password: '',
    };
  },
  methods: {
    async login() {
      try {
        const user = await this.$store.dispatch('signin', {
          email: this.email,
          password: this.password,
        });

        console.log('Signin successful, user:', user);
        this.$router.push('/'); // Redirect to chat after successful signup
      } catch (error) {
        console.error('Signin failed:', error);
        // Handle signin failure, show error message to user, etc.
      }
    },
  },
};
</script>
