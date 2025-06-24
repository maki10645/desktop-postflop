<template>
  <div class="mt-2">
    <div class="gap-2">
      <label for="notearea">note:</label>
      <textarea v-model="note_str" id="notearea" rows="25" 
        class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border 
               focus:ring-blue-500 focus:border-blue-500
               [&::-webkit-scrollbar]:w-4
               [&::-webkit-scrollbar-track]:rounded-full
               [&::-webkit-scrollbar-track]:bg-gray-100
               [&::-webkit-scrollbar-thumb]:rounded-full
               [&::-webkit-scrollbar-thumb]:bg-gray-300
               dark:[&::-webkit-scrollbar-track]:bg-neutral-700
               dark:[&::-webkit-scrollbar-thumb]:bg-neutral-500
               resize-none
               "
               placeholder="Write your note here...">
      </textarea>
    </div>

    <div class="flex gap-2 align-baseline">
        <button class="button-base button-blue my-3" @click="SaveResult">Save</button>
        <button class="button-base button-blue my-3" @click="submitLoad">Load</button>
    </div>
    <div class="flex items-center">
      <span v-if="isSaving || isLoading" class="spinner inline-block mr-3"></span>
      <label class="mr-3">{{ status_str }}</label>
      <!-- <label class="mr-3">Status text here</label> -->
    </div>
  </div>
</template>





<script setup lang="ts">
import { ref } from "vue";
import * as invokes from "../invokes";
import {
	saveConfig,
	saveConfigTmp,
	useConfigStore,
	useSavedConfigStore,
	useStore,
	useTmpConfigStore,
} from "../store";

import { join, tempDir } from "@tauri-apps/api/path";
import * as dialog from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

const note_str = ref("");
const status_str = ref("");

const isSaving = ref(false);
const isLoading = ref(false);

const SaveResult = async () => {
	isSaving.value = true;
	status_str.value = "Saving...";

	const config_ = useConfigStore();
	const store_ = useStore();
	const saved_config_ = useSavedConfigStore();

	let store;
	let config;
	let saved_config;

	try {
		//status_str.value = "store_: " + Object.keys(store_) + "\r\nconfig_: " + Object.keys(config_) + "\r\nsaved_config_: " + Object.keys(saved_config_);
		//status_str.value = "store_: " + Object.keys(store_.$state)
		store = JSON.parse(JSON.stringify(store_.$state, undefined, 0));
		config = JSON.parse(JSON.stringify(config_.$state, undefined, 0));
		saved_config = JSON.parse(
			JSON.stringify(saved_config_.$state, undefined, 0),
		);
	} catch (e: any) {
		status_str.value = `Error: ${e.message}`;
		isSaving.value = false;
		return;
	}

	const SaveFrontendConfig = {
		note: note_str.value,
		store,
		config,
		saved_config,
	};

	const save_path = await dialog.save({
		defaultPath: "result.psr",
		filters: [
			// stand for PostSolver result
			{ name: "Psr Files", extensions: ["psr"] },
			{ name: "All Files", extensions: ["*"] },
		],
	});

	if (save_path) {
		try {
			const jsonstr = JSON.stringify(SaveFrontendConfig, undefined, 0);

			//status_str.value = "Debug: " + jsonstr;

			if (await invokes.savePostSolveResult(save_path as string, jsonstr)) {
				status_str.value = `Save complete. ${save_path}`;
				isSaving.value = false;
				return;
			}
			status_str.value = "Save failure.";
			isSaving.value = false;
			return;
		} catch (e: any) {
			//JSON.stringify fail
			status_str.value = `Error: ${e.message}`;
			isSaving.value = false;
			return;
		}
	} else {
		//dialogue cancelled
		status_str.value = "";
		isSaving.value = false;
		return;
	}
};

const submitLoad = async () => {
	const saved_config = useSavedConfigStore();
	const store = useStore();
	const config = useConfigStore();

	const load_path = await dialog.open({
		defaultPath: "result.psr",
		filters: [
			// Psr: PostSolver result
			{ name: "Psr Files", extensions: ["psr"] },
			{ name: "All Files", extensions: ["*"] },
		],
	});

	if (load_path === null) {
		// dialogue cancelled
		return;
	}

	if (Array.isArray(load_path)) {
		// multiple file were selected
		return;
	}

	isLoading.value = true;
	status_str.value = "Loading...";

	const loadresult = await invokes.loadPostSolveResult(load_path);

	if (!loadresult.status) {
		status_str.value = `Error: ${loadresult.config_str}`;
		isLoading.value = false;
		return;
	}
	try {
		const LoadFrontendConfig = JSON.parse(loadresult.config_str);

		//textarea note
		note_str.value = LoadFrontendConfig.note;

		//app, ip/oop and bunching fold ranges, bunching settings, solver-status
		for (const key in LoadFrontendConfig.store) {
			if (
				key.indexOf("navView") < 0 &&
				key.indexOf("sideView") < 0 &&
				key.indexOf("headers") < 0 &&
				key.indexOf("hasSolverRun") < 0
			) {
				(store as any)[key as keyof typeof store] =
					LoadFrontendConfig.store[key];
				//status_str.value = status_str.value + "           dbg: store " + key + " " +  (store as any)[key as keyof typeof store];
			}
		}

		//restore pinia's store
		//config, tree-config, action-tree
		for (const key in LoadFrontendConfig.config) {
			(config as any)[key as keyof typeof config] =
				LoadFrontendConfig.config[key];
		}

		//saved_config, refferd from result tab maybe?
		for (const key in LoadFrontendConfig.saved_config) {
			(saved_config as any)[key as keyof typeof saved_config] =
				LoadFrontendConfig.saved_config[key];
		}

		//restore range_state
		for (let player = 0; player < store.ranges.length; player++) {
			await invokes.rangeFromString(player, store.rangeText[player]);
		}

		status_str.value = "Load complete.";
		isLoading.value = false;
		return;
	} catch {
		//JSON.parse fail

		//saved_config
		const back_end_card_config = await invokes.loadCardConfig();
		saved_config.board = back_end_card_config.flop;
		const is_valid_card = (card: number) => {
			return card >= 0 && card <= 52;
		};
		if (is_valid_card(back_end_card_config.turn)) {
			saved_config.board.push(back_end_card_config.turn);
		}
		if (is_valid_card(back_end_card_config.river)) {
			saved_config.board.push(back_end_card_config.river);
		}
		saved_config.startingPot = back_end_card_config.starting_pot;
		saved_config.effectiveStack = back_end_card_config.effective_stack;

		//config
		const restore_frontend_config = await invokes.restoreFrontendConfig();
		config.board = back_end_card_config.flop;
		config.startingPot = restore_frontend_config.startingPot;
		config.effectiveStack = restore_frontend_config.effectiveStack;
		config.rakePercent = restore_frontend_config.rakePercent;
		config.rakeCap = restore_frontend_config.rakeCap;
		config.donkOption = restore_frontend_config.donkOption;
		config.oopFlopBet = restore_frontend_config.oopFlopBet;
		config.oopFlopRaise = restore_frontend_config.oopFlopRaise;
		config.oopTurnBet = restore_frontend_config.oopTurnBet;
		config.oopTurnRaise = restore_frontend_config.oopTurnRaise;
		config.oopTurnDonk = restore_frontend_config.oopTurnDonk;
		config.oopRiverBet = restore_frontend_config.oopRiverBet;
		config.oopRiverRaise = restore_frontend_config.oopRiverRaise;
		config.oopRiverDonk = restore_frontend_config.oopRiverDonk;
		config.ipFlopBet = restore_frontend_config.ipFlopBet;
		config.ipFlopRaise = restore_frontend_config.ipFlopRaise;
		config.ipTurnBet = restore_frontend_config.ipTurnBet;
		config.ipTurnRaise = restore_frontend_config.ipTurnRaise;
		config.ipRiverBet = restore_frontend_config.ipRiverBet;
		config.ipRiverRaise = restore_frontend_config.ipRiverRaise;
		config.addAllInThreshold = restore_frontend_config.addAllInThreshold;
		config.forceAllInThreshold = restore_frontend_config.forceAllInThreshold;
		config.mergingThreshold = restore_frontend_config.mergingThreshold;
		config.expectedBoardLength = restore_frontend_config.expectedBoardLength;
		config.addedLines = restore_frontend_config.addedLines;
		config.removedLines = restore_frontend_config.removedLines;

		store.isSolverFinished = true;

		status_str.value =
			"Load complete, but some frontend-config data is missing.";
		isLoading.value = false;
		return;
	}
};
</script>
