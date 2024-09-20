<template>
    <main class="min-h-screen flex items-center ccontainer mx-auto p-2">
        <div class="w-full bg-[var(--content)] border border-black shadow-content rounded-xl p-4">
            <div class="grid grid-cols-9">
                <div class="col-span-2">
                    <div class="relative left-[-17px] top-[-16px]">
                        <img class=" rounded-tl-xl" src="./assets/origins.png" alt="" style="image-rendering:pixelated">
                    </div>
                </div>
                <div v-if="error_msg === null" class="col-span-7 flex flex-col gap-3">
                    <div>
                        <p class="leading-4 text-sm text-white font-semibold" style="text-shadow:0px 1px 1px #000000;">Installed version: {{ installed_version }}</p>
                        <p class="leading-4 text-sm text-white font-semibold underline decoration-wavy" style="text-shadow:0px 1px 1px #000000;">Version available: {{ clienturls?.shockwave_windows_version ?? '?' }}</p>
                    </div>
                    <div class="flex gap-2">
                        <SuccessBtn v-if="clienturls === null || clienturls.shockwave_windows_version <= installed_version" @click="check_update" :disabled="disable_update">Check for updates</SuccessBtn>
                        <SuccessBtn v-else @click="install_update" :disabled="disable_update">Update</SuccessBtn>
                        <InfoBtn @click="launch" :disabled="disable_launch">Launch Habbo:Origins</InfoBtn>
                    </div>
                </div>
                <div v-else class="col-span-7 flex flex-col gap-3">
                   <div class="bg-[var(--text-red-dark)] rounded-xl p-2">
                        <p class="text-white">{{ error_msg }}</p>
                        <p class="text-white underline">
                            Please restart the launcher
                        </p>
                   </div>
                </div>
            </div>
            <input 
                    class="w-full text-[#444] leading-[1.2] bg-[#ccd8df] focus:bg-[#fff] border-[3px] border-[#275d8e] focus:border-[#0074a6] shadow-[inset_0_2px_0_0_#9ebecc] rounded outline-none p-[5px_12px]" 
                    type="text" 
                    placeholder="username" 
                    maxlength="128"
                    v-model="lookup_username"
                    @keypress.enter="lookup"
                />
            <div v-if="this.lookup_result !== null" class="pt-2">
                <div class="bg-[#d8c38c] flex justify-between border border-black shadow-content rounded-xl p-4">
                    <div>
                        <p><span class="font-semibold">name:</span> <span>{{ lookup_result.name }}</span></p>
                        <p><span class="font-semibold">motto:</span> <span>{{ lookup_result.motto }}</span></p>
                        <p><span class="font-semibold">online:</span> <span>{{ lookup_result.online }}</span></p>
                        <p><span class="font-semibold">lastAccessTime:</span> <span>{{ new Date(lookup_result.lastAccessTime).toLocaleString() }}</span></p>
                        <p><span class="font-semibold">memberSince:</span> <span>{{ new Date(lookup_result.memberSince).toLocaleString() }}</span></p>
                        <p><span class="font-semibold">selectedBadges:</span> <span>{{ lookup_result.selectedBadges }}</span></p>
                    </div>
                    <img :src="`https://www.habbo.com/habbo-imaging/avatarimage?size=l&figure=${ lookup_result.figureString }&size=b&direction=4&head_direction=4&crr=0&gesture=sml&frame=1`" alt="">
                </div>
            </div>
        </div>
    </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";

import SuccessBtn from "./components/buttons/success.vue";
import InfoBtn from "./components/buttons/info.vue";

export default {
    components: {SuccessBtn, InfoBtn},
    data: () => ({
        installed_version: 0,
        clienturls: null,
        lookup_username: "",
        lookup_result: null,

        disable_update: false,
        disable_launch: true,

        error_msg: null,
    }),
    async beforeMount() {
        try {
            this.installed_version = await invoke('installed_version');
            if (this.installed_version > 0) {
                this.disable_launch = false;
            }
        } catch (err) {
            console.error(err)
            this.error_msg = err;
        }
    },
    methods: {
        async check_update() {
            this.disable_update = true;

            try {
                this.clienturls = await invoke('check_update');
            } catch (err) {
                console.error(err)
                this.error_msg = err;
            }

            this.disable_update = false;
        },
        async install_update() {
            this.disable_update = true;
            this.disable_launch = true;

            try {
                this.installed_version = await invoke('install_update');
                this.disable_launch = false;
            } catch (err) {
                console.error(err)
                this.error_msg = err;
            }

            this.disable_update = false;
        },
        async launch() {
            try {
                await invoke('launch');
            } catch (err) {
                /// TOODO
            }
        },
        async lookup() {
            if (this.lookup_username === "") {
                this.lookup_result = null;
                return;
            }

            try {
                let res = await invoke('user_lookup', { username: this.lookup_username });
                this.lookup_result = JSON.parse(res);
            } catch (err) {
                console.error(err)
                this.error_msg = err;

                this.lookup_result = null;
            }
        }
    }
}
</script>

<style lang="scss">
html, body {
    &::-webkit-scrollbar {
        display: none;
    }
}
body {
    background-image: url(./assets/bg.jpg);
    background-color: #161616;
}

.ccontainer {
    max-width: 1200px;
    width: 100%;
}

.shadow-content {
    box-shadow: 0 2px 3px rgba(6, 19, 25, .7);
}
</style>
