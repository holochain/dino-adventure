import type {AuthoredDino, Dino, DinoKind} from "../dino_adventure/dino_adventure/types";
import {callZome} from "./common.svelte";

export const createDino = async (dino: Dino): Promise<AuthoredDino> => {
    return callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "create_dino",
        cap_secret: null,
        payload: dino,
    })
}

export const getAllDinos = async (): Promise<AuthoredDino[]> => {
    return callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "get_all_dinos",
        cap_secret: null,
        payload: null,
    })
}
