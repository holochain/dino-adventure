import type {Dino} from "../dino_adventure/dino_adventure/types";
import {callZome} from "./common.svelte";

export const getAllDinos = async (): Promise<Dino[]> => {
    return callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "get_all_dinos",
        cap_secret: null,
        payload: null,
    })
}
