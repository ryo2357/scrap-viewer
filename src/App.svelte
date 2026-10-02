<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "./app.css";

  let name = $state("");
  let result = $state("入力して「確認」を押してください。");
  let sending = $state(false);

  async function confirmInput(event: SubmitEvent) {
    event.preventDefault();

    if (sending) return;

    sending = true;
    result = "確認中です。";

    try {
      result = await invoke<string>("greet", { name });
    } catch (error) {
      result =
        typeof error === "string"
          ? error
          : "Rustとの通信に失敗しました。もう一度お試しください。";
    } finally {
      sending = false;
    }
  }
</script>

<main class="mx-auto max-w-xl space-y-6 p-6">
  <h1 class="text-2xl font-bold">Scrap Viewer</h1>
  <p>Tauriの最小画面です。</p>

  <form class="flex flex-col gap-4" onsubmit={confirmInput}>
    <label class="flex flex-col gap-2" for="name">
      <span>確認用の入力</span>
      <input
        id="name"
        class="input input-bordered w-full"
        type="text"
        placeholder="日本語を入力してください"
        bind:value={name}
      />
    </label>
    <button
      class="btn btn-primary self-start"
      type="submit"
      disabled={sending}
    >
      {sending ? "確認中…" : "確認"}
    </button>
  </form>

  <section class="space-y-2" aria-labelledby="result-heading">
    <h2 id="result-heading" class="font-bold">結果</h2>
    <p class="whitespace-pre-wrap break-words" aria-live="polite">
      {result}
    </p>
  </section>
</main>
