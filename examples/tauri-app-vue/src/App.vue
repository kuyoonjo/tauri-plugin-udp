<template>
  <main class="container">
    <h1>Tauri plugin UDP demo</h1>

    <!-- 监听端口 -->
    <div class="section">
      <h2>接收</h2>
      <div class="row">
        <input v-model="listen_port" type="number" placeholder="监听端口" />
        <button @click="start_listen_udp">监听</button>
      </div>
      <textarea class="recv-box" readonly v-model="recv_msg" placeholder="接收到的数据"></textarea>
    </div>

    <!-- 目标地址端口 -->
    <div class="section">
      <h2>发送</h2>
      <div class="row">
        <input v-model="target_addr" type="text" placeholder="目标地址" />
        <input v-model="target_port" type="text" placeholder="目标端口" />
        <input v-model="send_msg" type="text" placeholder="发送消息" />
        <button @click="send_udp">发送</button>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import * as udp from "@kuyoonjo/tauri-plugin-udp";

interface UdpMsg {
  id: string,
  addr: string,
  data: ArrayBuffer,
}

let udp_id = "test"
const listen_port = ref(12345);
const recv_msg = ref("")

async function start_listen_udp() {
  if (udp_id === "test") {
    udp_id = "test" + Math.random().toString(36).substring(2, 15)
  }
  try {
    await udp.bind(udp_id, `0.0.0.0:${listen_port.value}`);
    console.log(`listen udp ${udp_id}`);
  } catch (e) {
    console.error(e);
    udp_id = "test"
    return
  }
  // bind udp recv callback
  const unlisen = await listen("plugin://udp", (event) => {
    const payload = event.payload as UdpMsg
    if (payload.id !== udp_id) {
      console.log(`ignore msg from ${payload.id}`);
      unlisen();
      return
    }
    console.log(`recv msg from ${payload.addr}`);
    console.log(payload.data);
    const decoder = new TextDecoder();
    const msg = decoder.decode(new Uint8Array(payload.data));
    console.log(msg);
    recv_msg.value += msg + "\n"
  })
}

const target_addr = ref("127.0.0.1")
const target_port = ref("2345")
const send_msg = ref("HelloWorld")

async function send_udp() {
  if (udp_id === "test") {
    console.warn("udp not bind")
    return
  }
  console.log(`send msg to ${target_addr.value}:${target_port.value}`);
  console.log(send_msg.value);
  udp.send(udp_id, `${target_addr.value}:${target_port.value}`, send_msg.value)
}

</script>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.row {
  display: flex;
  justify-content: center;
}

.recv-box {
  width: 95%;
  height: 200px;
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 10px;
  font-family: monospace;
  font-size: 14px;
  resize: none;
  background-color: #f9f9f9;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>