<template>
  <div class="flex shrink-0 h-12 border-y border-gray-500">
    <button
      v-for="mode in ['basics', 'graphs', 'compare'] as const"
      :key="mode"
      :class="
        'flex w-[9%] h-full items-center justify-center font-semibold text-lg transition ' +
        (chanceMode === ''
          ? displayMode === mode
            ? 'bg-blue-100 underline'
            : 'hover:bg-blue-100'
          : displayMode === mode
          ? 'bg-red-100 underline'
          : 'hover:bg-red-100')
      "
      @click="updateDisplayMode(mode)"
    >
      {{ capitalize(mode) }}
    </button>
    <button
      :class="
        'flex w-[9%] h-full items-center justify-center font-semibold text-lg transition enabled:hover:bg-blue-100 ' +
        (displayMode === 'chance' ? 'bg-blue-100 underline' : '')
      "
      :disabled="chanceMode === ''"
      @click="updateDisplayMode('chance')"
    >
      {{ capitalize(chanceMode) }}
    </button>

    <div
      class="flex ml-auto shrink-0 h-full px-4 items-center justify-start gap-2 snug"
    >
      <div
        v-if="['basics', 'graphs'].includes(displayMode)"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Player:</div>
        <select
          v-model="displayOptions.playerBasics"
          class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="auto">
            Auto ({{ autoPlayerBasics.toUpperCase() }})
          </option>
          <option value="oop">OOP</option>
          <option value="ip">IP</option>
        </select>
      </div>

      <div
        v-if="displayMode === 'chance'"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Player:</div>
        <select
          v-model="displayOptions.playerChance"
          class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="auto">
            Auto ({{ autoPlayerChance.toUpperCase() }})
          </option>
          <option value="oop">OOP</option>
          <option value="ip">IP</option>
        </select>
      </div>

      <div
        v-if="['basics', 'compare'].includes(displayMode)"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Bar Height:</div>
        <select
          v-model="displayOptions.barHeight"
          class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="normalized">Normalized</option>
          <option value="absolute">Absolute</option>
          <option value="full">Full</option>
        </select>
      </div>

      <div
        v-if="['basics', 'compare'].includes(displayMode)"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Suit:</div>
        <select
          v-model="displayOptions.suit"
          class="w-[6.25rem] px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="grouped">Grouped</option>
          <option value="individual">Individual</option>
        </select>
      </div>

      <div
        v-if="['basics', 'compare'].includes(displayMode)"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Display:</div>
        <select
          v-model="strategyContentPair"
          class="w-[8.75rem] px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="show,default">Strategy</option>
          <option value="show,eq">Strategy + EQ</option>
          <option value="show,ev">Strategy + EV</option>
          <option value="show,eqr">Strategy + EQR</option>
          <option value="none,default">Weight</option>
          <option value="none,eq">EQ</option>
          <option value="none,ev">EV</option>
          <option value="none,eqr">EQR</option>
        </select>
      </div>

      <div
        v-if="displayMode === 'graphs'"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Display:</div>
        <select
          v-model="displayOptions.contentGraphs"
          class="w-20 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="eq">EQ</option>
          <option value="ev">EV</option>
          <option value="eqr">EQR</option>
        </select>
      </div>

      <div
        v-if="displayMode === 'chance'"
        class="flex flex-col items-start justify-center h-full"
      >
        <div class="text-sm">Chart:</div>
        <select
          v-model="displayOptions.chartChance"
          class="w-[10.25rem] px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="strategy-combos">Strategy (Combos)</option>
          <option value="strategy">Strategy (%)</option>
          <option value="eq">Equity</option>
          <option value="ev">EV</option>
          <option value="eqr">EQR</option>
        </select>
      </div>

      <!--
      <div class="flex items-center px-4 gap-4">
        <Tippy content="Copy range text to clipboard">
          <button
            :class="
              'flex w-8 h-8 items-center justify-center rounded-lg shadow transition-colors border ' +
              (copySuccess === 1
                ? 'border-green-600 bg-green-100'
                : copySuccess === -1
                ? 'border-red-600 bg-red-100'
                : 'border-gray-600 bg-gray-200')
            "
            @click="copyToClipboard"
            @mouseleave="onCopyTooltipLeave"
          >
            <ClipboardDocumentIcon
              v-show="copySuccess !== 1"
              :class="'w-5 h-5 ' + (copySuccess === -1 ? 'text-red-600' : '')"
            />
            <ClipboardDocumentCheckIcon
              v-show="copySuccess === 1"
              class="w-5 h-5 text-green-600"
            />
          </button>
        </Tippy>

        <Tippy content="Open solver integration menu; not yet implemented">
          <button
            class="flex w-8 h-8 items-center justify-center border border-gray-600 bg-gray-200 rounded-lg shadow transition-colors active:bg-gray-300"
          >
            <ComputerDesktopIcon class="w-5 h-5" />
          </button>
        </Tippy>
      </div>
      -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, toRefs, watch } from "vue";
import * as Types from "../result-types";
import { capitalize } from "../utils";

// import { Tippy } from "vue-tippy";
// import { ComputerDesktopIcon } from "@heroicons/vue/24/solid";
// import {
//   ClipboardDocumentIcon,
//   ClipboardDocumentCheckIcon,
// } from "@heroicons/vue/24/outline";

const props = defineProps<{
	displayMode: Types.DisplayMode;
	chanceMode: string;
	autoPlayerBasics: "oop" | "ip";
	autoPlayerChance: "oop" | "ip";
	copySuccess: number;
}>();

const emit = defineEmits<{
	(event: "update:display-mode", displayMode: Types.DisplayMode): void;
	(event: "update:display-options", displayOptions: Types.DisplayOptions): void;
	// (event: "copy-to-clipboard"): void;
	// (event: "reset-copy-success"): void;
}>();

const { chanceMode } = toRefs(props);
let displayModeOld = "basics" as Types.DisplayMode;

watch(chanceMode, (newValue, oldValue) => {
	if (newValue && !oldValue) {
		displayModeOld = props.displayMode;
		emit("update:display-mode", "chance");
	} else if (!newValue && oldValue) {
		emit("update:display-mode", displayModeOld);
	}
});

const displayOptions = reactive<Types.DisplayOptions>({
	playerBasics: "auto",
	playerChance: "auto",
	barHeight: "normalized",
	suit: "grouped",
	strategy: "show",
	contentBasics: "default",
	contentGraphs: "eq",
	chartChance: "strategy-combos",
});

const strategyContentPair = ref("show,default");

const savedDisplayOptions = localStorage.getItem("display-options");

if (savedDisplayOptions) {
	const saved: Types.DisplayOptions = JSON.parse(savedDisplayOptions);

	if (Types.barHeightList.includes(saved?.barHeight)) {
		displayOptions.barHeight = saved.barHeight;
	}
	if (Types.suitList.includes(saved?.suit)) {
		displayOptions.suit = saved.suit;
	}
	if (Types.strategyList.includes(saved?.strategy)) {
		displayOptions.strategy = saved.strategy;
	}
	if (Types.contentBasicsList.includes(saved?.contentBasics)) {
		displayOptions.contentBasics = saved.contentBasics;
	}
	if (Types.contentGraphsList.includes(saved?.contentGraphs)) {
		displayOptions.contentGraphs = saved.contentGraphs;
	}
	if (Types.chartChanceList.includes(saved?.chartChance)) {
		displayOptions.chartChance = saved.chartChance;
	}

	strategyContentPair.value = [
		displayOptions.strategy,
		displayOptions.contentBasics,
	].join(",");

	emit("update:display-options", displayOptions);
}

const updateDisplayMode = (displayMode: Types.DisplayMode) => {
	if (displayMode !== "chance") {
		displayModeOld = displayMode;
	}
	emit("update:display-mode", displayMode);
};

const updateDisplayOptions = () => {
	const options = displayOptions;
	const [strategy, content] = strategyContentPair.value.split(",");
	options.strategy = strategy as Types.DisplayOptions["strategy"];
	options.contentBasics = content as Types.DisplayOptions["contentBasics"];
	localStorage.setItem("display-options", JSON.stringify(options));
	emit("update:display-options", options);
};

// const copyToClipboard = () => {
//   emit("copy-to-clipboard");
// };

// const onCopyTooltipLeave = () => {
//   emit("reset-copy-success");
// };
</script>
