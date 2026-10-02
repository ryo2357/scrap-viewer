import { mount } from "svelte";
import App from "./App.svelte";

const target = document.getElementById("app");

if (!target) {
  throw new Error("画面の起動先が見つかりません。");
}

const app = mount(App, { target });

export default app;
