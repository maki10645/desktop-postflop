<template>
  <div class="flex mt-1">
    <div class="shrink-0 ml-1">
      <table class="shadow-md select-none snug" @mouseleave="dragEnd">
        <tr v-for="row in 13" :key="row" class="h-9">
          <td
            v-for="col in 13"
            :key="col"
            class="relative w-[2.625rem] border border-black"
            @mousedown="dragStart(row, col)"
            @mouseup="dragEnd"
            @mouseenter="mouseEnter(row, col)"
          >
            <div
              :class="
                'absolute w-full h-full left-0 top-0 ' +
                (row === col ? 'bg-neutral-700' : 'bg-neutral-800')
              "
            >
              <div
                class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
                :style="{
                  'background-image': `linear-gradient(${yellow500} 0% 100%)`,
                  'background-size': `100% ${cellValue(row, col)}%`,
                }"
              ></div>
            </div>
            <div
              :class="
                'absolute -top-px left-[0.1875rem] z-10 text-shadow ' +
                (cellValue(row, col) > 0 ? 'text-white' : 'text-neutral-500')
              "
            >
              {{ cellText(row, col) }}
            </div>
            <div
              class="absolute bottom-px right-1 z-10 text-sm text-shadow text-white"
            >
              {{
                cellValue(row, col) > 0 && cellValue(row, col) < 100
                  ? cellValue(row, col).toFixed(1)
                  : ""
              }}
            </div>
          </td>
        </tr>
      </table>

      <div class="mt-5">
        <div class="flex">
          <input
            v-model="store.rangeText[props.player]"
            type="text"
            :class="
              'flex-grow mr-6 px-2 py-1 rounded-lg text-sm ' +
              (rangeTextError ? 'input-error' : '')
            "
            @focus="($event.target as HTMLInputElement).select()"
            @change="onRangeTextChange"
          />

          <button
            v-if="player >= 2"
            class="mr-4 button-base button-blue"
            @click="invertRange"
          >
            Invert
          </button>

          <button class="button-base button-blue" @click="clearRange">
            Clear
          </button>
        </div>

        <div v-if="rangeTextError" class="mt-1 text-red-500">
          Error: {{ rangeTextError }}
        </div>
      </div>

      <div class="flex mt-3.5 items-center">
        <div>
          Weight:
          <input
            v-model="weight"
            type="range"
            class="ml-3 w-40 align-middle"
            min="0"
            max="100"
            step="5"
            @change="onWeightChange"
          />
          <input
            v-model="weight"
            type="number"
            :class="
              'w-20 ml-4 px-2 py-1 rounded-lg text-sm text-center ' +
              (weight < 0 || weight > 100 ? 'input-error' : '')
            "
            min="0"
            max="100"
            step="5"
            @change="onWeightChange"
          />
          %
        </div>

        <span class="inline-block ml-auto">
          {{ store.rangeCombos[props.player].toFixed(1) }} combos ({{
            ((store.rangeCombos[props.player] * 100) / ((52 * 51) / 2)).toFixed(1)
          }}%)
        </span>
      </div>
    </div>

    <div class="flex-grow max-w-[18rem] ml-6">
      <DbItemPicker
        store-name="ranges"
        :index="player"
        :value="store.rangeText[props.player]"
        :allow-save="store.rangeText[props.player] !== '' && rangeTextError === ''"
        :hide-import-export="player >= 2"
        @load-item="loadRange"
      />

      <div v-if="player >= 2" class="flex mt-12 justify-center gap-4">
        <button class="button-base button-blue !px-5" @click="emit('save')">
          Save Edit
        </button>
        <button class="button-base button-red !px-5" @click="emit('cancel')">
          Cancel Edit
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import * as invokes from "../invokes";
import { useStore } from "../store";
import { rangeRegex, ranks, trimRegex } from "../utils";

import DbItemPicker from "./DbItemPicker.vue";

const yellow500 = "#eab308";

type DraggingMode = "none" | "enabling" | "disabling";

const props = withDefaults(
	defineProps<{ player: number; defaultText?: string }>(),
	{ defaultText: "" },
);

const emit = defineEmits<{
	(event: "save"): void;
	(event: "cancel"): void;
}>();

const store = useStore();
//const rangeText = ref(props.defaultText);
store.rangeText[props.player] = "";
const rangeTextError = ref("");
const weight = ref(100);
//const numCombos = ref(0);
store.rangeCombos[props.player] = 0;

let draggingMode: DraggingMode = "none";

const cellText = (row: number, col: number) => {
	const r1 = 13 - Math.min(row, col);
	const r2 = 13 - Math.max(row, col);
	return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
};

const cellIndex = (row: number, col: number) => {
	return 13 * (row - 1) + col - 1;
};

const cellValue = (row: number, col: number) => {
	return store.ranges[props.player][cellIndex(row, col)];
};

const onUpdate = async () => {
	store.rangeText[props.player] = await invokes.rangeToString(props.player);
	store.rangeCombos[props.player] = await invokes.rangeNumCombos(props.player);
	rangeTextError.value = "";
};

const update = async (row: number, col: number, weight: number) => {
	const idx = 13 * (row - 1) + col - 1;
	await invokes.rangeUpdate(props.player, row, col, weight / 100);
	store.ranges[props.player][idx] = weight;
	await onUpdate();
};

const onRangeTextChange = async () => {
	const trimmed = store.rangeText[props.player].replace(trimRegex, "$1").trim();
	const ranges = trimmed.split(",");

	if (ranges[ranges.length - 1] === "") {
		ranges.pop();
	}

	for (const range of ranges) {
		if (!rangeRegex.test(range)) {
			rangeTextError.value = `Failed to parse range: ${
				range || "(empty string)"
			}`;
			return;
		}
	}

	const errorString = await invokes.rangeFromString(props.player, trimmed);

	if (errorString) {
		rangeTextError.value = errorString;
	} else {
		const weights = await invokes.rangeGetWeights(props.player);
		for (let i = 0; i < 13 * 13; ++i) {
			store.ranges[props.player][i] = weights[i] * 100;
		}
		await onUpdate();
	}
};

const dragStart = (row: number, col: number) => {
	const idx = 13 * (row - 1) + col - 1;

	if (store.ranges[props.player][idx] !== weight.value) {
		draggingMode = "enabling";
		update(row, col, weight.value);
	} else {
		draggingMode = "disabling";
		update(row, col, 0);
	}
};

const dragEnd = () => {
	draggingMode = "none";
};

const mouseEnter = (row: number, col: number) => {
	if (draggingMode === "enabling") {
		update(row, col, weight.value);
	} else if (draggingMode === "disabling") {
		update(row, col, 0);
	}
};

const onWeightChange = () => {
	weight.value = Math.round(Math.max(0, Math.min(100, weight.value)));
};

const clearRange = async () => {
	await invokes.rangeClear(props.player);
	store.ranges[props.player].fill(0);
	store.rangeText[props.player] = "";
	rangeTextError.value = "";
	weight.value = 100;
	store.rangeCombos[props.player] = 0;
};

const invertRange = async () => {
	await invokes.rangeInvert(props.player);
	for (let i = 0; i < 13 * 13; ++i) {
		store.ranges[props.player][i] = 100 - store.ranges[props.player][i];
	}
	await onUpdate();
};

const loadRange = (rangeStr: unknown) => {
	store.rangeText[props.player] = String(rangeStr);
	onRangeTextChange();
};
</script>
