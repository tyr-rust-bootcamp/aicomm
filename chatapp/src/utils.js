import { invoke } from '@tauri-apps/api/core';

const URL_BASE = 'http://localhost:6688/api';
const SSE_URL = 'http://localhost:6687/events';

let config = null;
try {
  if (invoke) {
    config = await invoke('get_config');
  }
} catch (error) {
  console.warn('failed to get config: fallback');
}

const getUrlBase = () => {
  if (config && config.server.chat) {
    return config.server.chat;
  }
  return URL_BASE;
}

const getSseBase = () => {
  if (config && config.server.notification) {
    return config.server.notification;
  }
  return SSE_URL;
}

const initSSE = (store) => {
  let sse_base = getSseBase();
  let url = `${sse_base}?token=${store.state.token}`;
  const sse = new EventSource(url);

  sse.addEventListener("NewMessage", (e) => {
    let data = JSON.parse(e.data);
    console.log('message:', e.data);
    delete data.event;
    store.commit('addMessage', { channelId: data.chatId, message: data });
  });

  sse.onmessage = (event) => {
    console.log('got event:', event);
    // const data = JSON.parse(event.data);
    // commit('addMessage', data);
  };

  sse.onerror = (error) => {
    console.error('EventSource failed:', error);
    sse.close();
  };

  return sse;
}

export {
  getUrlBase,
  initSSE,
};

export function formatMessageDate(timestamp) {
  const date = new Date(timestamp);
  const now = new Date();
  const diffDays = Math.floor((now - date) / (1000 * 60 * 60 * 24));
  const timeString = date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

  if (diffDays === 0) {
    return timeString;
  } else if (diffDays < 30) {
    return `${timeString}, ${diffDays} ${diffDays === 1 ? 'day' : 'days'} ago`;
  } else {
    return `${timeString}, ${date.toLocaleDateString([], { month: 'short', day: 'numeric', year: 'numeric' })}`;
  }
}
