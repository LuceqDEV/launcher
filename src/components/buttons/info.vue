<template>
    <button 
        type="submit"
        class="text-center bg-[var(--btn-info)] hover:enabled:bg-[var(--btn-info-hover)] active:enabled:bg-[var(--btn-info-active)] border-2 border-[var(--btn-info-border)] hover:enabled:border-[var(--btn-info-hover-border)] active:enabled:border-[var(--btn-info-active-border)] shadow-[0_3px_0_1px_rgba(0,0,0,.3)] active:enabled:shadow-[0_1px_0_1px_rgba(0,0,0,.3)] transform active:enabled:translate-y-[2px] rounded px-4 py-2"
        :disabled="disabled === true"
        :style="{ opacity: disabled === true ? '0.5' : '1' }"
    >
        <div class="text-white flex gap-1">
            <slot></slot>
        </div>
    </button>
</template>

<script>
export default {
    props: {
        disabled: {
            type: Boolean,
            default: false
        }
    }
}
</script>