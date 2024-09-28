<template>
  <div class="flex flex-col p-4 bg-white border-t border-gray-200">
    <div class="flex items-center">
      <button @click="triggerFileUpload" class="p-2 mr-2 text-gray-600 hover:text-blue-600 focus:outline-none">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
      </button>
      <input
        type="file"
        ref="fileInput"
        @change="handleFileUpload"
        multiple
        accept="image/*"
        class="hidden"
      />
      <input
        v-model="message"
        @keyup.enter="sendMessage"
        placeholder="Type a message..."
        class="flex-1 px-4 py-3 mr-2 text-sm bg-gray-100 border-none rounded-lg focus:outline-none"
        type="text"
      />
      <div v-if="files.length > 0" class="flex mt-2 space-x-2 overflow-x-auto">
        <img
          v-for="file in files"
          :key="file.path"
          :src="file.fullUrl"
          class="w-16 h-16 object-cover rounded"
          alt="Uploaded image"
        />
      </div>
      <button @click="sendMessage" class="p-2 text-white bg-blue-600 rounded-full hover:bg-blue-700 focus:outline-none">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M5 12h14M12 5l7 7-7 7"
          />
        </svg>
      </button>

    </div>

  </div>
</template>

<script>
export default {
  data() {
    return {
      message: '',
      files: [],
    };
  },
  computed: {
    userId() {
      return this.$store.state.user.id;
    },
    activeChannelId() {
      let channel = this.$store.state.activeChannel;
      if (!channel) {
        return null;
      }
      return channel.id;
    },
  },
  methods: {
    sendMessage() {
      if (this.message.trim() === '') return;

      const payload = {
        chatId: this.activeChannelId,
        content: this.message,
        files: this.files.map(file => file.path),
      };

      console.log('Sending message:', payload);

      try {
        this.$store.dispatch('sendMessage', payload);
        this.message = ''; // Clear the input after sending
        this.files = []; // Clear the files after sending
      } catch (error) {
        console.error('Failed to send message:', error);
      }
    },
    triggerFileUpload() {
      this.$refs.fileInput.click();
    },
    async handleFileUpload(event) {
      const files = Array.from(event.target.files);
      if (files.length === 0) return;

      try {
        const uploadedFiles = await this.$store.dispatch('uploadFiles', files);
        this.files = [...this.files, ...uploadedFiles];
      } catch (error) {
        console.error('Failed to upload files:', error);
      }
    },
  },
};
</script>
