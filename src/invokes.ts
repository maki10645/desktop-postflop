import { invoke } from "@tauri-apps/api/core";
import type { ChanceReports, Results } from "./result-types";

export const osName = async (): Promise<"windows" | "macos" | "linux"> => {
	return await invoke("os_name");
};

export const memory = async (): Promise<number[]> => {
	return await invoke("memory");
};

export const setNumThreads = async (numThreads: number) => {
	await invoke("set_num_threads", { numThreads });
};

/* Ranges */

export const rangeNumCombos = async (player: number): Promise<number> => {
	return await invoke("range_num_combos", { player });
};

export const rangeClear = async (player: number) => {
	await invoke("range_clear", { player });
};

export const rangeInvert = async (player: number) => {
	await invoke("range_invert", { player });
};

export const rangeUpdate = async (
	player: number,
	row: number,
	col: number,
	weight: number,
) => {
	await invoke("range_update", { player, row, col, weight });
};

export const rangeFromString = async (
	player: number,
	str: string,
): Promise<string | null> => {
	return await invoke("range_from_string", { player, str });
};

export const rangeToString = async (player: number): Promise<string> => {
	return await invoke("range_to_string", { player });
};

export const rangeGetWeights = async (player: number): Promise<number[]> => {
	return await invoke("range_get_weights", { player });
};

export const rangeRawData = async (player: number): Promise<number[]> => {
	return await invoke("range_raw_data", { player });
};

/* Action Tree */

export const treeNew = async (
	boardLen: number,
	startingPot: number,
	effectiveStack: number,
	donkOption: boolean,
	oopFlopBet: string,
	oopFlopRaise: string,
	oopTurnBet: string,
	oopTurnRaise: string,
	oopTurnDonk: string,
	oopRiverBet: string,
	oopRiverRaise: string,
	oopRiverDonk: string,
	ipFlopBet: string,
	ipFlopRaise: string,
	ipTurnBet: string,
	ipTurnRaise: string,
	ipRiverBet: string,
	ipRiverRaise: string,
	addAllinThreshold: number,
	forceAllinThreshold: number,
	mergingThreshold: number,
	addedLines: string,
	removedLines: string,
): Promise<boolean> => {
	return await invoke("tree_new", {
		boardLen,
		startingPot,
		effectiveStack,
		donkOption,
		oopFlopBet,
		oopFlopRaise,
		oopTurnBet,
		oopTurnRaise,
		oopTurnDonk,
		oopRiverBet,
		oopRiverRaise,
		oopRiverDonk,
		ipFlopBet,
		ipFlopRaise,
		ipTurnBet,
		ipTurnRaise,
		ipRiverBet,
		ipRiverRaise,
		addAllinThreshold,
		forceAllinThreshold,
		mergingThreshold,
		addedLines,
		removedLines,
	});
};

export const treeAddedLines = async (): Promise<string> => {
	return await invoke("tree_added_lines");
};

export const treeRemovedLines = async (): Promise<string> => {
	return await invoke("tree_removed_lines");
};

export const treeInvalidTerminals = async (): Promise<string> => {
	return await invoke("tree_invalid_terminals");
};

export const treeActions = async (): Promise<string[]> => {
	return await invoke("tree_actions");
};

export const treeIsTerminalNode = async (): Promise<boolean> => {
	return await invoke("tree_is_terminal_node");
};

export const treeIsChanceNode = async (): Promise<boolean> => {
	return await invoke("tree_is_chance_node");
};

export const treeBackToRoot = async () => {
	await invoke("tree_back_to_root");
};

export const treeApplyHistory = async (line: string[]) => {
	await invoke("tree_apply_history", { line });
};

export const treePlay = async (action: string): Promise<number> => {
	return await invoke("tree_play", { action });
};

export const treeTotalBetAmount = async (): Promise<number[]> => {
	return await invoke("tree_total_bet_amount");
};

export const treeAddBetAction = async (amount: number, isRaise: boolean) => {
	await invoke("tree_add_bet_action", { amount, isRaise });
};

export const treeRemoveCurrentNode = async () => {
	await invoke("tree_remove_current_node");
};

export const treeDeleteAddedLine = async (line: string) => {
	await invoke("tree_delete_added_line", { line });
};

export const treeDeleteRemovedLine = async (line: string) => {
	await invoke("tree_delete_removed_line", { line });
};

/* Bunching effect */

export const bunchingInit = async (board: number[]): Promise<string | null> => {
	return await invoke("bunching_init", { board });
};

export const bunchingClear = async () => {
	await invoke("bunching_clear");
};

export const bunchingProgress = async (): Promise<number[]> => {
	return await invoke("bunching_progress");
};

/* Game */

export const gameInit = async (
	board: number[],
	startingPot: number,
	effectiveStack: number,
	rakeRate: number,
	rakeCap: number,
	oopBubbleFactor: number,
	ipBubbleFactor: number,
	donkOption: boolean,
	oopFlopBet: string,
	oopFlopRaise: string,
	oopTurnBet: string,
	oopTurnRaise: string,
	oopTurnDonk: string,
	oopRiverBet: string,
	oopRiverRaise: string,
	oopRiverDonk: string,
	ipFlopBet: string,
	ipFlopRaise: string,
	ipTurnBet: string,
	ipTurnRaise: string,
	ipRiverBet: string,
	ipRiverRaise: string,
	addAllinThreshold: number,
	forceAllinThreshold: number,
	mergingThreshold: number,
	addedLines: string,
	removedLines: string,
): Promise<string | null> => {
	return await invoke("game_init", {
		board,
		startingPot,
		effectiveStack,
		rakeRate,
		rakeCap,
		oopBubbleFactor,
		ipBubbleFactor,
		donkOption,
		oopFlopBet,
		oopFlopRaise,
		oopTurnBet,
		oopTurnRaise,
		oopTurnDonk,
		oopRiverBet,
		oopRiverRaise,
		oopRiverDonk,
		ipFlopBet,
		ipFlopRaise,
		ipTurnBet,
		ipTurnRaise,
		ipRiverBet,
		ipRiverRaise,
		addAllinThreshold,
		forceAllinThreshold,
		mergingThreshold,
		addedLines,
		removedLines,
	});
};

export const gamePrivateCards = async (): Promise<number[][]> => {
	return await invoke("game_private_cards");
};

export const gameMemoryUsage = async (): Promise<number[]> => {
	return await invoke("game_memory_usage");
};

export const gameMemoryUsageBunching = async (): Promise<number> => {
	return await invoke("game_memory_usage_bunching");
};

export const gameAllocateMemory = async (enableCompression: boolean) => {
	await invoke("game_allocate_memory", { enableCompression });
};

export const gameSetBunching = async (): Promise<string | null> => {
	return await invoke("game_set_bunching");
};

export const gameSolveStep = async (currentIteration: number) => {
	await invoke("game_solve_step", { currentIteration });
};

export const gameExploitability = async (): Promise<number> => {
	return await invoke("game_exploitability");
};

export const gameFinalize = async () => {
	await invoke("game_finalize");
};

export const gameApplyHistory = async (history: number[]) => {
	await invoke("game_apply_history", { history });
};

export const gameTotalBetAmount = async (
	append: number[],
): Promise<number[]> => {
	return await invoke("game_total_bet_amount", { append });
};

export const gameActionsAfter = async (append: number[]): Promise<string[]> => {
	return await invoke("game_actions_after", { append });
};

export const gamePossibleCards = async (): Promise<bigint> => {
	return BigInt(await invoke("game_possible_cards"));
};

type ResultsResponse = {
	current_player: "oop" | "ip" | "chance" | "terminal";
	num_actions: number;
	is_empty: number;
	eqr_base: number[];
	weights: number[][];
	normalizer: number[][];
	equity: number[][];
	ev: number[][];
	eqr: number[][];
	strategy: number[];
	action_ev: number[];
};

export const gameGetResults = async (): Promise<Results> => {
	const results: ResultsResponse = await invoke("game_get_results");
	return {
		currentPlayer: results.current_player,
		numActions: results.num_actions,
		isEmpty: results.is_empty,
		eqrBase: results.eqr_base,
		weights: results.weights,
		normalizer: results.normalizer,
		equity: results.equity,
		ev: results.ev,
		eqr: results.eqr,
		strategy: results.strategy,
		actionEv: results.action_ev,
	};
};

type ChanceReportsResponse = {
	num_actions: number;
	status: number[];
	combos: number[][];
	equity: number[][];
	ev: number[][];
	eqr: number[][];
	strategy: number[];
};

export const gameGetChanceReports = async (
	append: number[],
	currentPlayer: "oop" | "ip" | "terminal",
	numActions: number,
): Promise<ChanceReports> => {
	const reports: ChanceReportsResponse = await invoke(
		"game_get_chance_reports",
		{ append, numActions },
	);
	return {
		currentPlayer,
		numActions,
		status: reports.status,
		combos: reports.combos,
		equity: reports.equity,
		ev: reports.ev,
		eqr: reports.eqr,
		strategy: reports.strategy,
	};
};

export const savePostSolveResult = async (
	path: string,
	frontendconfig: string,
): Promise<boolean> => {
	const success: boolean = await invoke("save_post_solver_result", {
		path: path,
		frontendconfig: frontendconfig,
	});
	return success;
};

export const savePostSolveResult2 = async (
	path: string,
	frontcfgpath: string,
): Promise<boolean> => {
	const success: boolean = await invoke("save_post_solver_result2", {
		path,
		frontcfgpath,
	});
	return success;
};

export const DeleteDesigPath = async (path: string): Promise<boolean> => {
	const success: boolean = await invoke("delete_desig_path", {
		path,
	});
	return success;
};

export const exportWeights = async (
	player: number,
	filePath: string,
): Promise<void> => {
	await invoke("export_weights", {
		player,
		filePath,
	});
};

export const loadPostSolveResult = async (
	path: string,
): Promise<loadPostSolveResultReturn> => {
	const ret: loadPostSolveResultReturn = await invoke(
		"load_post_solver_result",
		{ path: path },
	);
	return ret;
};
type loadPostSolveResultReturn = {
	status: boolean;
	config_str: string;
};

type CardConfig = {
	range: number[][];
	flop: number[];
	turn: number;
	river: number;
	starting_pot: number;
	effective_stack: number;
};

export const loadCardConfig = async (): Promise<CardConfig> => {
	const back_end_card_config: CardConfig = await invoke("load_card_config");

	return back_end_card_config;
};

//save & restore frontend configuration
type RestoreFrontendConfig = {
	startingPot: number;
	effectiveStack: number;
	rakePercent: number;
	rakeCap: number;
	donkOption: boolean;
	oopFlopBet: string;
	oopFlopRaise: string;
	oopTurnBet: string;
	oopTurnRaise: string;
	oopTurnDonk: string;
	oopRiverBet: string;
	oopRiverRaise: string;
	oopRiverDonk: string;
	ipFlopBet: string;
	ipFlopRaise: string;
	ipTurnBet: string;
	ipTurnRaise: string;
	ipRiverBet: string;
	ipRiverRaise: string;
	addAllInThreshold: number;
	forceAllInThreshold: number;
	mergingThreshold: number;
	expectedBoardLength: number;
	addedLines: string;
	removedLines: string;
};
export const restoreFrontendConfig =
	async (): Promise<RestoreFrontendConfig> => {
		const ret: RestoreFrontendConfig = await invoke("load_frontend_config");
		return ret;
	};

export const saveUint8ArrayToDesigPath = async (
	path: string,
	chunk: Uint8Array,
): Promise<boolean> => {
	const success: boolean = await invoke("save_uint8array_to_desig_path", {
		path,
		chunk,
	});
	return success;
};

export const testtest = async (
	test1: string,
	test2: string,
): Promise<boolean> => {
	const success: boolean = await invoke("test_test", {
		test1,
		test2,
	});
	return success;
};

export const testtest2 = async (
	test1: string,
	test2: Uint8Array,
): Promise<boolean> => {
	const success: boolean = await invoke("test_test2", {
		test1,
		test2,
	});
	return success;
};
