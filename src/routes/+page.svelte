<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DailyCheckinModal from "$lib/components/DailyCheckinModal.svelte";
  // 状態管理
  let isCheckinModalOpen = $state(false);
  let todayCheckin: any = $state(null);
  let showToast = $state(false);
  let toastMessage = $state("");
  let toastType = $state("success"); // "success" | "error"

  // 今日の日付 (YYYY-MM-DD形式)
  function getTodayString(): string {
    return new Date().toISOString().split('T')[0];
  }

  // 今日のチェックインを取得
  async function loadTodayCheckin() {
    try {
      const today = getTodayString();
      const checkin = await invoke('get_daily_checkin_by_date', { date: today });
      todayCheckin = checkin;
    } catch (error) {
      // 今日のチェックインがない場合はnullのまま
      console.log('今日のチェックインはまだありません');
    }
  }

  // トースト表示
  function showToastMessage(message: string, type: "success" | "error" = "success") {
    toastMessage = message;
    toastType = type;
    showToast = true;
    
    // 3秒後に自動的に非表示
    setTimeout(() => {
      showToast = false;
    }, 3000);
  }

  // チェックインモーダルを開く
  function openCheckinModal() {
    isCheckinModalOpen = true;
  }

  // チェックインモーダルを閉じる
  function closeCheckinModal() {
    isCheckinModalOpen = false;
    // モーダルが閉じられたら今日のチェックインを再読み込み
    loadTodayCheckin();
  }

  // アプリケーション起動時の処理
  onMount(async () => {
    await loadTodayCheckin();
    
    // 今日のチェックインがない場合、自動的にモーダルを表示
    if (!todayCheckin) {
      openCheckinModal();
    }
  });
</script>

<main class="min-h-screen bg-base-100">  <!-- ヘッダー -->
  <div class="navbar bg-primary text-primary-content">
    <div class="flex-1">
      <span class="text-xl font-bold">Mind Track</span>
    </div>
    <div class="flex-none">
      <button class="btn btn-ghost" onclick={openCheckinModal}>
        📝 チェックイン
      </button>
    </div>
  </div>

  <!-- メインコンテンツ -->
  <div class="container mx-auto px-4 py-8">
    <div class="hero">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <h1 class="text-5xl font-bold">Mind Track</h1>
          <p class="py-6">
            毎日の心身の状態を記録し、自己理解を深めるためのアプリケーションです。
          </p>
          
          {#if todayCheckin}
            <div class="alert alert-success">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span>今日のチェックインは完了しています！</span>
            </div>
          {:else}
            <div class="alert alert-warning">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
              <span>今日のチェックインがまだ完了していません。</span>
            </div>
          {/if}          <div class="mt-6">
            <button class="btn btn-primary btn-lg" onclick={openCheckinModal}>
              {todayCheckin ? 'チェックインを編集' : '今日のチェックイン'}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 今後の機能エリア -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
      <div class="card bg-base-200 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">📋 マイクロタスクプランナー</h2>
          <p>小さなタスクを計画し、達成感を積み重ねましょう。</p>
          <div class="card-actions justify-end">
            <button class="btn btn-primary btn-sm" disabled>準備中</button>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">🏆 達成ログ & フィードバック</h2>
          <p>あなたの頑張りを記録し、ポジティブなフィードバックを得ましょう。</p>
          <div class="card-actions justify-end">
            <button class="btn btn-primary btn-sm" disabled>準備中</button>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">🔔 リマインダー</h2>
          <p>大切な習慣を忘れずに続けるためのリマインダー機能。</p>
          <div class="card-actions justify-end">
            <button class="btn btn-primary btn-sm" disabled>準備中</button>
          </div>
        </div>
      </div>
    </div>
  </div>  <!-- デイリーチェックインモーダル -->
  <DailyCheckinModal 
    bind:isOpen={isCheckinModalOpen} 
    onClose={closeCheckinModal}
    onSuccess={(message: string) => showToastMessage(message, 'success')}
    onError={(message: string) => showToastMessage(message, 'error')}
  />

  <!-- トースト -->
  {#if showToast}
    <div class="toast toast-end">
      <div class="alert {toastType === 'success' ? 'alert-success' : 'alert-error'}">
        <span>{toastMessage}</span>
      </div>
    </div>
  {/if}
</main>


