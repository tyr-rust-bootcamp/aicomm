<template>
  <div class="flex h-screen">
    <Sidebar />
    <div class="flex flex-col flex-1 relative">
      <MessageList class="flex-1 overflow-y-auto" />
      <MessageSend class="absolute bottom-0 left-0 w-full" />
    </div>
  </div>
</template>

<script>
import Sidebar from '../components/Sidebar.vue';
import MessageList from '../components/MessageList.vue';
import MessageSend from '../components/MessageSend.vue';

export default {
  components: {
    Sidebar,
    MessageList,
    MessageSend,
  },
};
</script>
