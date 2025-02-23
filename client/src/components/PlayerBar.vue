<template>
<div>

    <div class="row q-mx-md">
        <!-- Meta -->
        <div class="row q-mr-md" style="width: 17vw; max-width: 17vw;">
            <div class="column q-mt-sm q-pt-xs" style="width: calc(100% - 50px);">
                <div class="text-caption text-weight-bold full-width">
                    <div v-if="$1t.player.value.title" class="text-no-wrap overflow-hidden" style="text-overflow: ellipsis">
                        {{ $1t.player.value.title }}
                    </div>
                </div>

                <div class="text-caption full-width text-grey-5">
                    <div v-if="$1t.player.value.artists" class="text-no-wrap overflow-hidden" style="text-overflow: ellipsis">
                        {{ $1t.player.value.artists.join(', ') }}
                    </div>
                </div>
            </div>

            <!-- Controls -->
            <div class="col q-mt-sm" style="margin-left: 16px">
                <!-- Play button -->
                <q-btn
                    round
                    flat
                    icon="mdi-play"
                    class="q-mr-sm"
                    :ripple="false"
                    v-if="!$1t.player.value.playing"
                    @click="$1t.player.value.play()"
                    ref='playButton'
                ></q-btn>
                <!-- Pause -->
                <q-btn
                    round
                    flat
                    icon="mdi-pause"
                    class="q-mr-sm"
                    :ripple="false"
                    v-else
                    @click="$1t.player.value.pause()"
                    ref='playButton'
                ></q-btn>
            </div>
        </div>

        <div class="col">
            <Waveform></Waveform>
        </div>

        <!-- Browse button -->
        <div class="q-mt-sm q-pr-sm">
            <q-btn round icon="mdi-open-in-app" @click="browseQuickTag">
                <q-tooltip>
                    Click here to browse for new path
                </q-tooltip>
            </q-btn>
        </div>

        <!-- Playlist -->
        <div v-if='enablePlaylist'>
            <PlaylistDropZone
                tiny
                v-model="qtPlaylist"
                @update:model-value="loadQTPlaylist(); $1t.quickTagUnfocus()"
                @click.native='$1t.quickTagUnfocus'
                class="q-mt-sm q-mr-sm"
            ></PlaylistDropZone>
        </div>

        <!-- Volume -->
        <div class="q-pt-sm" style="width: 88px">
            <q-slider
                v-model="$1t.player.value.volume"
                :min="0.0"
                :max="1.0"
                :step="0.01"
                @update:model-value="(v) => $1t.player.value.setVolume(v)"
                @change="$1t.saveSettings(false)"
                style="margin-top: 6px"
            ></q-slider>
        </div>

    </div>

</div>
</template>

<script lang='ts' setup>
import Waveform from './Waveform.vue';
import PlaylistDropZone from "./PlaylistDropZone.vue";
import { Playlist } from '../scripts/utils';
import { onDeactivated, onMounted, ref, watch } from 'vue';
import { get1t } from '../scripts/onetagger';
import { useRoute, useRouter } from 'vue-router';

const $1t = get1t();
const qtPlaylist = ref<Playlist>({});
const enablePlaylist = ref(true);
const playButton = ref<any>();

function browseQuickTag() {
    $1t.browse('qt', $1t.settings.value.path);
    $1t.quickTagUnfocus();
}

// Load quicktag playlist
function loadQTPlaylist() {
    if (!qtPlaylist.value || !qtPlaylist.value.data) {
        $1t.loadQuickTag();
        return;
    }
    $1t.loadQuickTag(qtPlaylist.value);
}

onMounted(() => {
    // Unfocus callback
    $1t.quickTagUnfocus = () => {
        playButton.value!.$el.focus();
        playButton.value!.$el.blur();
    }
    // Enable playlist dropzone
    enablePlaylist.value = useRouter().currentRoute.value.path.includes('quicktag');
});

onDeactivated(() => $1t.quickTagUnfocus = () => {});

watch(useRoute(), (r) => {
    if (r.path == '/quicktag') enablePlaylist.value = true;
    else enablePlaylist.value = false;
});
</script>