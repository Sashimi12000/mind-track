<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // プロップス
  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSuccess?: (message: string) => void;
    onError?: (message: string) => void;
  }

  let { isOpen = $bindable(), onClose, onSuccess, onError }: Props = $props();

  // 状態管理
  let formData = $state({
    date: new Date().toISOString().split('T')[0], // YYYY-MM-DD
    moodLevel: 0, // 1-5, 0は未選択
    moodText: '', // moodMemoからmoodTextに変更
    physicalStateTags: [] as string[],
    newTag: '',
    potentialTodos: ['', '', ''], // 最大3つ
    // feelingForTodosは削除（バックエンドから削除されたため）
  });

  let errors = $state({
    moodLevel: '',
    moodText: '', // moodMemoからmoodTextに変更
    physicalStateTags: '',
    potentialTodos: '',
    // feelingForTodosは削除
  });

  let isSubmitting = $state(false);

  // 事前定義された体の状態タグ
  const predefinedTags = [
    '寝不足', '元気', '頭痛', '肩こり', 
    'リラックス', 'ストレス', '疲労感'
  ];

  // 気分レベルの選択肢（仕様書通り: 1=とても良い, 5=とても悪い）
  const moodOptions = [
    { level: 1, emoji: '😄', label: 'とても良い' },
    { level: 2, emoji: '🙂', label: '良い' },
    { level: 3, emoji: '😐', label: '普通' },
    { level: 4, emoji: '😥', label: '悪い' },
    { level: 5, emoji: '😭', label: 'とても悪い' }
  ];

  // 日付のフォーマット（仕様書通り: YYYY年MM月DD日 (曜日)）
  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    const weekdays = ['日', '月', '火', '水', '木', '金', '土'];
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const day = date.getDate();
    const weekday = weekdays[date.getDay()];
    return `${year}年${month}月${day}日 (${weekday})`;
  }

  // タグの追加
  function addTag() {
    if (formData.newTag.trim() && !formData.physicalStateTags.includes(formData.newTag.trim())) {
      if (formData.physicalStateTags.length < 10) { // 最大10個
        formData.physicalStateTags = [...formData.physicalStateTags, formData.newTag.trim()];
        formData.newTag = '';
        errors.physicalStateTags = '';
      } else {
        errors.physicalStateTags = '体の状態タグは最大10個まで選択可能です。';
      }
    }
  }

  // タグの削除
  function removeTag(tag: string) {
    formData.physicalStateTags = formData.physicalStateTags.filter(t => t !== tag);
  }

  // 事前定義タグのトグル
  function togglePredefinedTag(tag: string) {
    if (formData.physicalStateTags.includes(tag)) {
      removeTag(tag);
    } else {
      if (formData.physicalStateTags.length < 10) {
        formData.physicalStateTags = [...formData.physicalStateTags, tag];
        errors.physicalStateTags = '';
      } else {
        errors.physicalStateTags = '体の状態タグは最大10個まで選択可能です。';
      }
    }
  }

  // バリデーション
  function validateForm(): boolean {
    errors = {
      moodLevel: '',
      moodText: '',
      physicalStateTags: '',
      potentialTodos: '',
    };

    let isValid = true;

    // 気分レベルは必須
    if (formData.moodLevel === 0) {
      errors.moodLevel = '今の気分を選択してください。';
      isValid = false;
    }

    // 気分メモの文字数制限
    if (formData.moodText.length > 500) {
      errors.moodText = '気分メモは500文字以内で入力してください。';
      isValid = false;
    }

    // やらなきゃリストのバリデーション
    const nonEmptyTodos = formData.potentialTodos.filter(todo => todo.trim() !== '');
    for (const todo of nonEmptyTodos) {
      if (todo.length > 100) {
        errors.potentialTodos = '各項目は100文字以内で入力してください。';
        isValid = false;
        break;
      }
    }

    return isValid;
  }

  // フォーム送信
  async function handleSubmit() {
    if (!validateForm()) return;

    isSubmitting = true;
    try {
      const payload = {
        date: formData.date,
        mood_level: formData.moodLevel,
        mood_text: formData.moodText || null, // moodMemoからmoodTextに変更
        physical_state_tags: formData.physicalStateTags.length > 0 ? formData.physicalStateTags : null,
        potential_todos: formData.potentialTodos.filter(todo => todo.trim() !== ''),
        // feeling_for_todosは削除
      };

      const result = await invoke('record_daily_checkin', { payload });
      console.log('チェックイン記録成功:', result);
      
      // 成功メッセージ表示
      if (onSuccess) {
        onSuccess('今日も一日、あなたらしく。');
      }
      
      // モーダルを閉じる
      onClose();
      
    } catch (error) {
      console.error('チェックイン記録エラー:', error);
      if (onError) {
        onError('記録に失敗しました。もう一度お試しください。');
      }
    } finally {
      isSubmitting = false;
    }
  }

  // モーダル外クリックで閉じる
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
</script>

<!-- モーダル -->
{#if isOpen}
  <div 
    class="modal modal-open" 
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-box w-11/12 max-w-2xl">
      <!-- ヘッダー -->
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-bold">今日のチェックイン</h2>
        <button class="btn btn-sm btn-circle btn-ghost" onclick={onClose}>✕</button>
      </div>

      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
        <!-- 日付表示 -->
        <div>
          <span class="label-text font-semibold">日付</span>
          <div class="text-lg text-base-content">
            {formatDate(formData.date)}
          </div>
        </div>

        <!-- 今の気分 -->
        <fieldset>
          <legend class="label-text font-semibold">今の気分 <span class="text-error">*</span></legend>
          <div class="flex gap-4 justify-center mt-2">
            {#each moodOptions as option}
              <button
                type="button"
                class="btn btn-circle btn-lg {formData.moodLevel === option.level ? 'btn-primary' : 'btn-outline'}"
                onclick={() => {
                  formData.moodLevel = option.level;
                  errors.moodLevel = '';
                }}
                title={option.label}
              >
                <span class="text-2xl">{option.emoji}</span>
              </button>
            {/each}
          </div>
          {#if errors.moodLevel}
            <div class="label">
              <span class="label-text-alt text-error">{errors.moodLevel}</span>
            </div>
          {/if}
        </fieldset>

        <!-- 気分の補足 -->
        <div>
          <label for="mood-memo" class="label">
            <span class="label-text font-semibold">気分の補足</span>
          </label>
          <textarea
            id="mood-memo"
            class="textarea textarea-bordered w-full h-24"
            placeholder="例: 昨日よく眠れたので気分が良い"
            bind:value={formData.moodText}
            maxlength={500}
          ></textarea>
          <div class="label">
            <span class="label-text-alt">{formData.moodText.length}/500文字</span>
            {#if errors.moodText}
              <span class="label-text-alt text-error">{errors.moodText}</span>
            {/if}
          </div>
        </div>

        <!-- 体の状態（タグ形式） -->
        <fieldset>
          <legend class="label-text font-semibold">体の状態</legend>
          
          <!-- 事前定義タグ -->
          <div class="flex flex-wrap gap-2 mb-3 mt-2">
            {#each predefinedTags as tag}
              <button
                type="button"
                class="btn btn-sm {formData.physicalStateTags.includes(tag) ? 'btn-primary' : 'btn-outline'}"
                onclick={() => togglePredefinedTag(tag)}
              >
                {tag}
              </button>
            {/each}
          </div>

          <!-- 自由記述タグ追加 -->
          <div class="flex gap-2 mb-3">
            <input
              type="text"
              class="input input-bordered flex-1"
              placeholder="自由にタグを追加"
              bind:value={formData.newTag}
              maxlength={20}
              onkeydown={(e) => e.key === 'Enter' && (e.preventDefault(), addTag())}
            />
            <button type="button" class="btn btn-outline" onclick={addTag}>追加</button>
          </div>

          <!-- 選択済みタグ表示 -->
          {#if formData.physicalStateTags.length > 0}
            <div class="flex flex-wrap gap-2">
              {#each formData.physicalStateTags as tag}
                <div class="badge badge-primary gap-2">
                  {tag}
                  <button type="button" class="text-xs" onclick={() => removeTag(tag)}>✕</button>
                </div>
              {/each}
            </div>
          {/if}

          {#if errors.physicalStateTags}
            <div class="label">
              <span class="label-text-alt text-error">{errors.physicalStateTags}</span>
            </div>
          {/if}
        </fieldset>

        <!-- 今日の「やらなきゃ」と感じること -->
        <fieldset>
          <legend class="label-text font-semibold">今日の「やらなきゃ」と感じること</legend>
          <div class="mt-2">
            {#each formData.potentialTodos as todo, index}
              <input
                type="text"
                class="input input-bordered w-full mb-2"
                placeholder="例: 朝の散歩をする"
                bind:value={formData.potentialTodos[index]}
                maxlength={100}
              />
            {/each}
          </div>
          {#if errors.potentialTodos}
            <div class="label">
              <span class="label-text-alt text-error">{errors.potentialTodos}</span>
            </div>
          {/if}
        </fieldset>

        <!-- ボタン -->
        <div class="modal-action">
          <button type="button" class="btn btn-ghost" onclick={onClose}>キャンセル</button>
          <button 
            type="submit" 
            class="btn btn-primary" 
            disabled={isSubmitting}
          >
            {isSubmitting ? '記録中...' : '記録する'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
