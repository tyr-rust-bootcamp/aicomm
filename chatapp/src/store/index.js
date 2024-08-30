import { createStore } from 'vuex';
import axios from 'axios';
import { jwtDecode } from "jwt-decode";
import { getUrlBase } from '../utils';
import { initSSE } from '../utils';
import { formatMessageDate } from '../utils'; // Add this import

export default createStore({
  state: {
    user: null,         // User information
    token: null,        // Authentication token
    workspace: {},      // Current workspace
    channels: [],       // List of channels
    messages: {},       // Messages hashmap, keyed by channel ID
    users: {},         // Users hashmap under workspace, keyed by user ID
    activeChannel: null,
    sse: null,
  },
  mutations: {
    setSSE(state, sse) {
      state.sse = sse;
    },
    setUser(state, user) {
      state.user = user;
    },
    setToken(state, token) {
      state.token = token;
    },
    setWorkspace(state, workspace) {
      state.workspace = workspace;
    },
    setChannels(state, channels) {
      state.channels = channels;
    },
    setUsers(state, users) {
      state.users = users;
    },
    setMessages(state, { channelId, messages }) {
      // Format the date for each message before setting them in the state
      const formattedMessages = messages.map(message => ({
        ...message,
        formattedCreatedAt: formatMessageDate(message.createdAt)
      }));
      state.messages[channelId] = formattedMessages.reverse();
    },
    addChannel(state, channel) {
      state.channels.push(channel);
      state.messages[channel.id] = [];  // Initialize message list for the new channel
    },
    addMessage(state, { channelId, message }) {
      if (state.messages[channelId]) {
        // Format the message date before adding it to the state
        message.formattedCreatedAt = formatMessageDate(message.createdAt);
        state.messages[channelId].push(message);
      } else {
        message.formattedCreatedAt = formatMessageDate(message.createdAt);
        state.messages[channelId] = [message];
      }
    },
    setActiveChannel(state, channelId) {
      const channel = state.channels.find((c) => c.id === channelId);
      state.activeChannel = channel;
    },
    loadUserState(state) {
      const storedUser = localStorage.getItem('user');
      const storedToken = localStorage.getItem('token');
      const storedWorkspace = localStorage.getItem('workspace');
      const storedChannels = localStorage.getItem('channels');
      // we do not store messages in local storage, so this is always empty
      const storedMessages = localStorage.getItem('messages');
      const storedUsers = localStorage.getItem('users');

      if (storedUser) {
        state.user = JSON.parse(storedUser);
      }
      if (storedToken) {
        state.token = storedToken;
      }
      if (storedWorkspace) {
        state.workspace = JSON.parse(storedWorkspace);
      }
      if (storedChannels) {
        state.channels = JSON.parse(storedChannels);
      }
      if (storedMessages) {
        state.messages = JSON.parse(storedMessages);
      }
      if (storedUsers) {
        state.users = JSON.parse(storedUsers);
      }
    },
  },
  actions: {
    initSSE({ state, commit }) {
      if (state.sse) {
        state.sse.close();
      }
      const sse = initSSE(this);
      commit('setSSE', sse);
    },
    closeSSE({ state, commit }) {
      if (state.sse) {
        state.sse.close();
        commit('setSSE', null);
      }
    },
    async signup({ commit }, { email, fullname, password, workspace }) {
      try {
        const response = await axios.post(`${getUrlBase()}/signup`, {
          email,
          fullname,
          password,
          workspace
        });

        const user = await loadState(response, this, commit);

        return user;
      } catch (error) {
        console.error('Signup failed:', error);
        throw error;
      }
    },
    async signin({ commit }, { email, password }) {
      try {
        const response = await axios.post(`${getUrlBase()}/signin`, {
          email,
          password,
        });

        const user = await loadState(response, this, commit);
        return user;
      } catch (error) {
        console.error('Login failed:', error);
        throw error;
      }
    },
    logout({ commit }) {
      // Clear local storage and state
      localStorage.removeItem('user');
      localStorage.removeItem('token');
      localStorage.removeItem('workspace');
      localStorage.removeItem('channels');
      localStorage.removeItem('messages');

      commit('setUser', null);
      commit('setToken', null);
      commit('setWorkspace', '');
      commit('setChannels', []);
      commit('setMessages', {});

      // close SSE
      this.dispatch('closeSSE');
    },
    setActiveChannel({ commit }, channel) {
      commit('setActiveChannel', channel);
    },
    addChannel({ commit }, channel) {
      commit('addChannel', channel);

      // Update the channels and messages in local storage
      localStorage.setItem('channels', JSON.stringify(this.state.channels));
      localStorage.setItem('messages', JSON.stringify(this.state.messages));
    },
    async fetchMessagesForChannel({ state, commit }, channelId) {
      if (!state.messages[channelId] || state.messages[channelId].length === 0) {
        try {
          const response = await axios.get(`${getUrlBase()}/chats/${channelId}/messages`, {
            headers: {
              Authorization: `Bearer ${state.token}`,
            },
          });
        let messages = response.data;
          // messages = messages.map((message) => {
          //   const user = state.users[message.senderId];
          //   return {
          //     ...message,
          //     sender: user,
          //   };
          // } );
          commit('setMessages', { channelId, messages });
        } catch (error) {
          console.error(`Failed to fetch messages for channel ${channelId}:`, error);
        }
      }
    },
    async sendMessage({ state, commit }, payload) {
      try {
        const response = await axios.post(`${getUrlBase()}/chats/${payload.chatId}`, payload, {
          headers: {
            Authorization: `Bearer ${state.token}`,
          },
        });
        console.log('Message sent:', response.data);
        // commit('addMessage', { channelId: payload.chatId, message: response.data });
      } catch (error) {
        console.error('Failed to send message:', error);
        throw error;
      }
    },
    addMessage({ commit }, { channelId, message }) {
      commit('addMessage', { channelId, message });
    },
    loadUserState({ commit }) {
      commit('loadUserState');
      // if user is already logged in, then init SSE
      if (this.state.token) {
        this.dispatch('initSSE');
      }

    },
  },
  getters: {
    isAuthenticated(state) {
      return !!state.token;
    },
    getUser(state) {
      return state.user;
    },
    getUserById: (state) => (id) => {
      return state.users[id];
    },
    getWorkspace(state) {
      return state.workspace;
    },
    getChannels(state) {
      // filter out channels that type == 'single'
      return state.channels.filter((channel) => channel.type !== 'single');
    },
    getSingChannels(state) {
      const channels =  state.channels.filter((channel) => channel.type === 'single');
      // return channel member that is not myself
      return channels.map((channel) => {
        let members = channel.members;
        const id = members.find((id) => id !== state.user.id);
        channel.recipient = state.users[id];
        return channel;
      });
    },
    getChannelMessages: (state) => (channelId) => {
      return state.messages[channelId] || [];
    },
    getMessagesForActiveChannel(state) {
      if (!state.activeChannel) {
        return [];
      }
      return state.messages[state.activeChannel.id] || [];
    },
  },
});

async function loadState(response, self, commit) {
  const token = response.data.token;
  const user = jwtDecode(token); // Decode the JWT to get user info
  const workspace = { id: user.wsId, name: user.wsName };


  try {
    // fetch all workspace users
    const usersResp = await axios.get(`${getUrlBase()}/users`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    const users = usersResp.data;
    const usersMap = {};
    users.forEach((u) => {
      usersMap[u.id] = u;
    });

    // fetch all my channels
    const chatsResp = await axios.get(`${getUrlBase()}/chats`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    const channels = chatsResp.data;


    // Store user info, token, and workspace in localStorage
    localStorage.setItem('user', JSON.stringify(user));
    localStorage.setItem('token', token);
    localStorage.setItem('workspace', JSON.stringify(workspace));
    localStorage.setItem('users', JSON.stringify(usersMap));
    localStorage.setItem('channels', JSON.stringify(channels));

    // Commit the mutations to update the state
    commit('setUser', user);
    commit('setToken', token);
    commit('setWorkspace', workspace);
    commit('setChannels', channels);
    commit('setUsers', usersMap);

    // call initSSE action
    await self.dispatch('initSSE');

    return user;
  } catch (error) {
    console.error('Failed to load user state:', error);
    throw error;
  }

}
